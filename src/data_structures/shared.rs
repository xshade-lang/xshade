use ::std::rc::Rc;
use ::std::cell::{ RefCell, Ref, RefMut };

#[derive(Debug)]
pub struct Shared<T> {
    inner: Rc<RefCell<T>>,
}

impl<T> Shared<T> {
    pub fn new(inner: T) -> Shared<T> {
        Shared {
            inner: Rc::new(RefCell::new(inner)),
        }
    }

    pub fn clone(&self) -> Shared<T> {
        Shared {
            inner: Rc::clone(&self.inner),
        }
    }

    pub fn borrow(&self) -> Ref<T> {
        self.inner.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<T> {
        self.inner.borrow_mut()
    }
}
