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

#[macro_export]
macro_rules! link {
    ( $setup:expr, $render:expr ) => {
        fn main() {
            use rsx_embedding::rsx_shared::traits::{TRunner, TRuntime};
            use rsx_embedding::rsx_native_renderer::types::Runner;
            use rsx_embedding::types::Runtime;
            Runner::run(|api| Runtime::new(api, $setup, $render));
        }
    }
}
