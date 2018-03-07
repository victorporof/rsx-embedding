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

use std::ffi::CString;
use std::os::raw::{c_char, c_uchar};

use rsx_primitives::build::types::{BuiltDisplayList, DisplayListBuilder, ResourceUpdates};
use rsx_primitives::compare::types::DisplayListDiff;
use rsx_primitives::prelude::{DOMTree, EventManager, ResourceGroup};
use rsx_primitives::rsx_stylesheet::types::StyleDeclarations;
use rsx_primitives::types::DisplayList;
use rsx_shared::traits::TRuntime;
use rsx_shared::types::{DOMNodeRawId, EventTypeId, KnownElementNameId};

use helpers;
use util;
use web_events_api::{KeyEventData, MouseEventData};

pub struct Runtime {
    pub(crate) tree: DOMTree,
    pub(crate) styles: Vec<StyleDeclarations>,
    pub(crate) resources: ResourceGroup,
    pub(crate) event_manager: EventManager,
    pub(crate) old_display_list: DisplayList,
    pub(crate) old_zombie_strings: Vec<*mut c_char>,
    pub(crate) old_zombie_vecs: Vec<(*mut c_uchar, usize)>
}

impl Runtime {
    // Standard API
    pub fn static_generate_display_list((width, height): (u32, u32)) -> BuiltDisplayList {
        Runtime::get_mut(|mut runtime| runtime.generate_display_list((width, height)))
    }

    pub fn static_generate_display_list_diff((width, height): (u32, u32)) -> DisplayListDiff<DisplayListBuilder> {
        Runtime::get_mut(|mut runtime| runtime.generate_display_list_diff((width, height)))
    }

    pub fn static_take_resource_updates() -> ResourceUpdates {
        Runtime::get_mut(|mut runtime| runtime.take_resource_updates())
    }

    // Standard FFI API

    #[cfg(feature = "bincode-display-list")]
    pub fn static_generate_display_list_ptr((width, height): (u32, u32)) -> *mut c_char {
        Runtime::get_mut(|mut runtime| {
            let (ptr, len) = runtime.generate_display_list((width, height)).into();
            runtime.old_zombie_vecs.push((ptr, len));
            let info_ptr = helpers::ptr_and_len_string_from(ptr, len);
            runtime.old_zombie_strings.push(info_ptr);
            info_ptr
        })
    }

    #[cfg(feature = "json-display-list")]
    pub fn static_generate_display_list_ptr((width, height): (u32, u32)) -> *mut c_char {
        Runtime::get_mut(|mut runtime| {
            let ptr = runtime.generate_display_list((width, height)).into();
            runtime.old_zombie_strings.push(ptr);
            ptr
        })
    }

    pub fn static_generate_display_list_diff_ptr((width, height): (u32, u32)) -> *mut c_char {
        Runtime::get_mut(|mut runtime| {
            let ptr = runtime.generate_display_list_diff((width, height)).into();
            runtime.old_zombie_strings.push(ptr);
            ptr
        })
    }

    pub fn static_take_resource_updates_ptr() -> *mut c_char {
        Runtime::get_mut(|mut runtime| {
            let ptr = runtime.take_resource_updates().into();
            runtime.old_zombie_strings.push(ptr);
            ptr
        })
    }

    pub fn static_gc() {
        Runtime::get_mut(|mut runtime| {
            runtime.old_zombie_strings.drain(..).for_each(|raw| unsafe {
                CString::from_raw(raw);
            });
            runtime.old_zombie_vecs.drain(..).for_each(|raw| unsafe {
                Vec::from_raw_parts(raw.0, raw.1, raw.1);
            });
        });
    }

    // Debug API
    pub fn static_print_diag() -> Option<()> {
        Runtime::get(|runtime| runtime.print_diag())
    }

    // Resource API

    pub fn static_load_font(font_name: *mut c_char, data_uri: *mut c_char, face_index: usize) -> Option<()> {
        Runtime::get_mut(|mut runtime| runtime.load_font(font_name, data_uri, face_index))
    }

    pub fn static_load_image(virtual_src: *mut c_char, data_uri: *mut c_char) -> Option<()> {
        Runtime::get_mut(|mut runtime| runtime.load_image(virtual_src, data_uri))
    }

    // DOM API

    pub fn static_create_orphan_normal_node(element_name_id: KnownElementNameId) -> Option<DOMNodeRawId> {
        Runtime::get_mut(|mut runtime| runtime.create_orphan_normal_node(element_name_id))
    }

