use std::clone::Clone;
use std::fmt::Debug;

use crate::media::MediaHandler;

#[derive(Debug, Clone)]
pub struct MediaGroupHandler {
    media: MediaHandler,
    derived_medias: Vec<MediaHandler>,
}
