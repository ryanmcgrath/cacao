use objc::runtime::{Object};
use objc::{class, msg_send, sel, sel_impl};
use objc_id::ShareId;

use block::ConcreteBlock;

use url::Url;

use crate::error::Error;
use crate::foundation::{id, NSUInteger};
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

    pub fn generate<F>(&self, url: &Url, config: ThumbnailConfig, callback: F)
    where
        //F: Fn(Result<(Image, ThumbnailQuality), Error>) + Send + Sync + 'static
        F: Fn(Result<(Image, ThumbnailQuality), Error>) + Send + Sync + 'static
    {
        let block = ConcreteBlock::new(move |thumbnail: id, thumbnail_type: NSUInteger, error: id| {
            unsafe {
                let image = Image::with(msg_send![thumbnail, NSImage]);
                callback(Ok((image, ThumbnailQuality::Low)));
            }
        });

        let block = block.copy();
        let request = config.to_request(url);

        unsafe {
            let _: () = msg_send![&*self.0, generateRepresentationsForRequest:request 
                updateHandler:block];
        }
    }
}
