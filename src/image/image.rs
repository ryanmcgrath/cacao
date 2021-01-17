use objc_id::ShareId;
use objc::runtime::Object;

use crate::foundation::{id};

#[derive(Clone, Debug)]
pub struct Image(pub ShareId<Object>);

impl Image {
    pub fn with(image: id) -> Self {
        Image(unsafe {
            ShareId::from_ptr(image)
        })
    }
}

