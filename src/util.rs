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

//! Some basic utilities here.

use gl;

/// Checks if an OpenGL error has happened, and panics if so. Not really useful in release mode, as
/// it can be quite slow, and there's relatively little to do anyway if an error happens.
pub fn check_error(file: &str, line: u32) {
    let err_code = unsafe { gl::GetError() };
    if err_code != 0 {
        let message = match err_code {
            gl::INVALID_ENUM => "GL_INVALID_ENUM",
            gl::INVALID_VALUE => "GL_INVALID_VALUE",
            gl::INVALID_OPERATION => "GL_INVALID_OPERATION",
            gl::INVALID_FRAMEBUFFER_OPERATION => "GL_INVALID_FRAMEBUFFER_OPERATION",
            gl::OUT_OF_MEMORY => "GL_OUT_OF_MEMORY",
            // gl::STACK_UNDERFLOW => "GL_STACK_UNDERFLOW",
            // gl::STACK_OVERFLOW => "GL_STACK_OVERFLOW",
            _ => "Unrecognized error code"
        };
        panic!("OpenGL Error: {} ({}) at {}:{}", message, err_code, file, line);
    }
}

/// Takes a Vec<u8>, returns a String. Conversion may be lossy.
/// Always remember to shorten the vector to exclude the null byte before passing the Vec to this fn!
pub fn vec_to_string(vec: Vec<u8>) -> String {
    match String::from_utf8(vec) {
        Ok(string) => string,
        Err(err) => slice_to_string(&err.into_bytes()[..])
    }
}

/// Takes a &[u8], returns a String. Conversion may be lossy.
/// Leave no null bytes to the end of the string!
pub fn slice_to_string(slice: &[u8]) -> String {
    String::from_utf8_lossy(slice).into_owned()
}