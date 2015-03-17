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

use gl;
use gl::types::{GLenum,GLsizeiptr,GLvoid};

use std::mem::size_of;

use std::marker::PhantomData;

use super::tracker::Bind;
use super::context::RegistrationHandle;
use super::tracker::TrackerId;

pub use self::vertexbuffer::VertexBuffer;
pub use self::indexbuffer::IndexBuffer;
pub use self::uniformbuffer::UniformBuffer;

pub mod vertexbuffer;
pub mod indexbuffer;
pub mod uniformbuffer;

pub struct BufferObject<T> {
    pub id: u32,
    tracker_id: TrackerId,
    registration: RegistrationHandle,
    target: GLenum,
    marker: PhantomData<T>
}

impl<T> BufferObject<T> {
    fn new(tracker_id: TrackerId, target: GLenum, registration: RegistrationHandle) -> BufferObject<T> {
        let mut id: u32 = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
            check_error!();
        }
        BufferObject {
            id: id,
            tracker_id: tracker_id,
            registration: registration,
            target: target,
            marker: PhantomData
        }
    }

    pub fn data<D>(&self, data: &[D]) {
        let data_size = (size_of::<D>() * data.len()) as GLsizeiptr;
        unsafe {
            gl::BufferData(self.target, data_size, data.as_ptr() as *const GLvoid, gl::STATIC_DRAW);
            check_error!();
        }
    }

    pub fn sub_data<D>(&self, data: &[D], byte_offset: usize) {
        let data_size = (size_of::<D>() * data.len()) as GLsizeiptr;
        unsafe {
            gl::BufferSubData(self.target, data_size, byte_offset as GLsizeiptr, data.as_ptr() as *const GLvoid);
            check_error!();
        }
    }
}

#[unsafe_destructor]
impl<T> Drop for BufferObject<T> {
    fn drop(&mut self) {
        if self.registration.context_alive() {
            unsafe {
                gl::DeleteBuffers(1, &self.id);
                check_error!();
            }
        }
    }
}

impl<T> PartialEq for BufferObject<T> {
    fn eq(&self, other: &BufferObject<T>) -> bool {
        self.id == other.id
    }
}

impl<T> Bind for BufferObject<T> {
    fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.target, self.id);
        }
    }

    fn get_id(&self) -> TrackerId {
        self.tracker_id
    }
}
