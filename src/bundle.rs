//! Implements some functionality to handle dynamically setting the `NSBundle` identifier.
//!
//!
//
// This is not currently in use, but does have places where it's useful... and to be honest I'm
// kinda happy this is done as a swizzling implementation in pure Rust, which I couldn't find
// examples of anywhere else.
//
// Disregard until you can't, I guess.

use std::ffi::CString;
use std::mem;

use objc::ffi;
use objc::runtime::{Class, Imp, Object, Sel};
use objc::{class, msg_send, sel, Encode, EncodeArguments, Encoding, Message};

use crate::foundation::{id, nil, BOOL, YES, NSString};

/// Types that can be used as the implementation of an Objective-C method.
pub trait MethodImplementation {
    /// The callee type of the method.
    type Callee: Message;
    /// The return type of the method.
    type Ret: Encode;
    /// The argument types of the method.
    type Args: EncodeArguments;

    /// Returns self as an `Imp` of a method.
    fn imp(self) -> Imp;
}

macro_rules! method_decl_impl {
    (-$s:ident, $r:ident, $f:ty, $($t:ident),*) => (
        impl<$s, $r $(, $t)*> MethodImplementation for $f
                where $s: Message, $r: Encode $(, $t: Encode)* {
            type Callee = $s;
            type Ret = $r;
            type Args = ($($t,)*);

            fn imp(self) -> Imp {
                unsafe { mem::transmute(self) }
            }
        }
    );
    ($($t:ident),*) => (
        method_decl_impl!(-T, R, extern "C" fn(&T, Sel $(, $t)*) -> R, $($t),*);
        method_decl_impl!(-T, R, extern "C" fn(&mut T, Sel $(, $t)*) -> R, $($t),*);
    );
}

method_decl_impl!();
method_decl_impl!(A);

extern "C" fn get_bundle_id(this: &Object, s: Sel, v: id) -> id {
    unsafe {
        let bundle = class!(NSBundle);
        let main_bundle: id = msg_send![bundle, mainBundle];
        let e: BOOL = msg_send![this, isEqual:main_bundle];
        if e == YES {
            let url: id = msg_send![main_bundle, bundleURL];
            let x: id = msg_send![url, absoluteString];
            println!("Got here? {:?}", x);
            NSString::new("com.test.user_notifications").into()
        } else {
            msg_send![this, __bundleIdentifier]
        }
    }
}

unsafe fn swizzle_bundle_id<F>(bundle_id: &str, func: F)
where
    F: MethodImplementation<Callee = Object>,
{
    let name = CString::new("NSBundle").unwrap();
    let cls = ffi::objc_getClass(name.as_ptr());

    // let mut cls = class!(NSBundle) as *mut Class;
    // Class::get("NSBundle").unwrap();
    // let types = format!("{}{}{}", Encoding::String, <*mut Object>::ENCODING, Sel::ENCODING);

    let added = ffi::class_addMethod(
        cls as *mut ffi::objc_class,
        sel!(__bundleIdentifier).as_ptr(),
        func.imp(),
        CString::new("*@:").unwrap().as_ptr(),
    );

    let method1 = ffi::class_getInstanceMethod(cls, sel!(bundleIdentifier).as_ptr()) as *mut ffi::objc_method;
    let method2 = ffi::class_getInstanceMethod(cls, sel!(__bundleIdentifier).as_ptr()) as *mut ffi::objc_method;
    ffi::method_exchangeImplementations(method1, method2);
}

pub fn set_bundle_id(bundle_id: &str) {
    unsafe {
        swizzle_bundle_id(bundle_id, get_bundle_id as extern "C" fn(&Object, _, _) -> id);
    }
}
