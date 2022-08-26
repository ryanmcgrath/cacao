//! This provides some basic mapping for providing Key characters to controls. It's mostly meant as
//! a wrapper to stop magic symbols all over the place.

/// Represents a Key character.
#[derive(Debug)]
pub enum Key<'a> {
    /// Behind the scenes, this translates to NSDeleteCharacter (for AppKit).
    Delete,

    /// Whatever character you want.
    Char(&'a str)
}

impl<'a> From<&'a str> for Key<'a> {
    fn from(s: &'a str) -> Self {
        Key::Char(s)
    }
}
