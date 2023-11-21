use std::ffi::c_void;

use objc::foundation::{CGFloat, NSPoint, NSRect, NSSize};
use objc::rc::{Id, Shared};
use objc::runtime::{Bool, Class, Object};
use objc::{class, msg_send, msg_send_id, sel};

use block::ConcreteBlock;

use super::graphics_context::GraphicsContext;
use super::icons::*;
use crate::foundation::{id, NSData, NSString, NSURL};
use crate::geometry::Rect;
use crate::utils::os;

/// Specifies resizing behavior for image drawing.
#[derive(Copy, Clone, Debug)]
pub enum ResizeBehavior {
    /// Fit to the aspect ratio.
    AspectFit,

    /// Fill the aspect ratio.
    AspectFill,

    /// Stretch as necessary.
    Stretch,

    /// Center and then let whatever else flow around it.
    Center
}

fn max_cgfloat(x: CGFloat, y: CGFloat) -> CGFloat {
    if x == y {
        return x;
    }

    match x > y {
        true => x,
        false => y
    }
}

fn min_cgfloat(x: CGFloat, y: CGFloat) -> CGFloat {
    if x == y {
        return x;
    }

    match x < y {
        true => x,
        false => y
    }
}

impl ResizeBehavior {
    /// Given a source and target rectangle, configures and returns a new rectangle configured with
    /// the resizing properties of this enum.
    pub fn apply(&self, source: NSRect, target: NSRect) -> NSRect {
        // if equal, just return source
        if source == target {
            return source;
        }

        if source == NSRect::ZERO {
            return source;
        }

        let mut scale_width = (target.size.width() / source.size.width()).abs();
        let mut scale_height = (target.size.height() / source.size.height()).abs();

        match self {
            ResizeBehavior::AspectFit => {
                scale_width = min_cgfloat(scale_width, scale_height);
                scale_height = scale_width;
            },

            ResizeBehavior::AspectFill => {
                scale_width = max_cgfloat(scale_width, scale_height);
                scale_height = scale_width;
            },

            ResizeBehavior::Stretch => { /* will do this as default */ },

            ResizeBehavior::Center => {
                scale_width = 1.;
                scale_height = 1.;
            }
        }

        let result_size = NSSize::new(source.size.width() * scale_width, source.size.height() * scale_height);

        NSRect::new(
            NSPoint::new(
                target.origin.x + (target.size.width() - result_size.width()) / 2.,
                target.origin.y + (target.size.height() - result_size.height()) / 2.
            ),
            result_size
        )
    }
}

/// A config object that specifies how drawing into an image context should scale.
#[derive(Copy, Clone, Debug)]
pub struct DrawConfig {
    /// The size of the source.
    pub source: (CGFloat, CGFloat),

    /// The size of the target. This may be the same as the source; if not, the source will be
    /// scaled to this size.
    pub target: (CGFloat, CGFloat),

    /// The type of resizing to use during drawing and scaling.
    pub resize: ResizeBehavior
}

/// Wraps `NSImage` under AppKit, and `UIImage` on under UIKit (iOS and tvOS). Can be used to display images, icons,
/// and so on.
#[derive(Clone, Debug)]
pub struct Image(pub Id<Object, Shared>);

impl Image {
    fn class() -> &'static Class {
        #[cfg(feature = "appkit")]
        let class = class!(NSImage);
        #[cfg(all(feature = "uikit", not(feature = "appkit")))]
        let class = class!(UIImage);

