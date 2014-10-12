
use std::rc::Rc;

pub struct Handle<T> {
    resource: Rc<T>
}

pub fn new_handle<T>(resource: T) -> Handle<T> {
    Handle { resource: Rc::new(resource) }
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Handle<T> {
        Handle { resource: self.resource.clone() }
    }
}

pub trait HandleAccess<T> {
    fn access(&self) -> &T;
    fn rc(&self) -> &Rc<T>;
}

impl<T> HandleAccess<T> for Handle<T> {
    fn access(&self) -> &T {
        &*self.resource
    }

    fn rc(&self) -> &Rc<T> {
        &self.resource
    }
}