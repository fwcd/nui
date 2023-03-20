use std::ffi::{c_void, c_char, CString, CStr};

use crate::{NUIRoot, View, IdPath};

/// A C/FFI-compatible wrapper around `NUIRoot<T>`.
#[repr(C)]
pub struct CNUIRoot {
    /// The opaque pointer to the owned underlying Rust `NUIRoot<T>`.
    wrapped: *mut c_void,
    /// Renders the view to an owned JSON-serialized primitive tree.
    /// **Callers are responsible for calling `nui_drop_string` on this string!**
    render_json: extern fn(*const CNUIRoot) -> *const c_char,
    /// Fires a click action to the given JSON-serialized id path.
    fire_click_action: extern fn(*const CNUIRoot, *const c_char),
}

extern "C" fn render_json_impl<T>(c_root: *const CNUIRoot) -> *const c_char where T: View {
    unsafe {
        let root = (*c_root).wrapped as *mut NUIRoot<T>;
        let primitive = (*root).render();
        let json = serde_json::to_string(&primitive).expect("Could not serialize view");
        let c_string = CString::new(json).expect("Could not convert JSON to C string");
        c_string.into_raw()
    }
}

extern "C" fn fire_click_action_impl<T>(c_root: *const CNUIRoot, raw_json: *const c_char) where T: View {
    unsafe {
        let root = (*c_root).wrapped as *mut NUIRoot<T>;
        let c_str = CStr::from_ptr(raw_json);
        let id_path: IdPath = serde_json::from_slice(c_str.to_bytes()).expect("Could not deserialize id path");
        let storage = (*root).storage();
        storage.fire_click_action(&id_path);
    }
}

impl<T> From<Box<NUIRoot<T>>> for CNUIRoot where T: View {
    fn from(value: Box<NUIRoot<T>>) -> Self {
        Self {
            wrapped: Box::into_raw(value) as *mut c_void,
            render_json: render_json_impl::<T>,
            fire_click_action: fire_click_action_impl::<T>,
        }
    }
}

impl Drop for CNUIRoot {
    fn drop(&mut self) {
        unsafe {
            drop(Box::from_raw(self.wrapped as *mut c_void));
        }
    }
}