        class
    }

    /// Wraps a system-returned image, e.g from QuickLook previews.
    pub fn with(image: id) -> Self {
        Image(unsafe { Id::retain(image).unwrap() })
    }

    /// Loads an image from the specified path.
    pub fn with_contents_of_file(path: &str) -> Self {
        let file_path = NSString::new(path);

        Image(unsafe {
            let alloc = msg_send_id![Self::class(), alloc];
            msg_send_id![alloc, initWithContentsOfFile: &*file_path]
        })
    }

    #[cfg(target_os = "macos")]
    pub fn with_contents_of_url(url: NSURL) -> Self {
        Image(unsafe {
            let alloc = msg_send_id![Self::class(), alloc];
            msg_send_id![alloc, initWithContentsOfURL: &*url.objc]
        })
    }

    /// Given a Vec of data, will transform it into an Image by passing it through NSData.
    /// This can be useful for when you need to include_bytes!() something into your binary.
    pub fn with_data(data: &[u8]) -> Self {
        let data = NSData::with_slice(data);

        Image(unsafe {
            let alloc = msg_send_id![Self::class(), alloc];
            msg_send_id![alloc, initWithData: &*data]
        })
    }

    // @TODO: for Airyx, unsure if this is supported - and it's somewhat modern macOS-specific, so
    // let's keep the os flag here for now.
    /// Returns a stock system icon. These are guaranteed to exist across all versions of macOS
    /// supported.
    #[cfg(target_os = "macos")]
    pub fn system_icon(icon: MacSystemIcon) -> Self {
        Image(unsafe {
            let icon = icon.to_id();
            msg_send_id![Self::class(), imageNamed: icon]
        })
    }

    // @TODO: for Airyx, unsure if this is supported - and it's somewhat modern macOS-specific, so
    // let's keep the os flag here for now.
    /// The name here can be confusing, I know.
    ///
    /// A system symbol will swap an SFSymbol in for macOS 11.0+, but return the correct
    /// MacSystemIcon image type for versions prior to that. This is mostly helpful in situations
    /// like Preferences windows toolbars, where you want to have the correct modern styling for newer OS
    /// versions.
    ///
    /// However, if you need the correct "folder" icon for instance, you probably want `system_icon`.
    #[cfg(target_os = "macos")]
    pub fn toolbar_icon(icon: MacSystemIcon, accessibility_description: &str) -> Self {
        Image(unsafe {
            match os::is_minimum_version(11) {
                true => {
                    let icon = NSString::new(icon.to_sfsymbol_str());
                    let desc = NSString::new(accessibility_description);
                    msg_send_id![
                        Self::class(),
                        imageWithSystemSymbolName: &*icon,
                        accessibilityDescription: &*desc,
                    ]
                },

                false => {
                    let icon = icon.to_id();
                    msg_send_id![Self::class(), imageNamed: icon]
                }
            }
        })
    }

    /// Creates and returns an Image with the specified `SFSymbol`. Note that `SFSymbol` is
    /// supported on macOS 11.0+ and iOS 13.0+; as such, this will panic if called on a
    /// lower system. Take care to provide a fallback image or user experience if you
    /// need to support an older OS.
    ///
    /// This is `target_os` gated as SFSymbols is fairly Apple-specific. If another runtime
    /// ever exposes a compatible API, this can be tweaked in a PR.
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    pub fn symbol(symbol: SFSymbol, accessibility_description: &str) -> Self {
        // SFSymbols is macOS 11.0+
        #[cfg(feature = "appkit")]
        let min_version = 11;

        // SFSymbols is iOS 13.0+.
        #[cfg(all(feature = "uikit", not(feature = "appkit")))]
        let min_version = 13;

        Image(unsafe {
            match os::is_minimum_version(min_version) {
                true => {
                    let icon = NSString::new(symbol.to_str());
                    let desc = NSString::new(accessibility_description);
                    msg_send_id![
                        Self::class(),
                        imageWithSystemSymbolName:&*icon,
                        accessibilityDescription:&*desc,
                    ]
                },

                false => {
                    #[cfg(feature = "appkit")]
                    panic!("SFSymbols are only supported on macOS 11.0 and up.");

                    #[cfg(all(feature = "uikit", not(feature = "appkit")))]
                    panic!("SFSymbols are only supported on macOS 11.0 and up.");
                }
            }
        })
    }

    /// Draw a custom image and get it back as a returned `Image`.
    ///
    /// This is currently only supported on AppKit-based backends, and has
    /// only been tested on macOS.
    #[cfg(feature = "appkit")]
    pub fn draw<F>(config: DrawConfig, handler: F) -> Self
    where
        F: Fn(Rect, (f64, f64), *mut c_void) -> bool + 'static
    {
        let source_frame = NSRect::new(NSPoint::new(0., 0.), NSSize::new(config.source.0, config.source.1));

        let target_frame = NSRect::new(NSPoint::new(0., 0.), NSSize::new(config.target.0, config.target.1));

        let resized_frame: Rect = config.resize.apply(source_frame, target_frame).into();

        let block = ConcreteBlock::new(move |_destination: NSRect| {
            let context = GraphicsContext::current();
            context.save();

            let cg_context_ptr: *mut c_void = context.cg_context();

            // TODO: Automatically scale for the user
            // cg_context_ptr.translate(resized_frame.origin.x, resized_frame.origin.y);
            // cg_context_ptr.scale(
            //     resized_frame.size.width() / config.source.0,
            //     resized_frame.size.height() / config.source.1
            // );

            let result = handler(
                resized_frame,
                (config.source.0 as f64, config.source.1 as f64),
                cg_context_ptr
            );

            context.restore();

            Bool::new(result)
        });
        let block = block.copy();

        Image(unsafe {
            msg_send_id![
                Self::class(),
                imageWithSize: target_frame.size,
                flipped: Bool::YES,
                drawingHandler: &*block,
            ]
        })
    }
}

#[test]
fn test_image_from_bytes() {
    let image_bytes = include_bytes!("../../test-data/favicon.ico");
    let image = Image::with_data(image_bytes);
}
// It's unclear where the file is on the ios simulator.
#[test]
#[cfg(target_os = "macos")]
fn test_image_from_file() {
    let image = Image::with_contents_of_file("./test-data/favicon.ico");
}
