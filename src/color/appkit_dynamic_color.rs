//! This module provides a custom NSColor subclass for macOS that mimics the dynamic
//! UIColor provider found on iOS. Notably, this works with older versions of macOS as
//! well; it runs the block on creation and caches the created color instances to avoid
//! repeated allocations - this might not be a big thing to worry about as NSColor
//! changed slightly behind the scenes in 10.15+, so this could be changed down the
//! road.
//!
//! On versions where dark mode is not supported (e.g, pre-Mojave) this will return the
//! provided light color. Note that while 10.15 did introduce an `NSColor` initializer
//! that enables this functionality, we want to be able to provide this with some level of
//! backwards compatibility for Mojave, as that's still a supported OS.

use core_graphics::base::CGFloat;
use objc::runtime::{Class, Object, Sel};
use objc::{class, msg_send, sel};

use crate::foundation::{id, load_or_register_class, nil, NSArray, NSInteger};
use crate::utils::os;

pub(crate) const AQUA_LIGHT_COLOR_NORMAL_CONTRAST: &'static str = "AQUA_LIGHT_COLOR_NORMAL_CONTRAST";
pub(crate) const AQUA_LIGHT_COLOR_HIGH_CONTRAST: &'static str = "AQUA_LIGHT_COLOR_HIGH_CONTRAST";
pub(crate) const AQUA_DARK_COLOR_NORMAL_CONTRAST: &'static str = "AQUA_DARK_COLOR_NORMAL_CONTRAST";
pub(crate) const AQUA_DARK_COLOR_HIGH_CONTRAST: &'static str = "AQUA_DARK_COLOR_HIGH_CONTRAST";

// Certain platforms we're interested in supporting (airyx) might not have these yet, so this
// will just force the Aqua appearance on that platform.
#[cfg(target_os = "macos")]
extern "C" {
    static NSAppearanceNameAqua: id;
    static NSAppearanceNameAccessibilityHighContrastAqua: id;
    static NSAppearanceNameDarkAqua: id;
    static NSAppearanceNameAccessibilityHighContrastDarkAqua: id;
}

/// This function accepts an `Object` (our `CacaoDynamicColor` instance) and queries the system
/// to determine which color should be used. Note that this currently does not support high
/// contrast checking on systems prior to 10.14: it's not that it couldn't be supported, but the
/// ongoing question of how far back to support makes this not worth bothering with right now.
///
/// On non-Apple systems, this returns the light aqua color at all times.
///
/// Pull requests to implement that check would be welcome.
fn get_effective_color(this: &Object) -> id {
    #[cfg(target_os = "macos")]
    if os::is_minimum_semversion(10, 14, 0) {
        unsafe {
            let mut appearance: id = msg_send![class!(NSAppearance), currentAppearance];
            if appearance == nil {
                appearance = msg_send![class!(NSApp), effectiveAppearance];
            }

            let names = NSArray::new(&[
                NSAppearanceNameAqua,
                NSAppearanceNameAccessibilityHighContrastAqua,
                NSAppearanceNameDarkAqua,
                NSAppearanceNameAccessibilityHighContrastDarkAqua
            ]);

            let style: id = msg_send![appearance, bestMatchFromAppearancesWithNames:&*names];

            if style == NSAppearanceNameDarkAqua {
                return *this.get_ivar(AQUA_DARK_COLOR_NORMAL_CONTRAST);
            }

            if style == NSAppearanceNameAccessibilityHighContrastAqua {
                return *this.get_ivar(AQUA_LIGHT_COLOR_HIGH_CONTRAST);
            }

            if style == NSAppearanceNameAccessibilityHighContrastDarkAqua {
                return *this.get_ivar(AQUA_DARK_COLOR_HIGH_CONTRAST);
            }
        }
    }

    unsafe {
        return *this.get_ivar(AQUA_LIGHT_COLOR_NORMAL_CONTRAST);
    }
}

extern "C" fn color_space(this: &Object, _: Sel) -> id {
    let color = get_effective_color(this);
    unsafe { msg_send![color, colorSpace] }
}

extern "C" fn color_using_color_space(this: &Object, _: Sel, color_space: id) -> id {
    let color = get_effective_color(this);
    unsafe { msg_send![color, colorUsingColorSpace: color_space] }
}

extern "C" fn color_space_name(this: &Object, _: Sel) -> id {
    let color = get_effective_color(this);
    unsafe { msg_send![color, colorSpaceName] }
}

extern "C" fn color_using_color_space_name(this: &Object, _: Sel, color_space_name: id) -> id {
    let color = get_effective_color(this);
    unsafe { msg_send![color, colorUsingColorSpaceName: color_space_name] }
}

extern "C" fn number_of_components(this: &Object, _: Sel) -> NSInteger {
    let color = get_effective_color(this);
    unsafe { msg_send![color, numberOfComponents] }
}

// @TODO: Confirm this.
extern "C" fn get_components(this: &Object, _: Sel, components: CGFloat) {
    let color = get_effective_color(this);
    unsafe {
        let _: () = msg_send![color, getComponents: components];
    }
}

