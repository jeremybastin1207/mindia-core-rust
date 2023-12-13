use bytes::BytesMut;
use std::error::Error;

use crate::media::Path;

pub trait Transform {
    fn transform(&self, path: &mut Path, bytes: BytesMut) -> Result<(), Box<dyn Error>>;
}
