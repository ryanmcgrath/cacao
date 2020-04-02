//! iOS-specific implementations.
//!
//! In general, this framework tries to make things "just work" with regards to AppKit and UIKit
//! differences. With that said, there are certain things that just don't map between the two - for
//! iOS, these things are contained here.

mod app;
pub use app::*;
