//! Utils is a dumping ground for various methods that don't really have a particular module they
//! belong to. These are typically internal, and if you rely on them... well, don't be surprised if
//! they go away one day.

use std::rc::Rc;
use std::cell::RefCell;

use objc::runtime::Object;
use objc_id::ShareId;

/// A generic trait that's used throughout multiple different controls in this framework - acts as
/// a guard for whether something is a (View|etc)Controller. Only needs to return the backing node.
pub trait Controller {
    fn get_backing_node(&self) -> ShareId<Object>;
}

/// Used for moving a pointer back into an Rc, so we can work with the object held behind it. Note
/// that it's very important to make sure you reverse this when you're done (using
/// `Rc::into_raw()`) otherwise you'll cause problems due to the `Drop` logic.
pub fn load<T>(this: &Object, ptr: &str) -> Rc<RefCell<T>> {
    unsafe {
        let ptr: usize = *this.get_ivar(ptr);
        let view_ptr = ptr as *const RefCell<T>;
        Rc::from_raw(view_ptr)
    }
}
