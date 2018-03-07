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
        #[link_args = "-s MODULARIZE=1"]
        #[link_args = "-s USE_FREETYPE=1"]
        #[link_args = "-s EXPORTED_FUNCTIONS=['_initialize_runtime','_get_display_list','_get_display_list_diff','_get_resource_updates','_gc','_print_diag','_load_font','_load_image','_create_orphan_normal_node','_create_orphan_inline_text_node','_append_child','_remove_child','_append_child_to_container','_remove_child_from_container','_set_text_content','_set_styles','_add_event_listener','_remove_event_listener','_register_style','_unregister_style','_receive_key_event','_receive_mouse_event','_poll_events']"]
        #[link_args = "-s EXTRA_EXPORTED_RUNTIME_METHODS=['ccall','cwrap']"]
        #[link_args = "-s TOTAL_MEMORY=67108864"]
        #[link_args = "--llvm-opts 3"]
        #[link_args = "--llvm-lto 3"]
        #[link_args = "-s NO_FILESYSTEM=1"]
        #[link_args = "-s ASSERTIONS=0"]
        #[link_args = "-s DISABLE_EXCEPTION_CATCHING=1"]
        #[link_args = "-s ELIMINATE_DUPLICATE_FUNCTIONS=1"]
        #[link_args = "-s AGGRESSIVE_VARIABLE_ELIMINATION=1"]
        extern "C" {}

        fn main() {}

        // Standard API

        #[no_mangle]
        pub fn initialize_runtime() {
            rsx_embedding::types::Runtime::init($setup, $render);
        }

        #[no_mangle]
        pub fn get_display_list(width: usize, height: usize) -> *mut i8 {
            rsx_embedding::types::Runtime::static_generate_display_list_ptr((width as u32, height as u32))
        }

        #[no_mangle]
        pub fn get_display_list_diff(width: usize, height: usize) -> *mut i8 {
            rsx_embedding::types::Runtime::static_generate_display_list_diff_ptr((width as u32, height as u32))
        }

        #[no_mangle]
        pub fn get_resource_updates() -> *mut i8 {
            rsx_embedding::types::Runtime::static_take_resource_updates_ptr()
        }

        #[no_mangle]
        pub fn gc() {
            rsx_embedding::types::Runtime::static_gc()
        }

        // Debug API

        #[no_mangle]
        pub fn print_diag() {
            let success = rsx_embedding::types::Runtime::static_print_diag();
            success.expect("Panic: Couldn't print diagnostics")
        }

        // Resource API

        #[no_mangle]
        pub fn load_font(font_name: *mut i8, data_uri: *mut i8, face_index: usize) {
            let success = rsx_embedding::types::Runtime::static_load_font(font_name, data_uri, face_index);
            success.expect("Panic: Couldn't load font")
        }

        #[no_mangle]
        pub fn load_image(virtual_src: *mut i8, data_uri: *mut i8) {
            let success = rsx_embedding::types::Runtime::static_load_image(virtual_src, data_uri);
            success.expect("Panic: Couldn't load image")
        }

        // DOM API

        #[no_mangle]
        pub fn create_orphan_normal_node(element_name_id: usize) -> usize {
            let success = rsx_embedding::types::Runtime::static_create_orphan_normal_node(element_name_id as u16);
            success.expect("Panic: Couldn't create orphan node") as usize
        }

        #[no_mangle]
        pub fn create_orphan_inline_text_node(text_content: *mut i8) -> usize {
            let success = rsx_embedding::types::Runtime::static_create_orphan_inline_text_node(text_content);
            success.expect("Panic: Couldn't create orphan node") as usize
        }

        #[no_mangle]
        pub fn append_child(parent_id: usize, child_id: usize)  {
            let success = rsx_embedding::types::Runtime::static_append_child(parent_id as u64, child_id as u64);
            success.expect("Panic: Couldn't append child")
        }

        #[no_mangle]
        pub fn remove_child(child_id: usize)  {
            unimplemented!()
        }

        #[no_mangle]
        pub fn append_child_to_container(child_id: usize) {
            let success = rsx_embedding::types::Runtime::static_append_child_to_container(child_id as u64);
            success.expect("Panic: Couldn't append child to container")
        }

        #[no_mangle]
        pub fn remove_child_from_container(child_id: usize)  {
            unimplemented!()
        }

        #[no_mangle]
        pub fn set_text_content(node_id: usize, text_content: *mut i8) {
            let success = rsx_embedding::types::Runtime::static_set_text_content(node_id as u64, text_content);
            success.expect("Panic: Couldn't set text content")
        }

        #[no_mangle]
        pub fn set_styles(node_id: usize, style_ids: *mut u8, len: usize) {
            let success = rsx_embedding::types::Runtime::static_set_styles(node_id as u64, style_ids, len);
            success.expect("Panic: Couldn't set styles")
        }

        #[no_mangle]
        pub fn add_event_listener(node_id: usize, event_type_id: usize) {
            let success = rsx_embedding::types::Runtime::static_add_event_listener(node_id as u64, event_type_id as u8);
            success.expect("Panic: Couldn't add event listener")
        }

        #[no_mangle]
        pub fn remove_event_listener(node_id: usize, event_type_id: usize) {
            let success = rsx_embedding::types::Runtime::static_remove_event_listener(node_id as u64, event_type_id as u8);
            success.expect("Panic: Couldn't remove event listener")
        }

        // Stylesheet API

        #[no_mangle]
        pub fn register_style(style_declarations: *mut i8) -> usize {
            let success = rsx_embedding::types::Runtime::static_register_style(style_declarations);
            success.expect("Panic: Couldn't register style")
        }

        #[no_mangle]
        pub fn unregister_style(style_id: usize) {
            unimplemented!()
        }

        // Events API

        #[no_mangle]
        pub fn receive_key_event(virtual_event_type_id: usize, alt_key: usize, ctrl_key: usize, meta_key: usize, shift_key: usize, code_id: usize) {
            let success = rsx_embedding::types::Runtime::static_receive_key_event((virtual_event_type_id as u8, alt_key != 0, ctrl_key != 0, meta_key != 0, shift_key != 0, code_id as u8));
            success.expect("Panic: Couldn't receive virtual key event")
        }

        #[no_mangle]
        pub fn receive_mouse_event(virtual_event_type_id: usize, alt_key: usize, ctrl_key: usize, meta_key: usize, shift_key: usize, button_id: usize, mouse_x: usize, mouse_y: usize) {
            let success = rsx_embedding::types::Runtime::static_receive_mouse_event((virtual_event_type_id as u8, alt_key != 0, ctrl_key != 0, meta_key != 0, shift_key != 0, button_id as u8, mouse_x as u32, mouse_y as u32));
            success.expect("Panic: Couldn't receive virtual mouse event")
        }

        #[no_mangle]
        pub fn poll_events() -> *mut i8 {
            let success = rsx_embedding::types::Runtime::static_poll_events();
            success.expect("Panic: Couldn't poll events").into_raw()
        }
    }
}
