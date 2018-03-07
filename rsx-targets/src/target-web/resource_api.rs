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

use rsx_resources::fonts::types::{EncodedFont, FontId};
use rsx_resources::images::types::{EncodedImage, ImageId};
use rsx_shared::traits::{TEncodedFont, TEncodedImage, TFontCache, TImageCache, TResourceGroup};

use helpers::string_from;
use types::Runtime;

impl Runtime {
    pub fn load_font(&mut self, font_name: *mut c_char, data_uri: *mut c_char, face_index: usize) -> Option<()> {
        let mut fonts = self.resources.fonts();

        let font_id = FontId::new(string_from(font_name));
        let encoded = EncodedFont::from_data_uri(string_from(data_uri)).ok()?;
        fonts.add_font_with_id(font_id, &encoded, face_index);

        Some(())
    }

    pub fn load_image(&mut self, virtual_src: *mut c_char, data_uri: *mut c_char) -> Option<()> {
        let mut images = self.resources.images();

        let image_id = ImageId::new(string_from(virtual_src));
        let encoded = EncodedImage::from_data_uri(string_from(data_uri)).ok()?;
        images.add_image_with_id(image_id, &encoded);

        Some(())
    }
}
