use std::path::Path;

use objc::runtime::{Object};
use objc::{class, msg_send, sel, sel_impl};
use objc_id::ShareId;

use block::ConcreteBlock;

use crate::error::Error;
use crate::foundation::{id, nil, NSUInteger};
use crate::image::Image;

mod config;
pub use config::{ThumbnailConfig, ThumbnailQuality};

#[derive(Debug)]
pub struct ThumbnailGenerator(pub ShareId<Object>);

impl ThumbnailGenerator {
    pub fn shared() -> Self {
        ThumbnailGenerator(unsafe {
            ShareId::from_ptr(msg_send![class!(QLThumbnailGenerator), sharedGenerator])
        })
    }

    pub fn generate<F>(&self, path: &Path, config: ThumbnailConfig, callback: F)
    where
        F: Fn(Result<(Image, ThumbnailQuality), Error>) + Send + Sync + 'static
    {
        let block = ConcreteBlock::new(move |thumbnail: id, thumbnail_type: NSUInteger, error: id| {
            if error == nil {
                unsafe {
                    let image = Image::with(msg_send![thumbnail, NSImage]);
                    let quality = ThumbnailQuality::from(thumbnail_type);
                    callback(Ok((image, ThumbnailQuality::Low)));
                }
            } else {
                let error = Error::new(error);
                callback(Err(error));
            }
        });

        let block = block.copy();
        let request = config.to_request(path);

        unsafe {
            let _: () = msg_send![&*self.0, generateRepresentationsForRequest:request 
                updateHandler:block];
        }
    }
}
