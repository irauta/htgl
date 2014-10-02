
use super::Handle;
use super::Bind;

pub struct SimpleBindingTracker<T> {
    currently_bound: u32,
    bound_for_drawing: Option<Handle<T>>
}

impl<T: Bind> SimpleBindingTracker<T> {
    pub fn new() -> SimpleBindingTracker<T> {
        SimpleBindingTracker { currently_bound: 0, bound_for_drawing: None }
    }

    pub fn bind_for_editing(&mut self, resource: &T) {
        let id = resource.get_id();
        if self.currently_bound != id {
            resource.bind();
            self.currently_bound = id;
        }
    }

    pub fn bind_for_drawing(&mut self, resource: &Handle<T>) {
        let resource_id = resource.access().get_id();
        if !self.is_bound_for_drawing(resource_id) {
            self.bound_for_drawing = Some(resource.clone());
        }
    }

    pub fn prepare_for_drawing(&mut self) {
        match self.bound_for_drawing {
            Some(ref bound_for_drawing) => {
                let resource = bound_for_drawing.access();
                let id = resource.get_id();
                if self.currently_bound != id {
                    resource.bind();
                    self.currently_bound = id;
                }
            },
            None => {}
        }
    }

    pub fn unregister(&mut self, id: u32) {
        if self.currently_bound == id {
            self.currently_bound = 0;
        }
        if self.is_bound_for_drawing(id) {
            self.bound_for_drawing = None;
        }
    }

    fn is_bound_for_drawing(&self, id: u32) -> bool {
        match self.bound_for_drawing {
            Some(ref bound_for_drawing) => bound_for_drawing.access().get_id() == id,
            None => false
        }
    }
}