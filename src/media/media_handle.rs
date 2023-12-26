use bytes::BytesMut;
use std::clone::Clone;
use std::fmt::Debug;

use crate::metadata::Metadata;

#[derive(Default, Debug, Clone)]
pub struct MediaHandle {
    pub body: BytesMut,
    pub metadata: Metadata,
}

impl MediaHandle {
    pub fn new(body: BytesMut, metadata: Metadata) -> Self {
        Self { body, metadata }
    }
}
