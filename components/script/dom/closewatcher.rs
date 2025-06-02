/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */
use crate::dom::globalscope::GlobalScope;
use std::rc::Rc;
use js::gc::HandleObject;
use dom_struct::dom_struct;
use script_bindings::codegen::GenericBindings::CloseWatcherBinding::CloseWatcherOptions;
use script_bindings::codegen::GenericBindings::EventHandlerBinding::EventHandlerNonNull;
use script_bindings::codegen::PrototypeList::ID::GlobalScope;
use script_bindings::root::DomRoot;
use script_bindings::script_runtime::CanGc;
// Checks struct syntax & implements HasParent


use crate::dom::bindings::codegen::Bindings::CloseWatcherBinding::CloseWatcherMethods;
use crate::dom::bindings::codegen::DomTypeHolder::DomTypeHolder;
use crate::dom::bindings::reflector::{Reflector, reflect_dom_object_with_proto}; // Helps figure out object type
use crate::dom::eventtarget::EventTarget;

#[dom_struct]
pub(crate) struct CloseWatcher {
    event_target: EventTarget,
}

impl CloseWatcher {
    fn new_inherited() -> Self {
        Self {
            event_target: EventTarget::new_inherited()
        }
    }
}

impl CloseWatcherMethods<crate::DomTypeHolder> for CloseWatcher {
    fn RequestClose(&self) {
        todo!()
    }

    fn Close(&self) {
        todo!()
    }

    fn Destroy(&self) {
        todo!()
    }

    fn GetOncancel(&self) -> Option<Rc<EventHandlerNonNull<DomTypeHolder>>> {
        todo!()
    }

    fn SetOncancel(&self, value: Option<Rc<EventHandlerNonNull<DomTypeHolder>>>) {
        todo!()
    }

    fn GetOnclose(&self) -> Option<Rc<EventHandlerNonNull<DomTypeHolder>>> {
        todo!()
    }

    fn SetOnclose(&self, value: Option<Rc<EventHandlerNonNull<DomTypeHolder>>>) {
        todo!()
    }

    fn Constructor(global: &GlobalScope, proto: Option<HandleObject>, can_gc: CanGc, options: &CloseWatcherOptions) -> DomRoot<DomTypeHolder::CloseWatcher> {
        todo!()
    }
}