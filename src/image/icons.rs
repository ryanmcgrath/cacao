
/// These icons are system-provided icons that are guaranteed to exist in all versions of macOS
/// that Cacao supports. These will use SFSymbols on Big Sur and onwards (11.0+), and the correct
/// controls for prior macOS versions.
///
/// Note that this list is by default small, as icons that match across OS's is limited in nature.
/// You'll need to determine if and/or how you choose to support icons for systems older than Big
/// Sur; SFSymbols does not exist on Catalina, Mojave, and earlier.
///
/// You can opt to include vector assets in your bundle, or draw icons with `Image::draw` by
/// converting Core Graphics calls (e.g, PaintCode can work well for this).
#[cfg(feature = "macos")]
#[derive(Debug)]
pub enum MacSystemIcon {
    /// A standard "General" preferences icon. This is intended for usage in Preferences toolbars.
    PreferencesGeneral,

    /// A standard "Advanced" preferences icon. This is intended for usage in Preferences toolbars.
    PreferencesAdvanced,

    /// A standard "Accounts" preferences icon. This is intended for usage in Preferences toolbars.
    PreferencesUserAccounts
}

impl MacSystemIcon {
    /// Maps system icons to their pre-11.0 framework identifiers.
    pub fn to_str(&self) -> &'static str {
        match self {
            MacSystemIcon::PreferencesGeneral => "NSPreferencesGeneral",
            MacSystemIcon::PreferencesAdvanced => "NSAdvanced",
            MacSystemIcon::PreferencesUserAccounts => "NSUserAccounts",
        }
    }

    /// Maps system icons to their SFSymbols-counterparts for use on 11.0+.
    pub fn to_sfsymbol_str(&self) -> &'static str {
         match self {
            MacSystemIcon::PreferencesGeneral => "gearshape",
            MacSystemIcon::PreferencesAdvanced => "slider.vertical.3",
            MacSystemIcon::PreferencesUserAccounts => "person.crop.circle"
        }       
    }
}
