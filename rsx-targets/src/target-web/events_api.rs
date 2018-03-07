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

use rsx_event_manager::types::{KeyCode, KeyCodeId, MouseButton, MouseButtonId};
use rsx_shared::traits::{FromPrimitive, TEventManager};
use rsx_shared::types::{VirtualEventType, VirtualEventTypeId};
use serde_json;
use std::ffi::CString;

use types::Runtime;

#[cfg_attr(rustfmt, rustfmt_skip)]
pub type KeyEventData = (VirtualEventTypeId, bool, bool, bool, bool, KeyCodeId);

#[cfg_attr(rustfmt, rustfmt_skip)]
pub type MouseEventData = (VirtualEventTypeId, bool, bool, bool, bool, MouseButtonId, u32, u32);

impl Runtime {
    pub fn receive_key_event(&mut self, (event_type_id, alt, ctrl, meta, shift, code_id): KeyEventData) -> Option<()> {
        let event_manager = &mut self.event_manager;

        let virtual_event_type = VirtualEventType::from_u8(event_type_id)?;
        let modifiers = (alt, ctrl, meta, shift);
        let code = KeyCode::from_u8(code_id)?;
        event_manager.receive_key_event(virtual_event_type, (modifiers, code));

        Some(())
    }

    pub fn receive_mouse_event(&mut self, (event_type_id, alt, ctrl, meta, shift, button_id, pos_x, pos_y): MouseEventData) -> Option<()> {
        let event_manager = &mut self.event_manager;

        let virtual_event_type = VirtualEventType::from_u8(event_type_id)?;
        let modifiers = (alt, ctrl, meta, shift);
        let button = MouseButton::from_u8(button_id)?;
        let mouse_pos = (pos_x, pos_y);
        event_manager.receive_mouse_event(virtual_event_type, (modifiers, button, mouse_pos));

        Some(())
    }

    pub fn poll_events(&mut self) -> Option<CString> {
        let event_manager = &mut self.event_manager;

        let mut events = Vec::new();
        event_manager.intercept_events(&self.tree, |e| events.push(e));

        let json = serde_json::to_string(&events).ok()?;
        CString::new(json).ok()
    }
}
