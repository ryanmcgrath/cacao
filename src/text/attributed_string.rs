use std::marker::PhantomData;
use std::ops::{Deref, DerefMut, Range};
use std::os::raw::c_char;
use std::{fmt, slice, str};

use objc::rc::{Id, Owned};
use objc::runtime::Object;
use objc::{class, msg_send, msg_send_id, sel};

use crate::color::Color;
use crate::foundation::{id, to_bool, NSString, BOOL, NO, YES};
use crate::utils::CFRange;

use super::Font;

extern "C" {
    static NSForegroundColorAttributeName: id;
    static NSFontAttributeName: id;
}

/// A wrapper around `NSMutableAttributedString`, which can be used for more complex text
/// rendering.
///
pub struct AttributedString(pub Id<Object, Owned>);

impl AttributedString {
    /// Creates a blank AttributedString. Internally, this allocates an
    /// `NSMutableAttributedString`, which is required for controls to make use of rich text.
    pub fn new(value: &str) -> Self {
        let text = NSString::no_copy(value);

        Self(unsafe {
            let alloc = msg_send_id![class!(NSMutableAttributedString), alloc];
            msg_send_id![alloc, initWithString:&*text]
        })
    }

    /// Creates a mutableCopy of a passed in `NSAttributedString` instance. This is mostly for
    /// internal use, but kept available as part of the public API for the more adventurous types
    /// who might need it.
    pub fn wrap(value: id) -> Self {
        Self(unsafe { msg_send_id![value, mutableCopy] })
    }

    /// Sets the text (foreground) color for the specified range.
    pub fn set_text_color<C: AsRef<Color>>(&mut self, color: C, range: Range<isize>) {
        let color: id = color.as_ref().into();
        let range = CFRange::init(range.start, range.end);

        unsafe {
            let _: () = msg_send![
                &*self.0,
                addAttribute: NSForegroundColorAttributeName,
                value: color,
                range: range,
            ];
        }
    }

    /// Set the font for the specified range.
    pub fn set_font(&mut self, font: Font, range: Range<isize>) {
        let range = CFRange::init(range.start, range.end);

        unsafe {
            let _: () = msg_send![
                &*self.0,
                addAttribute: NSFontAttributeName,
                value: &*font,
                range: range,
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
