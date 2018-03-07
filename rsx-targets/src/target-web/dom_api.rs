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

use rsx_dom::types::DOMTagName;
use rsx_primitives::prelude::{DOMNode, DOMNodeId};
use rsx_shared::traits::{FromPrimitive, TEventManager, ToPrimitive};
use rsx_shared::types::{DOMNodeRawId, EventType, EventTypeId, KnownElementName, KnownElementNameId};

use helpers::string_from;
use types::Runtime;

impl Runtime {
    pub fn create_orphan_normal_node(&mut self, element_name_id: KnownElementNameId) -> Option<DOMNodeRawId> {
        let element_name = KnownElementName::from_u16(element_name_id).unwrap_or(KnownElementName::View);
        let node = DOMNode::from(DOMTagName::from(element_name));
        self.tree.alloc(node).to_u64()
    }

    pub fn create_orphan_inline_text_node(&mut self, text_content: *mut c_char) -> Option<DOMNodeRawId> {
        let node = DOMNode::from(string_from(text_content));
        self.tree.alloc(node).to_u64()
    }

    pub fn append_child(&mut self, parent_id: DOMNodeRawId, child_id: DOMNodeRawId) -> Option<()> {
        let resources = &self.resources;

        let parent_id = DOMNodeId::from_u64(parent_id)?;
        let child_id = DOMNodeId::from_u64(child_id)?;
        let mut parent_ref = self.tree.get_mut(parent_id);
        parent_ref.append_with_layout(child_id, resources).ok()?;

        Some(())
    }

    pub fn append_child_to_container(&mut self, child_id: DOMNodeRawId) -> Option<()> {
        let resources = &self.resources;

        let child_id = DOMNodeId::from_u64(child_id)?;
        let mut parent_ref = self.tree.root_mut();
        parent_ref.append_with_layout(child_id, resources).ok()?;

        Some(())
    }

    pub fn set_text_content(&mut self, node_id: DOMNodeRawId, text_content: *mut c_char) -> Option<()> {
        let resources = &self.resources;

        let node_id = DOMNodeId::from_u64(node_id)?;
        let mut node_ref = self.tree.get_mut(node_id);
        node_ref.set_text_content(string_from(text_content), resources);

        Some(())
    }

    pub fn set_styles(&mut self, node_id: DOMNodeRawId, style_ids: *mut u8, len: usize) -> Option<()> {
        let styles = &self.styles;

        let node_id = DOMNodeId::from_u64(node_id)?;
        let mut node_ref = self.tree.get_mut(node_id);

        node_ref.reset_styles()?;

        for index in 0..len {
            let style_id = unsafe { *style_ids.offset(index as isize) };
            let style_declarations = styles.get(style_id as usize)?;
            node_ref.apply_styles(style_declarations.clone())?;
        }

        Some(())
    }

    pub fn add_event_listener(&mut self, node_id: DOMNodeRawId, event_type_id: EventTypeId) -> Option<()> {
        let event_manager = &mut self.event_manager;

        let target_id = DOMNodeId::from_u64(node_id)?;
        let event_type = EventType::from_u8(event_type_id)?;
        event_manager.add_event_listener(target_id, event_type, ());

        Some(())
    }

    pub fn remove_event_listener(&mut self, node_id: DOMNodeRawId, event_type_id: EventTypeId) -> Option<()> {
        let event_manager = &mut self.event_manager;

        let target_id = DOMNodeId::from_u64(node_id)?;
        let event_type = EventType::from_u8(event_type_id)?;
        event_manager.remove_event_listener(target_id, event_type, ());

        Some(())
    }
}
