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

#![feature(macro_reexport)]

extern crate serde;
extern crate serde_json;

#[macro_reexport(fragment)]
pub extern crate rsx_primitives;

#[cfg(feature = "native-embedding")]
pub extern crate rsx_native_renderer;

#[cfg(feature = "native-embedding")]
#[path = "target-native/macro.rs"]
mod native_macro;
#[cfg(feature = "native-embedding")]
#[path = "target-native/runtime.rs"]
mod native_runtime;
#[cfg(feature = "native-embedding")]
#[path = "util_webrender.rs"]
mod native_util;

#[macro_use]
#[cfg(any(feature = "web-embedding", feature = "web-standalone-json-embedding", feature = "web-standalone-bincode-embedding"))]
#[path = "target-web/macro.rs"]
mod web_macro;
#[cfg(any(feature = "web-embedding", feature = "web-standalone-json-embedding", feature = "web-standalone-bincode-embedding"))]
#[path = "target-web/runtime.rs"]
mod web_runtime;
#[cfg(any(feature = "web-embedding", feature = "web-standalone-json-embedding", feature = "web-standalone-bincode-embedding"))]
#[path = "target-web/standalone_api.rs"]
mod web_standalone_api;
#[cfg(any(feature = "web-embedding", feature = "web-standalone-json-embedding", feature = "web-standalone-bincode-embedding"))]
#[path = "target-web/debug_api.rs"]
mod web_debug_api;
#[cfg(any(feature = "web-embedding", feature = "web-standalone-json-embedding", feature = "web-standalone-bincode-embedding"))]
#[path = "target-web/resource_api.rs"]
mod web_resource_api;
#[cfg(any(feature = "web-embedding", feature = "web-standalone-json-embedding", feature = "web-standalone-bincode-embedding"))]
#[path = "target-web/dom_api.rs"]
mod web_dom_api;
#[cfg(any(feature = "web-embedding", feature = "web-standalone-json-embedding", feature = "web-standalone-bincode-embedding"))]
#[path = "target-web/stylesheet_api.rs"]
mod web_stylesheet_api;
#[cfg(any(feature = "web-embedding", feature = "web-standalone-json-embedding", feature = "web-standalone-bincode-embedding"))]
#[path = "target-web/events_api.rs"]
mod web_events_api;
#[cfg(any(feature = "web-embedding", feature = "web-standalone-json-embedding", feature = "web-standalone-bincode-embedding"))]
#[path = "util_json.rs"]
mod web_util;
#[cfg(any(feature = "web-embedding", feature = "web-standalone-json-embedding", feature = "web-standalone-bincode-embedding"))]
#[path = "target-web/helpers.rs"]
mod helpers;

#[cfg(feature = "ios-embedding")]
#[path = "target-ios/macro.rs"]
mod ios_macro;
#[cfg(feature = "ios-embedding")]
#[path = "util_json.rs"]
mod ios_util;

pub mod types {
    #[cfg(feature = "native-embedding")]
    pub use native_runtime::*;
    #[cfg(any(feature = "web-embedding", feature = "web-standalone-json-embedding", feature = "web-standalone-bincode-embedding"))]
    pub use web_runtime::*;
}

pub mod util {
    #[cfg(feature = "ios-embedding")]
    pub use ios_util::*;
    #[cfg(feature = "native-embedding")]
    pub use native_util::*;
    #[cfg(any(feature = "web-embedding", feature = "web-standalone-json-embedding", feature = "web-standalone-bincode-embedding"))]
    pub use web_util::*;
}

pub use rsx_primitives::rsx_dom;
pub use rsx_primitives::rsx_event_manager;
pub use rsx_primitives::rsx_layout;
pub use rsx_primitives::rsx_resources;
pub use rsx_primitives::rsx_shared;
pub use rsx_primitives::rsx_stylesheet;

#[macro_export]
macro_rules! empty_setup {
    () => {
        |_| ()
    }
}

#[macro_export]
macro_rules! empty_render {
    () => {
        || Default::default()
    }
}
