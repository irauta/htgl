
use super::Bind;

pub struct SimpleBindingTracker<T> {
    currently_bound: u32
}

impl<T: Bind> SimpleBindingTracker<T> {
    pub fn new() -> SimpleBindingTracker<T> {
        SimpleBindingTracker { currently_bound: 0 }
    }

    pub fn bind(&mut self, resource: &T) {
        let id = resource.get_id();
        if self.currently_bound != id {
            resource.bind();
            self.currently_bound = id;
        }
    }

    pub fn unregister(&mut self, id: u32) {
        if self.currently_bound == id {
            self.currently_bound = 0;
        }
    }
}
