
use std::rc::{Rc,Weak};

pub trait Bind {
    fn bind(&self);
    fn get_id(&self) -> TrackerId;
}

pub struct SimpleBindingTracker<T> {
    currently_bound: TrackerId
}

impl<T: Bind> SimpleBindingTracker<T> {
    pub fn new() -> SimpleBindingTracker<T> {
        SimpleBindingTracker { currently_bound: TrackerId { id: 0 } }
    }

    pub fn bind(&mut self, resource: &T) {
        let id = resource.get_id();
        if self.currently_bound != id {
            resource.bind();
            self.currently_bound = id;
        }
    }
}

pub struct RenderBindingTracker<T> {
    simple_tracker: SimpleBindingTracker<T>,
    bound_for_rendering: Option<Weak<T>>
}

impl<T: Bind> RenderBindingTracker<T> {
    pub fn new() -> RenderBindingTracker<T> {
        RenderBindingTracker { simple_tracker: SimpleBindingTracker::new(), bound_for_rendering: None }
    }

    pub fn bind_for_editing(&mut self, resource: &T) {
        self.simple_tracker.bind(resource);
    }

    pub fn bind_for_rendering(&mut self, resource: &Rc<T>) {
        self.simple_tracker.bind(&**resource);
        self.bound_for_rendering = Some(resource.downgrade());
    }

    pub fn restore_rendering_state(&mut self) {
        let upgraded = match self.bound_for_rendering {
            Some(ref weak) => weak.upgrade(),
            None => return
        };
        match upgraded {
            Some(resource) => self.simple_tracker.bind(&*resource),
            None => self.bound_for_rendering = None
        }
    }
}

pub struct TrackerId {
    id: u32
}

impl PartialEq for TrackerId {
    fn eq(&self, other: &TrackerId) -> bool {
        self.id == other.id
    }
}

pub struct TrackerIdGenerator {
    counter: u32
}

impl TrackerIdGenerator {
    pub fn new() -> TrackerIdGenerator {
        TrackerIdGenerator { counter: 0 }
    }

    pub fn new_id(&mut self) -> TrackerId {
        self.counter += 1;
        TrackerId { id: self.counter }
    }
}