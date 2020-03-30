//! Utils is a dumping ground for various methods that don't really have a particular module they
//! belong to. These are typically internal, and if you rely on them... well, don't be surprised if
//! they go away one day.

use core_graphics::base::CGFloat;

use objc::{Encode, Encoding};
use objc::runtime::Object;
use objc_id::ShareId;

/// A generic trait that's used throughout multiple different controls in this framework - acts as
/// a guard for whether something is a (View|etc)Controller. Only needs to return the backing node.
pub trait Controller {
    fn get_backing_node(&self) -> ShareId<Object>;
}


/// Utility method for taking a pointer and grabbing the corresponding delegate in Rust. This is
/// theoretically safe:
///
/// - The object (`this`) is owned by the wrapping component (e.g, a `Window`). It's released when
/// the `Window` is released.
/// - The only other place where you can retrieve a `Window` (or such control) is in the respective
/// delegate `did_load()` method, where you're passed one. This variant never includes the
/// delegate.
/// - Thus, provided the root object still exists, this pointer should be valid (root objects Box
/// them, so they ain't movin').
/// - The way this _could_ fail would be if the programmer decides to clone their `Window` or such
/// object deeper into the stack (or elsewhere in general). This is why we don't allow them to be
/// cloned, though.
/// 
/// This is, like much in this framework, subject to revision pending more thorough testing and
/// checking.
pub fn load<'a, T>(this: &'a Object, ptr_name: &str) -> &'a T {
    unsafe {
        let ptr: usize = *this.get_ivar(ptr_name);
        let obj = ptr as *const T;
        &*obj
    }
}

/// Upstream core graphics does not implement Encode for certain things, so we wrap them here -
/// these are only used in reading certain types passed to us from some delegate methods.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CGSize {
    pub width: CGFloat,
    pub height: CGFloat,
}

unsafe impl Encode for CGSize {
    fn encode() -> Encoding {
        let encoding = format!("{{CGSize={}{}}}",
            CGFloat::encode().as_str(),
            CGFloat::encode().as_str()
        );
        
        unsafe { Encoding::from_str(&encoding) }
    }
}
