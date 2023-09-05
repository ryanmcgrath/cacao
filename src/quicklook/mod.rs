use std::path::Path;

use objc::rc::{Id, Shared};
use objc::runtime::Object;
use objc::{class, msg_send, msg_send_id, sel};

use block::ConcreteBlock;

use crate::error::Error;
use crate::foundation::{id, nil, NSUInteger};
use crate::image::Image;

mod config;
pub use config::{ThumbnailConfig, ThumbnailQuality};

#[derive(Debug)]
pub struct ThumbnailGenerator(pub Id<Object, Shared>);

impl ThumbnailGenerator {
    /// Returns the global shared, wrapped, QLThumbnailGenerator.
    pub fn shared() -> Self {
        ThumbnailGenerator(unsafe { msg_send_id![class!(QLThumbnailGenerator), sharedGenerator] })
    }

    /// Given a path and config, will generate a preview image, calling back on the provided
    /// callback closure.
    ///
    /// Note that this callback can come back on a separate thread, so react accordingly to get to
    /// the main thread if you need to.
    pub fn generate_from_path<F>(&self, path: &Path, config: ThumbnailConfig, callback: F)
    where
        F: Fn(Result<(Image, ThumbnailQuality), Error>) + Send + Sync + 'static
    {
        let block = ConcreteBlock::new(move |thumbnail: id, thumbnail_type: NSUInteger, error: id| {
            if error == nil {
                unsafe {
                    let image = Image::with(msg_send![thumbnail, NSImage]);
                    let quality = ThumbnailQuality::from(thumbnail_type);
                    callback(Ok((image, quality)));
                }
            } else {
                let error = Error::new(error);
                callback(Err(error));
            }
        });

        let block = block.copy();
        let request = config.to_request(path);

        unsafe {
            let _: () = msg_send![
                &*self.0,
                generateRepresentationsForRequest: request,
                updateHandler: &*block,
            ];
        }
    }
}
