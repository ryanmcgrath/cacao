//! This module contains an NSObject subclass that can act as a generic target
//! for action dispatch - e.g, for buttons, toolbars, etc. It loops back around
//! to a Rust callback; you won't be able to necessarily use it like you would
//! elsewhere, but you can message pass to achieve what you need.
//!
//! Note that this is explicitly intended to be 1:1 with a widget; it is not
//! something you should ever attempt to clone or really bother interacting with.
//! It is imperative that this drop whenever a corresponding control/widget
//! is going away.

use std::fmt;
use std::sync::{Arc, Mutex, Once};

use block::{Block, ConcreteBlock, RcBlock};
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc::{class, msg_send, sel, sel_impl};
use objc_id::ShareId;

use crate::foundation::{id, nil, NSString};
use crate::utils::load;

pub static ACTION_CALLBACK_PTR: &str = "rstTargetActionPtr";

/// An Action is just an indirection layer to get around Rust and optimizing
/// zero-sum types; without this, pointers to callbacks will end up being
/// 0x1, and all point to whatever is there first (unsure if this is due to
/// Rust or Cocoa or what).
///
/// Point is, Button aren't created that much in the grand scheme of things,
/// and the heap isn't our enemy in a GUI framework anyway. If someone knows
/// a better way to do this that doesn't require double-boxing, I'm all ears.
pub struct Action(Box<dyn Fn(*const Object) + Send + Sync + 'static>);

impl fmt::Debug for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Action").finish()
    }
}

/// A handler that contains the class for callback storage and invocation on
/// the Objective-C side.
///
/// This effectively wraps the target:action selector usage on NSControl and
/// associated widgets.
///
/// Widgets that use this should keep it around; on drop,
/// it _will_ remove your events somewhat transparently per Cocoa rules.
#[derive(Debug)]
pub struct TargetActionHandler {
    action: Box<Action>,
    invoker: ShareId<Object>,
}

impl TargetActionHandler {
    /// Returns a new TargetEventHandler.
    pub fn new<F: Fn(*const Object) + Send + Sync + 'static>(control: &Object, action: F) -> Self {
        let block = Box::new(Action(Box::new(action)));
        let ptr = Box::into_raw(block);

        let invoker = unsafe {
            ShareId::from_ptr({
                let invoker: id = msg_send![register_invoker_class::<F>(), alloc];
                let invoker: id = msg_send![invoker, init];
                (&mut *invoker).set_ivar(ACTION_CALLBACK_PTR, ptr as usize);
                let _: () = msg_send![control, setAction: sel!(perform:)];
                let _: () = msg_send![control, setTarget: invoker];
                invoker
            })
        };

        TargetActionHandler {
            invoker: invoker,
            action: unsafe { Box::from_raw(ptr) },
        }
    }
}

/// This will fire for an NSButton callback.
extern "C" fn perform<F: Fn(*const Object) + 'static>(this: &mut Object, _: Sel, sender: id) {
    let action = load::<Action>(this, ACTION_CALLBACK_PTR);
    (action.0)(sender.cast_const());
}

/// Due to the way that Rust and Objective-C live... very different lifestyles,
/// we need to find a way to make events work without _needing_ the whole
/// target/action setup you'd use in a standard Cocoa/AppKit/UIKit app.
///
/// Here, we inject a subclass that can store a pointer for a callback. We use
/// this as our target/action combo, which allows passing a
/// generic block over. It's still Rust, so you can't do crazy callbacks, but
/// you can at least fire an event off and do something.
///
/// The `NSButton` owns this object on instantiation, and will release it
/// on drop. We handle the heap copy on the Rust side, so setting the block
/// is just an ivar.
pub(crate) fn register_invoker_class<F: Fn(*const Object) + 'static>() -> *const Class {
    static mut VIEW_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = Once::new();

    INIT.call_once(|| unsafe {
        let superclass = class!(NSObject);
        let mut decl = ClassDecl::new("RSTTargetActionHandler", superclass).unwrap();

        decl.add_ivar::<usize>(ACTION_CALLBACK_PTR);
        decl.add_method(sel!(perform:), perform::<F> as extern "C" fn(&mut Object, _, id));

        VIEW_CLASS = decl.register();
    });

    unsafe { VIEW_CLASS }
}
