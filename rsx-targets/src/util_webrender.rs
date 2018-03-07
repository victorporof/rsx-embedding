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

use rsx_layout::types::LayoutReflowDirection;
use rsx_primitives::build::types::ResourceUpdates;
use rsx_primitives::prelude::{DOMTree, FileCache, FontCache, ImageCache, ResourceGroup};
use rsx_primitives::types::DisplayList;
use rsx_primitives::webrender::api::{LayoutSize, PipelineId, RenderApi};
use rsx_shared::traits::{TDOMNode, TFontCache, TFontKeysAPI, TImageCache, TImageKeysAPI, TResourceGroup};

pub use std::ffi::CString;
pub use std::os::raw::c_char;

pub fn make_constellation<S, R>(api: &Rc<RenderApi>, setup: S, render: R) -> (DOMTree, ResourceGroup)
where
    S: Fn(&mut ResourceGroup),
    R: Fn() -> DOMTree
{
    let files = FileCache::new().unwrap();
    let images = ImageCache::new(TImageKeysAPI::new(Rc::clone(api))).unwrap();
    let fonts = FontCache::new(TFontKeysAPI::new(Rc::clone(api))).unwrap();
    let mut resources = ResourceGroup::new(files, images, fonts);
    setup(&mut resources);

    let mut tree = render();
    tree.generate_layout_tree(&resources);

    (tree, resources)
}

pub fn generate_display_list(tree: &mut DOMTree, pipeline_id: PipelineId, layout_size: LayoutSize) -> DisplayList {
    let LayoutSize { width, height, .. } = layout_size;
    tree.reflow_subtree(width as u32, height as u32, LayoutReflowDirection::LTR);
    DisplayList::from(tree, pipeline_id, layout_size)
}

pub fn take_resource_updates(resources: &mut ResourceGroup) -> ResourceUpdates {
    let images = resources.images().take_resource_updates();
    let fonts = resources.fonts().take_resource_updates();
    let mut updates = ResourceUpdates::new();
    updates.merge(images);
    updates.merge(fonts);
    updates
}
