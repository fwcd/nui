pub use nui_shared::*;

/// Blocks and presents the given view to the user.
pub fn run_app(view: impl View) {
    let root = NUIRoot::new(view);
    let ffi_view = CNUIRoot::from(Box::new(root));

    unsafe {
        #[cfg(target_os = "macos")]
        {
            nui_swiftui_bridge::run_app(&ffi_view);
        }
    }
}
