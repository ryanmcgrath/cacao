use crate::foundation::id;

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
#[cfg(target_os = "macos")]
#[derive(Debug)]
pub enum MacSystemIcon {
    /// A standard "General" preferences icon. This is intended for usage in Preferences toolbars.
    PreferencesGeneral,

    /// A standard "Advanced" preferences icon. This is intended for usage in Preferences toolbars.
    PreferencesAdvanced,

    /// A standard "Accounts" preferences icon. This is intended for usage in Preferences toolbars.
    PreferencesUserAccounts,

    /// Returns a stock "+" icon that's common to the system. Use this for buttons that need the
    /// symbol.
    Add,

    /// A stock "-" icon that's common to the system. Use this for buttons that need the symbol.
    Remove,

    /// Returns a Folder icon.
    Folder
}

extern "C" {
    static NSImageNamePreferencesGeneral: id;
    static NSImageNameAdvanced: id;
    static NSImageNameUserAccounts: id;
    static NSImageNameAddTemplate: id;
    static NSImageNameFolder: id;
    static NSImageNameRemoveTemplate: id;
}

#[cfg(target_os = "macos")]
impl MacSystemIcon {
    /// Maps system icons to their pre-11.0 framework identifiers.
    pub fn to_id(&self) -> id {
        unsafe {
            match self {
                MacSystemIcon::PreferencesGeneral => NSImageNamePreferencesGeneral,
                MacSystemIcon::PreferencesAdvanced => NSImageNameAdvanced,
                MacSystemIcon::PreferencesUserAccounts => NSImageNameUserAccounts,
                MacSystemIcon::Add => NSImageNameAddTemplate,
                MacSystemIcon::Remove => NSImageNameRemoveTemplate,
                MacSystemIcon::Folder => NSImageNameFolder
            }
        }
    }

    /// Maps system icons to their SFSymbols-counterparts for use on 11.0+.
    pub fn to_sfsymbol_str(&self) -> &'static str {
         match self {
            MacSystemIcon::PreferencesGeneral => SFSymbol::GearShape.to_str(),
            MacSystemIcon::PreferencesAdvanced => SFSymbol::SliderVertical3.to_str(),
            MacSystemIcon::PreferencesUserAccounts => SFSymbol::AtSymbol.to_str(),
            MacSystemIcon::Add => SFSymbol::Plus.to_str(),
            MacSystemIcon::Remove => SFSymbol::Minus.to_str(),
            MacSystemIcon::Folder => SFSymbol::FolderFilled.to_str()
        }       
    }
}

#[derive(Debug)]
pub enum SFSymbol {
    AtSymbol,
    GearShape,
    FolderFilled,
    PaperPlane,
    PaperPlaneFilled,
    Plus,
    Minus,
    SliderVertical3,
    SquareAndArrowUpOnSquare,
    SquareAndArrowUpOnSquareFill,
    SquareAndArrowDownOnSquare,
    SquareAndArrowDownOnSquareFill,
    SquareDashed
}

impl SFSymbol {
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::AtSymbol => "at",
            Self::GearShape => "gearshape",
            Self::FolderFilled => "folder.fill",
            Self::PaperPlane => "paperplane",
            Self::PaperPlaneFilled => "paperplane.fill",
            Self::Plus => "plus",
            Self::Minus => "minus",
            Self::SliderVertical3 => "slider.vertical.3",
            Self::SquareAndArrowUpOnSquare => "square.and.arrow.up.on.square",
            Self::SquareAndArrowUpOnSquareFill => "square.and.arrow.up.on.square.fill",
            Self::SquareAndArrowDownOnSquare => "square.and.arrow.down.on.square",
            Self::SquareAndArrowDownOnSquareFill => "square.and.arrow.down.on.square.fill",
            Self::SquareDashed => "square.dashed"
        }
    }
}
