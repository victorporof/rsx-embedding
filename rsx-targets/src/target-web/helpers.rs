/*
Copyright 2016 Mozilla
Licensed under the Apache License, Version 2.0 (the "License"); you may not use
this file except in compliance with the License. You may obtain a copy of the
License at http://www.apache.org/licenses/LICENSE-2.0
Unless required by applicable law or agreed to in writing, software distributed
under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
CONDITIONS OF ANY KIND, either express or implied. See the License for the
specific language governing permissions and limitations under the License.
*/

use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::{c_char, c_uchar};

pub fn string_from(raw: *mut c_char) -> String {
    // Strings come from JavaScript. They're guaranteed to be UTF-8, and
    // emscripten properly creates a c-style null-terminated string with cwrap.
    let bytes = unsafe { CStr::from_ptr(raw) }.to_bytes().to_owned();
    unsafe { String::from_utf8_unchecked(bytes) }
}

pub fn ptr_and_len_string_from(raw: *mut c_uchar, len: usize) -> *mut c_char {
    let info = format!("[{}, {}]", raw as usize, len);
    unsafe { CString::from_vec_unchecked(info.into_bytes()) }.into_raw()
}
