// Copyright 2015 Ilkka Rauta
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Tracker types are meant to help "client side" state tracking. (Quotes because the client side
//! mentioned here is not the same as OpenGL client side, but rather this library vs. the actual
//! OpenGL implementation) Tracking is used to take care that same resource is not bound with a
//! glBind* call several times in succession - this is redundant, so causes by definition
//! unnecessary calls to OpenGL. That this provides actual performance benefits in real-life
//! situations, is not actually known yet.

use std::rc::Rc;

use std::marker::PhantomData;

/// Helper types that bind resources implement Bind. The types don't bind themselves directly when
/// using trackers, because additional parameters may be needed, and the "binder" objects provide
/// those. The binder type may naturally just call a method of the object that is being bound.
pub trait Bind<R> {
    /// Do the actual binding, that is, call glBind* for the resource.
    fn bind(&self, resource: &R);
    /// Return (process-locally) unique identifier of the resource.
    fn get_id(&self, resource: &R) -> TrackerId;
}

/// As the name says, a simple binding tracker. Knows what is currently bound to the context.
pub struct SimpleBindingTracker<T: Bind<R>, R> {
    currently_bound: TrackerId,
    binder: T,
    /// The type uses generics to keep the tracker type-specific, but PhantomData is needed because
    /// there's no member of the type (or a borrow) in the struct.
    marker: PhantomData<R>
}

impl<T: Bind<R>, R> SimpleBindingTracker<T, R> {
    /// Construct a new `SimpleBindingTracker`.
    pub fn new(binder: T) -> SimpleBindingTracker<T, R> {
        SimpleBindingTracker {
            currently_bound: TrackerId { id: 0 },
            binder: binder,
            marker: PhantomData
        }
    }

    /// Bind resource or do nothing if it was already bound.
    pub fn bind(&mut self, resource: &R) {
        let id = self.binder.get_id(resource);
        if self.currently_bound != id {
            self.binder.bind(resource);
            self.currently_bound = id;
        }
    }
}

/// A tracker type that knows what's currently bound, but also remembers what was bound for
/// rendering. It can return the bound-for-drawing resource to actually bound state even if another
/// resource was temporarily bound for editing.
pub struct RenderBindingTracker<T: Bind<R>, R> {
    simple_tracker: SimpleBindingTracker<T, R>,
    // TODO: This could be Weak instead of Rc when it gets stable to allow resources to die ASAP.
    bound_for_rendering: Option<Rc<R>>
}

impl<T: Bind<R>, R> RenderBindingTracker<T, R> {
    /// Construct a new tracker.
    pub fn new(binder: T) -> RenderBindingTracker<T, R> {
        RenderBindingTracker { simple_tracker: SimpleBindingTracker::new(binder), bound_for_rendering: None }
    }

    /// Bind resource for editing - resource is bound immediately if not already bound.
    pub fn bind_for_editing(&mut self, resource: &R) {
        self.simple_tracker.bind(resource);
    }

    /// Bind resource for drawing - resource is bound immediately if not already bound, but also
    /// marked as being used for rendering. If another resource is bound for editing, this binding
    /// may still be restored by `restore_rendering_state()`.
    pub fn bind_for_rendering(&mut self, resource: &Rc<R>) {
        self.simple_tracker.bind(&**resource);
        self.bound_for_rendering = Some(resource.clone());
    }

    /// If a resource has been bound for rendering earlier, bind it again (if not bound already),
    /// even if another resource had been bound for editing.
    pub fn restore_rendering_state(&mut self) {
        match self.bound_for_rendering {
            Some(ref resource) => self.simple_tracker.bind(&*resource),
            None => return
        }
    }
}

/// A identifier type used by the tracker types.
#[derive(Clone,Copy)]
pub struct TrackerId {
    id: u32
}

impl PartialEq for TrackerId {
    fn eq(&self, other: &TrackerId) -> bool {
        self.id == other.id
    }
}

/// Tracker id generator always returns new identifiers (within reason, the value is internally a
/// regular integer). This is better than the way OpenGL itself works, as it may reuse identifiers,
/// causing problems with binding trackers that might think a new resource is already bound, when
/// the value was actually used by already-deleted resource.
pub struct TrackerIdGenerator {
    counter: u32
}

impl TrackerIdGenerator {
    /// Construct a new tracker.
    pub fn new() -> TrackerIdGenerator {
        TrackerIdGenerator { counter: 0 }
    }

    /// Construct a new resource identifier.
    pub fn new_id(&mut self) -> TrackerId {
        self.counter += 1;
        TrackerId { id: self.counter }
    }
}