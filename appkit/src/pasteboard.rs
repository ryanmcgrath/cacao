//! This module provides some basic wrappers for PasteBoard functionality. It's currently not an
//! exhaustive clone, but feel free to pull request accordingly!

use cocoa::base::{id, nil};
use cocoa::foundation::NSString;

/// Represents different PasteBoard types that can be referred to.
#[derive(Debug, Copy, Clone)]
pub enum PasteBoardType {
    /// URL data for one file or resource.
    URL,

    /// Color data.
    Color,

    /// A file URL.
    FileURL,

    /// Font and character information.
    Font,

    /// Type for HTML content.
    HTML,

    /// Multiple text selection.
    MultipleTextSelection,

    /// PDF data.
    PDF,

    /// PNG image data.
    PNG,

    /// Rich Text Format (RTF) data.
    RTF,

    /// RTFD formatted file contents.
    RTFD,

    /// Paragraph formatting information.
    Ruler,

    /// Sound data.
    Sound,

    /// String data.
    String,

    /// Tab-separated fields of text.
    TabularText,

    /// Tag Image File Format (TIFF) data.
    TIFF
}

impl PasteBoardType {
    /// Creates an `NSString` out of the underlying type.
    pub fn to_nsstring(&self) -> id {
        unsafe {
            NSString::alloc(nil).init_str(match self {
                PasteBoardType::URL => "public.url",
                PasteBoardType::Color => "com.apple.cocoa.pasteboard.color",
                PasteBoardType::FileURL => "public.file-url",
                PasteBoardType::Font => "com.apple.cocoa.pasteboard.character-formatting",
                PasteBoardType::HTML => "public.html",
                PasteBoardType::MultipleTextSelection => "com.apple.cocoa.pasteboard.multiple-text-selection",
                PasteBoardType::PDF => "com.adobe.pdf",
                PasteBoardType::PNG => "public.png",
                PasteBoardType::RTF => "public.rtf",
                PasteBoardType::RTFD => "com.apple.flat-rtfd",
                PasteBoardType::Ruler => "com.apple.cocoa.pasteboard.paragraph-formatting",
                PasteBoardType::Sound => "com.apple.cocoa.pasteboard.sound",
                PasteBoardType::String => "public.utf8-plain-text",
                PasteBoardType::TabularText => "public.utf8-tab-separated-values-text",
                PasteBoardType::TIFF => "public.tiff",
            })
        }
    }
}
