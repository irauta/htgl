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

//! This module defines a handle structure, that is the user-facing representation of GL resources.

use std::rc::Rc;

/// `Handle` is essentially a wrapped `std::rc::Rc`, though you should not rely it always being that
/// way. `Handle` does not implement `Deref`, as the contents are not meant to be used by user
/// directly.
pub struct Handle<T> {
    resource: Rc<T>
}

/// Constructs a new handle given ownership of a resource. Not a method of Handle to keep
/// visibility limited.
pub fn new_handle<T>(resource: T) -> Handle<T> {
    Handle { resource: Rc::new(resource) }
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Handle<T> {
        Handle { resource: self.resource.clone() }
    }
}

/// A "workaround" for the missing `Deref´ implementation: an nonpublic trait to access handle's
/// target contents.
pub trait HandleAccess<T> {
    fn access(&self) -> &T;
    fn rc(&self) -> &Rc<T>;
}

impl<T> HandleAccess<T> for Handle<T> {
    /// Acts as "deref"
    fn access(&self) -> &T {
        &*self.resource
    }

    fn rc(&self) -> &Rc<T> {
        &self.resource
    }
}