    pub fn static_create_orphan_inline_text_node(text_content: *mut c_char) -> Option<DOMNodeRawId> {
        Runtime::get_mut(|mut runtime| runtime.create_orphan_inline_text_node(text_content))
    }

    pub fn static_append_child(parent_id: DOMNodeRawId, child_id: DOMNodeRawId) -> Option<()> {
        Runtime::get_mut(|mut runtime| runtime.append_child(parent_id, child_id))
    }

    pub fn static_append_child_to_container(child_id: DOMNodeRawId) -> Option<()> {
        Runtime::get_mut(|mut runtime| runtime.append_child_to_container(child_id))
    }

    pub fn static_set_styles(node_id: DOMNodeRawId, style_ids: *mut u8, len: usize) -> Option<()> {
        Runtime::get_mut(|mut runtime| runtime.set_styles(node_id, style_ids, len))
    }

    pub fn static_set_text_content(node_id: DOMNodeRawId, text_content: *mut c_char) -> Option<()> {
        Runtime::get_mut(|mut runtime| runtime.set_text_content(node_id, text_content))
    }

    pub fn static_add_event_listener(node_id: DOMNodeRawId, event_type_id: EventTypeId) -> Option<()> {
        Runtime::get_mut(|mut runtime| runtime.add_event_listener(node_id, event_type_id))
    }

    pub fn static_remove_event_listener(node_id: DOMNodeRawId, event_type_id: EventTypeId) -> Option<()> {
        Runtime::get_mut(|mut runtime| runtime.remove_event_listener(node_id, event_type_id))
    }

    // Stylesheet API

    pub fn static_register_style(style_declarations: *mut c_char) -> Option<usize> {
        Runtime::get_mut(|mut runtime| runtime.register_style(style_declarations))
    }

    // Events API

    pub fn static_receive_key_event(event: KeyEventData) -> Option<()> {
        Runtime::get_mut(|mut runtime| runtime.receive_key_event(event))
    }

    pub fn static_receive_mouse_event(event: MouseEventData) -> Option<()> {
        Runtime::get_mut(|mut runtime| runtime.receive_mouse_event(event))
    }

    pub fn static_poll_events() -> Option<CString> {
        Runtime::get_mut(|mut runtime| runtime.poll_events())
    }
}

impl TRuntime for Runtime {
    type RootRendererAPI = ();
    type DOMResources = ResourceGroup;
    type DOMTree = DOMTree;
    type VirtualEventMetadata = ();
    type ReflowMetadata = (u32, u32);
    type BuiltDisplayList = BuiltDisplayList;
    type ResourceUpdates = ResourceUpdates;

    fn new<S, R>(_: &Self::RootRendererAPI, setup: S, render: R) -> Self
    where
        S: Fn(&mut Self::DOMResources),
        R: Fn() -> Self::DOMTree
    {
        let (tree, resources) = util::make_constellation(setup, render);
        let styles = Vec::new();
        let event_manager = EventManager::default();
        let old_display_list = DisplayList::default();
        let old_zombie_strings = Vec::new();
        let old_zombie_vecs = Vec::new();

        Runtime {
            tree,
            styles,
            resources,
            event_manager,
            old_display_list,
            old_zombie_strings,
            old_zombie_vecs
        }
    }

    fn should_set_window_position(&mut self) -> Option<(i32, i32)> {
        None
    }

    fn should_set_window_size(&mut self) -> Option<(u32, u32)> {
        None
    }

    fn should_redraw(&mut self) -> bool {
        false
    }

    fn handle_event(&mut self, _: Self::VirtualEventMetadata) -> bool {
        false
    }

    fn take_resource_updates(&mut self) -> Self::ResourceUpdates {
        util::take_resource_updates(&mut self.resources)
    }

    fn generate_display_list(&mut self, (width, height): Self::ReflowMetadata) -> Self::BuiltDisplayList {
        util::generate_display_list(&mut self.tree, width, height).serialize()
    }
}

impl Runtime {
    pub fn generate_display_list_diff(&mut self, (width, height): (u32, u32)) -> DisplayListDiff<DisplayListBuilder> {
        let new_display_list = util::generate_display_list(&mut self.tree, width, height);
        let display_list_diff = self.old_display_list.diff(&new_display_list);

        self.old_display_list = new_display_list;
        display_list_diff
    }
}
