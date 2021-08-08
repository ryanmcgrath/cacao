use objc_id::ShareId;
use objc::runtime::Object;

use objc::{class, msg_send, sel, sel_impl};

use block::ConcreteBlock;

use core_graphics::{
    base::{CGFloat},
    geometry::{CGRect, CGPoint, CGSize}
};
use core_graphics::context::{CGContext, CGContextRef};

use crate::foundation::{id, YES, NO, NSString};
use crate::utils::os;
use super::icons::*;

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
    if x == y { return x; }

    match x > y {
        true => x,
        false => y
    }
}

fn min_cgfloat(x: CGFloat, y: CGFloat) -> CGFloat {
    if x == y { return x; }

    match x < y {
        true => x,
        false => y
    }
}

impl ResizeBehavior {
    /// Given a source and target rectangle, configures and returns a new rectangle configured with
    /// the resizing properties of this enum.
    pub fn apply(&self, source: CGRect, target: CGRect) -> CGRect {
        // if equal, just return source
        if
            source.origin.x == target.origin.x && 
            source.origin.y == target.origin.y &&
            source.size.width == target.size.width &&
            source.size.height == target.size.height
        {
            return source;
        }

        if
            source.origin.x == 0. && 
            source.origin.y == 0. &&
            source.size.width == 0. &&
            source.size.height == 0. 
        {
            return source;
        }

        let mut scales = CGSize::new(0., 0.);
        scales.width = (target.size.width / source.size.width).abs();
        scales.height = (target.size.height / source.size.height).abs();

        match self {
            ResizeBehavior::AspectFit => {
                scales.width = min_cgfloat(scales.width, scales.height);
                scales.height = scales.width;
            },

            ResizeBehavior::AspectFill => {
                scales.width = max_cgfloat(scales.width, scales.height);
                scales.height = scales.width;
            },

            ResizeBehavior::Stretch => { /* will do this as default */ },

            ResizeBehavior::Center => {
                scales.width = 1.;
                scales.height = 1.;
            }
        }

        let mut result = source;
        result.size.width *= scales.width;
        result.size.height *= scales.height;
        result.origin.x = target.origin.x + (target.size.width - result.size.width) / 2.;
        result.origin.y = target.origin.y + (target.size.height - result.size.height) / 2.;
        result
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
pub struct Image(pub ShareId<Object>);

impl Image {
    /// Wraps a system-returned image, e.g from QuickLook previews.
    pub fn with(image: id) -> Self {
        Image(unsafe {
            ShareId::from_ptr(image)
        })
    }

    // @TODO: for Airyx, unsure if this is supported - and it's somewhat modern macOS-specific, so
    // let's keep the os flag here for now.
    /// Returns a stock system icon. These are guaranteed to exist across all versions of macOS
    /// supported.
    #[cfg(target_os = "macos")]
    pub fn system_icon(icon: MacSystemIcon, accessibility_description: &str) -> Self {
        Image(unsafe {
            ShareId::from_ptr(match os::is_minimum_version(11) {
                true => {
                    let icon = NSString::new(icon.to_sfsymbol_str());
                    let desc = NSString::new(accessibility_description);
                    msg_send![class!(NSImage), imageWithSystemSymbolName:&*icon
                        accessibilityDescription:&*desc]
                },

                false => {
                    let icon = NSString::new(icon.to_str());
                    msg_send![class!(NSImage), imageNamed:&*icon]
                }
            })
        })
    }

    /// Creates and returns an Image with the specified `SFSymbol`. Note that `SFSymbol` is 
    /// supported on 11.0+; as such, this will panic if called on a lower system. Take care to
    /// provide a fallback image or user experience if you need to support an older OS.
    pub fn symbol(symbol: SFSymbol, accessibility_description: &str) -> Self {
        Image(unsafe {
            ShareId::from_ptr(match os::is_minimum_version(11) {
                true => {
                    let icon = NSString::new(symbol.to_str());
                    let desc = NSString::new(accessibility_description);
                    msg_send![class!(NSImage), imageWithSystemSymbolName:&*icon
                        accessibilityDescription:&*desc]
                },

                false => {
                    #[cfg(target_os = "macos")]
                    panic!("SFSymbols are only supported on macOS 11.0 and up.");
                }
            })
        })
    }

    /// Draw a custom image and get it back as a returned `Image`.
    pub fn draw<F>(config: DrawConfig, handler: F) -> Self
    where
        F: Fn(CGRect, &CGContextRef) -> bool + 'static
    {
        let source_frame = CGRect::new(
             &CGPoint::new(0., 0.),
             &CGSize::new(config.source.0, config.source.1)
        );

        let target_frame = CGRect::new(
             &CGPoint::new(0., 0.),
             &CGSize::new(config.target.0, config.target.1)
        );

        let resized_frame = config.resize.apply(source_frame, target_frame);

        let block = ConcreteBlock::new(move |_destination: CGRect| unsafe {
            let current_context: id = msg_send![class!(NSGraphicsContext), currentContext];
            let context_ptr: core_graphics::sys::CGContextRef = msg_send![current_context, CGContext];
            let context = CGContext::from_existing_context_ptr(context_ptr);
            let _: () = msg_send![class!(NSGraphicsContext), saveGraphicsState];

            context.translate(resized_frame.origin.x, resized_frame.origin.y);
            context.scale(
                resized_frame.size.width / config.source.0,
                resized_frame.size.height / config.source.1
            );

            let result = handler(resized_frame, &context);
            
            let _: () = msg_send![class!(NSGraphicsContext), restoreGraphicsState];

            match result {
                true => YES,
                false => NO
            }
        });
        let block = block.copy();

        Image(unsafe {
            let img: id = msg_send![class!(NSImage), imageWithSize:target_frame.size 
                flipped:YES 
                drawingHandler:block
            ];

            ShareId::from_ptr(img)
        })
    }
}
