use std::cell::{Ref, RefCell};
use std::rc::Rc;

use objc::rc::{Id, Owned};
use objc::runtime::Object;

use crate::foundation::id;

/// A wrapper for single-threaded `ObjcProperty` types.
///
/// An `ObjcProperty` is something that exists on the Objective-C side that we want to interact with, and
/// support cloning with respect to our side and the general Rust rules. Thus, we do a layer of
/// Rc/RefCell to shield things and make life easier.
///
/// It is possible we could remove the `Id` wrapper in here if we're just doing this ourselves, and
/// is probably worth investigating at some point.
#[derive(Clone, Debug)]
pub struct ObjcProperty(Rc<RefCell<Id<Object, Owned>>>);

impl ObjcProperty {
    /// Given an Objective-C object, retains it and wraps it as a `Property`.
    pub fn retain(obj: id) -> Self {
        ObjcProperty(Rc::new(RefCell::new(unsafe { Id::retain(obj).unwrap() })))
    }

    /// Runs a handler with mutable access for the underlying Objective-C object.
    ///
    /// Note that this is mutable access from the Rust side; we make every effort to ensure things are valid
    /// on the Objective-C side as well, but there be dragons.
    pub fn with_mut<F: Fn(id)>(&self, handler: F) {
        let mut obj = self.0.borrow_mut();
        handler(&mut **obj);
    }

    /// Runs a handler with the underlying Objective-C type.
    ///
    /// The handler can return whatever; this is primarily intended for dynamically calling getters
    /// on the underlying type.
    pub fn get<R, F: Fn(&Object) -> R>(&self, handler: F) -> R {
        let obj = self.0.borrow();
        handler(&**obj)
    }

    pub fn get_ref(&self) -> Ref<'_, Id<Object, Owned>> {
        self.0.borrow()
    }
}

/// A wrapper for a single-threaded nullable `Property`.
#[derive(Debug, Default)]
pub struct PropertyNullable<T>(Rc<RefCell<Option<T>>>);

impl<T> PropertyNullable<T> {
    pub fn new(obj: T) -> Self {
        Self(Rc::new(RefCell::new(Some(obj))))
    }

    pub fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }

    pub fn with<F>(&self, handler: F)
    where
        F: Fn(&T),
    {
        let borrow = self.0.borrow();
        if let Some(s) = &*borrow {
            handler(s);
        }
    }

    pub fn set(&self, obj: T) {
        let mut borrow = self.0.borrow_mut();
        *borrow = Some(obj);
    }
}
