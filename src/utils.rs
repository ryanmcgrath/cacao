//! Utils is a dumping ground for various methods that don't really have a particular module they
//! belong to. These are typically internal, and if you rely on them... well, don't be surprised if
//! they go away one day.

use core_graphics::base::CGFloat;

use objc::{class, msg_send, sel, sel_impl};

use objc::{Encode, Encoding};
use objc::runtime::Object;
use objc_id::ShareId;

use crate::foundation::{id, BOOL, YES, NO};

pub mod os {
    use lazy_static::lazy_static;
    use os_info::Version;

    lazy_static! {
        pub static ref OS_VERSION: os_info::Info = os_info::get();
    }

    /// In rare cases we need to check whether something is a specific version of macOS. This is a
    /// runtime check thhat returns a boolean indicating whether the current version is a minimum target.
    #[inline(always)]
    pub fn is_minimum_version(minimum_major: u64) -> bool {
        match OS_VERSION.version() {
            Version::Semantic(os_major, _, _) => { *os_major >= minimum_major },
            _ => false
        }
    }

    /// In rare cases we need to check whether something is a specific version of macOS. This is a
    /// runtime check thhat returns a boolean indicating whether the current version is a minimum target.
    #[inline(always)]
    pub fn is_minimum_semversion(major: u64, minor: u64, patch: u64) -> bool {
        let target = Version::Semantic(major, minor, patch);
        OS_VERSION.version() > &target
    }
}

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

pub fn async_main_thread<F>(method: F)
where
    F: Fn() + Send + 'static
{
    let queue = dispatch::Queue::main();    
    queue.exec_async(method);
}

pub fn sync_main_thread<F>(method: F)
where
    F: Fn() + Send + 'static
{
    let queue = dispatch::Queue::main();    
    queue.exec_sync(method);    
}

/// Upstream core graphics does not implement Encode for certain things, so we wrap them here -
/// these are only used in reading certain types passed to us from some delegate methods.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CGSize {
    pub width: CGFloat,
    pub height: CGFloat,
}

impl CGSize {
    pub fn new(width: CGFloat, height: CGFloat) -> Self {
        CGSize { width, height }
    }
    
    pub fn zero() -> Self {
        CGSize { width: 0., height: 0. }
    }
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

/// A helper method for ensuring that Cocoa is running in multi-threaded mode.
///
/// Why do we need this? According to Apple, if you're going to make use of standard POSIX threads,
/// you need to, before creating and using a POSIX thread, first create and immediately detach a
/// `NSThread`. This ensures that Cocoa utilizes proper locking in certain places where it might
/// not be doing so for performance reasons.
///
/// In general, you should aim to just start all of your work inside of your `AppDelegate` methods.
/// There are some cases where you might want to do things before that, though - and if you spawn a
/// thread there, just call this first... otherwise you may have some unexpected issues later on.
///
/// _(This is called inside the `App::new()` construct for you already, so as long as you're doing
/// nothing before your `AppDelegate`, you can pay this no mind)._
pub fn activate_cocoa_multithreading() {
    unsafe {
        let thread: id = msg_send![class!(NSThread), new];
        let _: () = msg_send![thread, start];
    }
}