// @TODO: Confirm this.
extern "C" fn get_rgba(this: &Object, _: Sel, red: CGFloat, green: CGFloat, blue: CGFloat, alpha: CGFloat) {
    let color = get_effective_color(this);
    unsafe {
        let _: () = msg_send![color, getRed:red green:green blue:blue alpha:alpha];
    }
}

extern "C" fn red_component(this: &Object, _: Sel) -> CGFloat {
    let color = get_effective_color(this);
    unsafe { msg_send![color, redComponent] }
}

extern "C" fn green_component(this: &Object, _: Sel) -> CGFloat {
    let color = get_effective_color(this);
    unsafe { msg_send![color, greenComponent] }
}

extern "C" fn blue_component(this: &Object, _: Sel) -> CGFloat {
    let color = get_effective_color(this);
    unsafe { msg_send![color, blueComponent] }
}

extern "C" fn hue_component(this: &Object, _: Sel) -> CGFloat {
    let color = get_effective_color(this);
    unsafe { msg_send![color, hueComponent] }
}

extern "C" fn saturation_component(this: &Object, _: Sel) -> CGFloat {
    let color = get_effective_color(this);
    unsafe { msg_send![color, saturationComponent] }
}

extern "C" fn brightness_component(this: &Object, _: Sel) -> CGFloat {
    let color = get_effective_color(this);
    unsafe { msg_send![color, brightnessComponent] }
}

// @TODO: Confirm this.
extern "C" fn get_hsba(this: &Object, _: Sel, hue: CGFloat, sat: CGFloat, brit: CGFloat, alpha: CGFloat) {
    let color = get_effective_color(this);
    unsafe {
        let _: () = msg_send![color, getHue:hue saturation:sat brightness:brit alpha:alpha];
    }
}

extern "C" fn white_component(this: &Object, _: Sel) -> CGFloat {
    let color = get_effective_color(this);
    unsafe { msg_send![color, whiteComponent] }
}

// @TODO: Confirm this.
extern "C" fn get_white(this: &Object, _: Sel, white: CGFloat, alpha: CGFloat) {
    let color = get_effective_color(this);
    unsafe {
        let _: () = msg_send![color, getWhite:white alpha:alpha];
    }
}

extern "C" fn cyan_component(this: &Object, _: Sel) -> CGFloat {
    let color = get_effective_color(this);
    unsafe { msg_send![color, cyanComponent] }
}

extern "C" fn magenta_component(this: &Object, _: Sel) -> CGFloat {
    let color = get_effective_color(this);
    unsafe { msg_send![color, magentaComponent] }
}

extern "C" fn yellow_component(this: &Object, _: Sel) -> CGFloat {
    let color = get_effective_color(this);
    unsafe { msg_send![color, yellowComponent] }
}

extern "C" fn black_component(this: &Object, _: Sel) -> CGFloat {
    let color = get_effective_color(this);
    unsafe { msg_send![color, blackComponent] }
}

// @TODO: Confirm this.
extern "C" fn get_cmyk(this: &Object, _: Sel, c: CGFloat, m: CGFloat, y: CGFloat, k: CGFloat, a: CGFloat) {
    let color = get_effective_color(this);
    unsafe {
        let _: () = msg_send![color, getCyan:c magenta:m yellow:y black:k alpha:a];
    }
}

extern "C" fn alpha_component(this: &Object, _: Sel) -> CGFloat {
    let color = get_effective_color(this);
    unsafe { msg_send![color, alphaComponent] }
}

extern "C" fn cg_color(this: &Object, _: Sel) -> id {
    let color = get_effective_color(this);
    unsafe { msg_send![color, CGColor] }
}

extern "C" fn set_stroke(this: &Object, _: Sel) {
    let color = get_effective_color(this);
    unsafe {
        let _: () = msg_send![color, setStroke];
    }
}

extern "C" fn set_fill(this: &Object, _: Sel) {
    let color = get_effective_color(this);
    unsafe {
        let _: () = msg_send![color, setFill];
    }
}

extern "C" fn call_set(this: &Object, _: Sel) {
    let color = get_effective_color(this);
    unsafe {
        let _: () = msg_send![color, set];
    }
}

extern "C" fn highlight_with_level(this: &Object, _: Sel, level: CGFloat) -> id {
    let color = get_effective_color(this);
    unsafe { msg_send![color, highlightWithLevel: level] }
}

extern "C" fn shadow_with_level(this: &Object, _: Sel, level: CGFloat) -> id {
    let color = get_effective_color(this);
    unsafe { msg_send![color, shadowWithLevel: level] }
}

extern "C" fn color_with_alpha_component(this: &Object, _: Sel, alpha: CGFloat) -> id {
    let color = get_effective_color(this);
    unsafe { msg_send![color, colorWithAlphaComponent: alpha] }
}

extern "C" fn blended_color(this: &Object, _: Sel, fraction: CGFloat, with_color: id) -> id {
    let color = get_effective_color(this);
    unsafe { msg_send![color, blendedColorWithFraction:fraction ofColor:with_color] }
}

