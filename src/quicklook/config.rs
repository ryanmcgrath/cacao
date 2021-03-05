use std::path::Path;

use core_graphics::base::CGFloat;
use objc::runtime::{Object};
use objc::{class, msg_send, sel, sel_impl};
use objc_id::ShareId;

use crate::foundation::{id, YES, NSString, NSUInteger};
use crate::utils::CGSize;

/// Describes the quality of the thumbnail you expect back from the 
/// generator service.
#[derive(Debug)]
pub enum ThumbnailQuality {
    /// Appropriate for a file icon.
    Icon,

    /// Low-ish quality, but fast.
    Low,

    /// Higher quality, but potentially slower.
    High,

    /// Ask for them all, and pick which one you
    /// use via your provided callback.
    All,

    /// Provided in case this is ever expanded by the OS, and the system
    /// returns a thumbnail quality type that can't be matched here. Users
    /// could then handle the edge case themselves.
    Unknown(NSUInteger)
}

impl From<&ThumbnailQuality> for NSUInteger {
    fn from(quality: &ThumbnailQuality) -> Self {
        match quality {
            ThumbnailQuality::Icon => 1 << 0,
            ThumbnailQuality::Low => 1 << 1,
            ThumbnailQuality::High => 1 << 2,
            ThumbnailQuality::All => NSUInteger::MAX,
            ThumbnailQuality::Unknown(x) => *x
        }
    }
}

impl From<ThumbnailQuality> for NSUInteger {
    fn from(quality: ThumbnailQuality) -> Self {
        match quality {
            ThumbnailQuality::Icon => 1 << 0,
            ThumbnailQuality::Low => 1 << 1,
            ThumbnailQuality::High => 1 << 2,
            ThumbnailQuality::All => NSUInteger::MAX,
            ThumbnailQuality::Unknown(x) => x
        }
    }
}

impl From<NSUInteger> for ThumbnailQuality {
    fn from(i: NSUInteger) -> Self {
        match i {
            0 => ThumbnailQuality::Icon,
            2 => ThumbnailQuality::Low,
            4 => ThumbnailQuality::High,
            NSUInteger::MAX => ThumbnailQuality::All,
            i => ThumbnailQuality::Unknown(i)
        }
    }
}

#[derive(Clone, Debug)]
pub struct ThumbnailConfig {
    pub size: (CGFloat, CGFloat),
    pub scale: CGFloat,
    pub minimum_dimension: CGFloat,
    pub icon_mode: bool,
    pub types: &'static [ThumbnailQuality]
}

impl Default for ThumbnailConfig {
    fn default() -> Self {
        ThumbnailConfig {
            size: (44., 44.),
            
            // #TODO: Should query the current screen size maybe? 2x is fairly safe
            // for most moderns Macs right now.
            scale: 2.,
            
            minimum_dimension: 0.,

            icon_mode: false,

            types: &[ThumbnailQuality::All]
        }
    }
}

impl ThumbnailConfig {
    /// Consumes the request and returns a native representation 
    /// (`QLThumbnailGenerationRequest`).
    pub fn to_request(self, path: &Path) -> id {
        let file = NSString::new(path.to_str().unwrap());

        let mut types: NSUInteger = 0;
        for mask in self.types {
            let i: NSUInteger = mask.into();
            types = types | i;
        }

        unsafe {
            let size = CGSize::new(self.size.0, self.size.1);
            // @TODO: Check nil here, or other bad conversion
            let from_url: id = msg_send![class!(NSURL), fileURLWithPath:&*file];

            let request: id = msg_send![class!(QLThumbnailGenerationRequest), alloc];
            let request: id = msg_send![request, initWithFileAtURL:from_url
                size:size
                scale:self.scale
                representationTypes:types];

            if self.icon_mode {
                let _: () = msg_send![request, setIconMode:YES];
            }

            if self.minimum_dimension != 0. {
                let _: () = msg_send![request, setMinimumDimension:self.minimum_dimension];
            }

            request
        }
    }
}
