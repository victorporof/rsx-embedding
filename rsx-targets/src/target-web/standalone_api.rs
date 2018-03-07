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

use std::cell::{Ref, RefCell, RefMut};

use rsx_primitives::prelude::{DOMTree, ResourceGroup};
use rsx_shared::traits::TRuntime;

use types::Runtime;

thread_local! {
    pub static SINGLETON: RefCell<Option<Runtime>> = Default::default();
}

impl Runtime {
    pub fn init<S, R>(setup: S, render: R)
    where
        S: Fn(&mut ResourceGroup),
        R: Fn() -> DOMTree
    {
        SINGLETON.with(|v| *v.borrow_mut() = Some(Runtime::new(&(), setup, render)));
    }

    pub fn get<F, O>(f: F) -> O
    where
        F: FnOnce(Ref<Runtime>) -> O
    {
        SINGLETON.with(|v| f(Ref::map(v.borrow(), |v| v.as_ref().unwrap())))
    }

    pub fn get_mut<F, O>(f: F) -> O
    where
        F: FnOnce(RefMut<Runtime>) -> O
    {
        SINGLETON.with(|v| f(RefMut::map(v.borrow_mut(), |v| v.as_mut().unwrap())))
    }
}