extern "C" fn color_with_system_effect(this: &Object, _: Sel, effect: NSInteger) -> id {
    let color = get_effective_color(this);
    unsafe { msg_send![color, colorWithSystemEffect: effect] }
}

pub(crate) fn register_class() -> *const Class {
    load_or_register_class("NSColor", "CacaoDynamicColor", |decl| unsafe {
        // These methods all need to be forwarded, so let's hook them up.
        decl.add_method(sel!(colorSpace), color_space as extern "C" fn(&Object, _) -> id);
        decl.add_method(
            sel!(colorUsingColorSpace:),
            color_using_color_space as extern "C" fn(&Object, _, id) -> id
        );
        decl.add_method(sel!(colorSpaceName), color_space_name as extern "C" fn(&Object, _) -> id);
        decl.add_method(
            sel!(colorUsingColorSpaceName:),
            color_using_color_space_name as extern "C" fn(&Object, _, id) -> id
        );
        decl.add_method(
            sel!(numberOfComponents),
            number_of_components as extern "C" fn(&Object, _) -> NSInteger
        );

        decl.add_method(sel!(getComponents:), get_components as extern "C" fn(&Object, _, CGFloat));
        decl.add_method(
            sel!(getRed:green:blue:alpha:),
            get_rgba as extern "C" fn(&Object, _, CGFloat, CGFloat, CGFloat, CGFloat)
        );
        decl.add_method(sel!(redComponent), red_component as extern "C" fn(&Object, _) -> CGFloat);
        decl.add_method(sel!(greenComponent), green_component as extern "C" fn(&Object, _) -> CGFloat);
        decl.add_method(sel!(blueComponent), blue_component as extern "C" fn(&Object, _) -> CGFloat);

        decl.add_method(sel!(hueComponent), hue_component as extern "C" fn(&Object, _) -> CGFloat);
        decl.add_method(
            sel!(saturationComponent),
            saturation_component as extern "C" fn(&Object, _) -> CGFloat
        );
        decl.add_method(
            sel!(brightnessComponent),
            brightness_component as extern "C" fn(&Object, _) -> CGFloat
        );
        decl.add_method(
            sel!(getHue:saturation:brightness:alpha:),
            get_hsba as extern "C" fn(&Object, _, CGFloat, CGFloat, CGFloat, CGFloat)
        );

        decl.add_method(sel!(whiteComponent), white_component as extern "C" fn(&Object, _) -> CGFloat);
        decl.add_method(
            sel!(getWhite:alpha:),
            get_white as extern "C" fn(&Object, _, CGFloat, CGFloat)
        );

        decl.add_method(sel!(cyanComponent), cyan_component as extern "C" fn(&Object, _) -> CGFloat);
        decl.add_method(
            sel!(magentaComponent),
            magenta_component as extern "C" fn(&Object, _) -> CGFloat
        );
        decl.add_method(
            sel!(yellowComponent),
            yellow_component as extern "C" fn(&Object, _) -> CGFloat
        );
        decl.add_method(sel!(blackComponent), black_component as extern "C" fn(&Object, _) -> CGFloat);
        decl.add_method(
            sel!(getCyan:magenta:yellow:black:alpha:),
            get_cmyk as extern "C" fn(&Object, _, CGFloat, CGFloat, CGFloat, CGFloat, CGFloat)
        );

        decl.add_method(sel!(alphaComponent), alpha_component as extern "C" fn(&Object, _) -> CGFloat);

        decl.add_method(sel!(CGColor), cg_color as extern "C" fn(&Object, _) -> id);
        decl.add_method(sel!(setStroke), set_stroke as extern "C" fn(&Object, _));
        decl.add_method(sel!(setFill), set_fill as extern "C" fn(&Object, _));
        decl.add_method(sel!(set), call_set as extern "C" fn(&Object, _));

        decl.add_method(
            sel!(highlightWithLevel:),
            highlight_with_level as extern "C" fn(&Object, _, CGFloat) -> id
        );
        decl.add_method(
            sel!(shadowWithLevel:),
            shadow_with_level as extern "C" fn(&Object, _, CGFloat) -> id
        );

        decl.add_method(
            sel!(colorWithAlphaComponent:),
            color_with_alpha_component as extern "C" fn(&Object, _, CGFloat) -> id
        );
        decl.add_method(
            sel!(blendedColorWithFraction:ofColor:),
            blended_color as extern "C" fn(&Object, _, CGFloat, id) -> id
        );
        decl.add_method(
            sel!(colorWithSystemEffect:),
            color_with_system_effect as extern "C" fn(&Object, _, NSInteger) -> id
        );

        decl.add_ivar::<id>(AQUA_LIGHT_COLOR_NORMAL_CONTRAST);
        decl.add_ivar::<id>(AQUA_LIGHT_COLOR_HIGH_CONTRAST);
        decl.add_ivar::<id>(AQUA_DARK_COLOR_NORMAL_CONTRAST);
        decl.add_ivar::<id>(AQUA_DARK_COLOR_HIGH_CONTRAST);
    })
}
