use std::{rc::Rc, cell::RefCell};

use crate::{Storage, Primitive, View, Context};

/// The central state of a NUI application.
pub struct NUIRoot<T> {
    view: T,
    storage: Rc<RefCell<Storage>>,
}

impl<T> NUIRoot<T> {
    pub fn new(view: T) -> Self {
        Self {
            view,
            storage: Rc::new(RefCell::new(Storage::new())),
        }
    }
}

impl<T> NUIRoot<T> where T: View {
    pub fn render(&mut self) -> Primitive {
        self.view.render(&Context::new(self.storage.clone()))
    }
}
