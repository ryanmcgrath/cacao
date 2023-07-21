use std::marker::PhantomData;
use std::ops::{Deref, DerefMut, Range};
use std::os::raw::c_char;
use std::{fmt, slice, str};

use core_foundation::base::CFRange;

use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};
use objc_id::Id;

use crate::color::Color;
use crate::foundation::{id, to_bool, NSString, Retainable, BOOL, NO, YES};

use super::Font;

extern "C" {
    static NSForegroundColorAttributeName: id;
    static NSFontAttributeName: id;
}

/// A wrapper around `NSMutableAttributedString`, which can be used for more complex text
/// rendering.
///
pub struct AttributedString(pub Id<Object>);

impl AttributedString {
    /// Creates a blank AttributedString. Internally, this allocates an
    /// `NSMutableAttributedString`, which is required for controls to make use of rich text.
    pub fn new(value: &str) -> Self {
        let text = NSString::no_copy(value);

        Self(unsafe {
            let alloc: id = msg_send![class!(NSMutableAttributedString), alloc];
            Id::from_ptr(msg_send![alloc, initWithString:&*text])
        })
    }

    /// Creates a mutableCopy of a passed in `NSAttributedString` instance. This is mostly for
    /// internal use, but kept available as part of the public API for the more adventurous types
    /// who might need it.
    pub fn wrap(value: id) -> Self {
        Self(unsafe { Id::from_ptr(msg_send![value, mutableCopy]) })
    }

    /// Sets the text (foreground) color for the specified range.
    pub fn set_text_color<C: AsRef<Color>>(&mut self, color: C, range: Range<isize>) {
        let color: id = color.as_ref().into();
        let range = CFRange::init(range.start, range.end);

        unsafe {
            let _: () = msg_send![&*self.0, addAttribute:NSForegroundColorAttributeName
                value:color
                range:range
            ];
        }
    }

    /// Set the font for the specified range.
    pub fn set_font(&mut self, font: Font, range: Range<isize>) {
        unsafe {
            let _: () = msg_send![&*self.0, addAttribute:NSFontAttributeName
                value:&*font
                range:range
            ];
        }
    }
}

impl fmt::Display for AttributedString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = NSString::from_retained(unsafe { msg_send![&*self.0, string] });

        write!(f, "{}", string.to_str())
    }
}

impl fmt::Debug for AttributedString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = NSString::from_retained(unsafe { msg_send![&*self.0, string] });

        f.debug_struct("AttributedString").field("text", &string.to_str()).finish()
    }
}

impl From<AttributedString> for id {
    /// Consumes and returns the pointer to the underlying NSMutableAttributedString instance.
    fn from(mut string: AttributedString) -> Self {
        &mut *string.0
    }
}

impl Deref for AttributedString {
    type Target = Object;

    /// Derefs to the underlying Objective-C Object.
    fn deref(&self) -> &Object {
        &*self.0
    }
}

impl DerefMut for AttributedString {
    /// Derefs to the underlying Objective-C Object.
    fn deref_mut(&mut self) -> &mut Object {
        &mut *self.0
    }
}
