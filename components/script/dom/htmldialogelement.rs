/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use dom_struct::dom_struct;
use html5ever::{LocalName, Prefix, local_name, namespace_url, ns};
use js::rust::HandleObject;
use stylo_dom::ElementState;
use crate::dom::bindings::cell::DomRefCell;
use crate::dom::bindings::codegen::Bindings::HTMLDialogElementBinding::HTMLDialogElementMethods;
use crate::dom::bindings::inheritance::Castable;
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::str::DOMString;
use crate::dom::document::Document;
use crate::dom::element::Element;
use crate::dom::eventtarget::EventTarget;
use crate::dom::htmlelement::HTMLElement;
use crate::dom::node::{Node, NodeTraits};
use crate::script_runtime::CanGc;

#[dom_struct]
pub(crate) struct HTMLDialogElement {
    htmlelement: HTMLElement,
    return_value: DomRefCell<DOMString>,
}

impl HTMLDialogElement {
    fn new_inherited(
        local_name: LocalName,
        prefix: Option<Prefix>,
        document: &Document,
    ) -> HTMLDialogElement {
        HTMLDialogElement {
            htmlelement: HTMLElement::new_inherited(local_name, prefix, document),
            return_value: DomRefCell::new(DOMString::new()),
        }
    }

    #[cfg_attr(crown, allow(crown::unrooted_must_root))]
    pub(crate) fn new(
        local_name: LocalName,
        prefix: Option<Prefix>,
        document: &Document,
        proto: Option<HandleObject>,
        can_gc: CanGc,
    ) -> DomRoot<HTMLDialogElement> {
        Node::reflect_node_with_proto(
            Box::new(HTMLDialogElement::new_inherited(
                local_name, prefix, document,
            )),
            document,
            proto,
            can_gc,
        )
    }
}

impl HTMLDialogElementMethods<crate::DomTypeHolder> for HTMLDialogElement {
    // https://html.spec.whatwg.org/multipage/#dom-dialog-open
    make_bool_getter!(Open, "open");

    // https://html.spec.whatwg.org/multipage/#dom-dialog-open
    make_bool_setter!(SetOpen, "open");

    // https://html.spec.whatwg.org/multipage/#dom-dialog-returnvalue
    fn ReturnValue(&self) -> DOMString {
        let return_value = self.return_value.borrow();
        return_value.clone()
    }

    // https://html.spec.whatwg.org/multipage/#dom-dialog-returnvalue
    fn SetReturnValue(&self, return_value: DOMString) {
        *self.return_value.borrow_mut() = return_value;
    }

    // <https://html.spec.whatwg.org/multipage/#dom-dialog-show>
    fn Show(&self, can_gc: CanGc) {
        let element = self.upcast::<Element>();

    //TODO: STEP 1: If this has an open attribute and is modal of this is false, then return.
         if element.has_attribute(&local_name!("open")) {
                    return;
         }

//         TODO: STEP 2: If this has an open attribute, then throw an "InvalidStateError" DOMException.

//         TODO: STEP 3: If the result of firing an event named beforetoggle, using ToggleEvent,
//         with the cancelable attribute initialized to true, the oldState attribute
//         initialized to "closed", and the newState attribute initialized to "open"
//         at this is false, then return.
        self.upcast::<EventTarget>().fire_cancelable_event(atom!("beforetoggle"), CanGc::note());

//         TODO: STEP 4: If this has an open attribute, then return.

//         TODO: STEP 5: Queue a dialog toggle event task given this, "closed", and "open".

//         TODO: STEP 6: Add an open attribute to this, whose value is the empty string.
         element.set_bool_attribute(&local_name!("open"), true, can_gc);

//         TODO: STEP 7: Assert: this's node document's open dialogs list does not contain this.

//         TODO: STEP 8: Add this to this's node document's open dialogs list.

//         TODO: STEP 9: Set the dialog close watcher with this.

//         TODO: STEP 10: Set this's previously focused element to the focused element.

//         TODO: STEP 11: Let document be this's node document.

//         TODO: STEP 12: Let hideUntil be the result of running topmost popover ancesto
//            given this, document's showing hint popover list, null, and false.

//         TODO: STEP 13: If hideUntil is null, then set hideUntil to the result of
//            running topmost popover ancestor given this, document's showing auto
//            popover list, null, and false.

//         TODO: STEP 14: If hideUntil is null, then set hideUntil to document.

//         TODO: STEP 15: Run hide all popovers until given hideUntil, false, and true.

//         TODO: STEP 16 (Issue #32702): Run the dialog focusing steps given this.

    }

    // https://html.spec.whatwg.org/multipage/#dom-dialog-close
    fn Close(&self, return_value: Option<DOMString>, can_gc: CanGc) {
        let element = self.upcast::<Element>();
        let target = self.upcast::<EventTarget>();

        // Step 1 & 2
        if element
            .remove_attribute(&ns!(), &local_name!("open"), can_gc)
            .is_none()
        {
            return;
        }

        // Step 3
        if let Some(new_value) = return_value {
            *self.return_value.borrow_mut() = new_value;
        }

        // TODO: Step 4 implement pending dialog stack removal

        // Step 5
        self.owner_global()
            .task_manager()
            .dom_manipulation_task_source()
            .queue_simple_event(target, atom!("close"));
    }
}
