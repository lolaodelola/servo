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
use crate::dom::event::{Event, EventBubbles, EventCancelable, EventDefault};
use crate::dom::window::Window;

#[dom_struct]
pub(crate) struct CloseWatcher {
    event_target: EventTarget,
    // https://html.spec.whatwg.org/multipage/interaction.html#close-watcher
    // A close watcher is a struct with the following items:
 
    // An is running cancel action boolean.
    is_running_cancel_action: bool,
    // A get enabled state, an algorithm accepting no arguments and returning a boolean. This algorithm can never throw an exception.
    enabled: bool,
}

impl CloseWatcher {
    fn new_inherited() -> Self {
        Self {
            event_target: EventTarget::new_inherited(),
            is_running_cancel_action: false,
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
    
    fn request_to_close(&mut self, require_history_action_activation: bool, can_gc: CanGc) -> bool {
        // 1. If closeWatcher is not active, then return true.
        // 2. If the result of running closeWatcher's get enabled state is false, then return true.
        // 3. If closeWatcher's is running cancel action is true, then return true.
        if !self.is_active() || !self.enabled || !self.is_running_cancel_action {
            return true;
        } 
        
        // 4. Let window be closeWatcher's window.
        let global = self.global();
        let window = global.as_window();
        
        // 5. If window's associated Document is not fully active, then return true.
        if !window.Document().is_fully_active() {
            return false;
        }
        
 
        // 6. Let canPreventClose be true if requireHistoryActionActivation is false, 
        // or if window's close watcher manager's groups's size is less than 
        // window's close watcher manager's allowed number of groups, 
        // and window has history-action activation; otherwise false.
        
        // todo(implement history_activation which requires user_activation)
        let has_valid_history_activation = true;
        
        // todo(implement in closewatcher manager)
        let can_grow = true;
        
        let can_prevent_close = (
            !require_history_action_activation ||
                (can_grow && has_valid_history_activation)
        ).into();
       
        
        // 7. Set closeWatcher's is running cancel action to true.
        self.is_running_cancel_action = true;
        
        // 8. Let shouldContinue be the result of running closeWatcher's cancel action given canPreventClose.
        let event = Event::new(&self.global(), atom!("cancel"), EventBubbles::DoesNotBubble, can_prevent_close);
        self.event_target.dispatch_event(event, can_gc);
        let should_continue = event.get_cancel_state() != EventDefault::Prevented;
        // 9. Set closeWatcher's is running cancel action to false.
        self.is_running_cancel_action = false;
        
        // 10. If shouldContinue is false, then:
        if !should_continue {
            //  10.1. Assert: canPreventClose is true.
            debug_assert!(can_prevent_close);
            //  10.2 Consume history-action user activation given window.
            // todo()
            //  10.3 Return false.
            return false;
        }
        
        // 11. Close closeWatcher.
        
        // 12. Return true.
        return true;
    }
    
    fn is_active(&self) -> bool {
        false
    }

}

impl CloseWatcherMethods<crate::DomTypeHolder> for CloseWatcher {
    /// <https://html.spec.whatwg.org/multipage/#dom-closewatcher-requestclose>
    fn RequestClose(&mut self) {
        self.request_to_close(false, CanGc::note());
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