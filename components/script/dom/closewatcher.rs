/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */
use crate::dom::globalscope::GlobalScope;
use std::rc::Rc;
use js::gc::HandleObject;
use dom_struct::dom_struct;
use script_bindings::codegen::GenericBindings::CloseWatcherBinding::CloseWatcherOptions;
use script_bindings::codegen::GenericBindings::EventHandlerBinding::EventHandlerNonNull;
use script_bindings::codegen::GenericBindings::WindowBinding::WindowMethods;
use script_bindings::error::{Fallible, Error};
use script_bindings::reflector::DomGlobalGeneric;
use script_bindings::root::DomRoot;
use script_bindings::script_runtime::CanGc;
// Checks struct syntax & implements HasParent


use crate::dom::bindings::codegen::Bindings::CloseWatcherBinding::CloseWatcherMethods;
use crate::dom::bindings::codegen::DomTypeHolder::DomTypeHolder;
use crate::dom::bindings::reflector::{Reflector, reflect_dom_object_with_proto, DomGlobal}; // Helps figure out object type
use crate::dom::eventtarget::EventTarget;
use crate::dom::window::Window;

#[dom_struct]
pub(crate) struct CloseWatcher {
    event_target: EventTarget,
    enabled: bool,
}

impl CloseWatcher {
    fn new_inherited() -> Self {
        Self {
            event_target: EventTarget::new_inherited(),
            enabled: true,
        }
    }
    fn new(window:&Window, proto: Option<HandleObject>, can_gc: CanGc) -> DomRoot<Self> {
        reflect_dom_object_with_proto(Box::new(Self::new_inherited()), window, proto, can_gc)
    }
    
    /// Set the enabled state of the CloseWatcher
    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
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
    fn Constructor(window: &Window, proto: Option<HandleObject>, can_gc: CanGc, options: &CloseWatcherOptions) -> Fallible<DomRoot<CloseWatcher>> {
        // 1. If this's relevant global object's associated Document is not fully active, then throw an "InvalidStateError" DOMException.
        if !window.Document().is_fully_active() {
            return Err(Error::InvalidState);
        }
       
        // 2. Let closeWatcher be the result of establishing a close watcher given this's relevant global object, with:
        let close_watcher = CloseWatcher::new(window, proto, can_gc);
        
        //TODO: Step 3 relies on AbortSignal being implemented
        
        return Ok(close_watcher);
    }
}