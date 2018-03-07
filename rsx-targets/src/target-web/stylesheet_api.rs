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

use std::os::raw::c_char;

use serde_json;

use helpers::string_from;
use types::Runtime;

impl Runtime {
    pub fn register_style(&mut self, style_declarations: *mut c_char) -> Option<usize> {
        let style_declarations = serde_json::from_str(&string_from(style_declarations)).ok()?;

        let id = self.styles.len();
        self.styles.push(style_declarations);

        Some(id)
    }
}
