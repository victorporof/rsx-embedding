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

use std::rc::Rc;

use rsx_native_renderer::glutin::Event;
use rsx_native_renderer::webrender::api::{BuiltDisplayList, LayoutSize, PipelineId, RenderApi, ResourceUpdates};
use rsx_primitives::prelude::{DOMTree, ResourceGroup};
use rsx_shared::traits::TRuntime;

use util;

pub struct Runtime {
    pub(crate) tree: DOMTree,
    pub(crate) resources: ResourceGroup
}

impl TRuntime for Runtime {
    type RootRendererAPI = Rc<RenderApi>;
    type DOMResources = ResourceGroup;
    type DOMTree = DOMTree;
    type VirtualEventMetadata = (Event,);
    type ReflowMetadata = (PipelineId, LayoutSize);
    type BuiltDisplayList = BuiltDisplayList;
    type ResourceUpdates = ResourceUpdates;

    fn new<S, R>(api: &Self::RootRendererAPI, setup: S, render: R) -> Self
    where
        S: Fn(&mut Self::DOMResources),
        R: Fn() -> Self::DOMTree
    {
        let (tree, resources) = util::make_constellation(api, setup, render);
        Runtime { tree, resources }
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

    fn generate_display_list(&mut self, (pipeline_id, layout_size): Self::ReflowMetadata) -> Self::BuiltDisplayList {
        util::generate_display_list(&mut self.tree, pipeline_id, layout_size).serialize()
    }
}
