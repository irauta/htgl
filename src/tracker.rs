
use super::Bind;

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