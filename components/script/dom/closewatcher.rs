/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */
use crate::dom::globalscope::GlobalScope;
use std::rc::Rc;
use js::gc::HandleObject;
use dom_struct::dom_struct;
use script_bindings::codegen::GenericBindings::CloseWatcherBinding::CloseWatcherOptions;
use script_bindings::codegen::GenericBindings::EventHandlerBinding::EventHandlerNonNull;

use script_bindings::root::DomRoot;
use script_bindings::script_runtime::CanGc;
// Checks struct syntax & implements HasParent


use crate::dom::bindings::codegen::Bindings::CloseWatcherBinding::CloseWatcherMethods;
use crate::dom::bindings::codegen::DomTypeHolder::DomTypeHolder;
use crate::dom::bindings::reflector::{Reflector, reflect_dom_object_with_proto}; // Helps figure out object type
use crate::dom::eventtarget::EventTarget;
use crate::dom::window::Window;

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
    /// <https://html.spec.whatwg.org/multipage/#dom-closewatcher-requestclose>
    fn RequestClose(&self) {
        todo!()
    }

    /// <https://html.spec.whatwg.org/multipage/interaction.html#dom-closewatcher-close>
    fn Close(&self) {
        todo!()
    }

    /// <https://html.spec.whatwg.org/multipage/interaction.html#dom-closewatcher-destroy>
    fn Destroy(&self) {
        todo!()
    }
    
    /// <https://html.spec.whatwg.org/multipage/#handler-closewatcher-oncancel>
    event_handler!(cancel, GetOncancel, SetOncancel);

    /// <https://html.spec.whatwg.org/multipage/#handler-closewatcher-onclose>
    event_handler!(close, GetOnclose, SetOnclose);

    /// <https://html.spec.whatwg.org/multipage/interaction.html#dom-closewatcher>
    fn Constructor(window: &Window, proto: Option<HandleObject>, can_gc: CanGc, options: &CloseWatcherOptions) -> DomRoot<CloseWatcher> {
        todo!()
    }
}