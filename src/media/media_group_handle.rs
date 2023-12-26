use std::clone::Clone;
use std::fmt::Debug;

use crate::media::MediaHandle;

#[derive(Default, Debug, Clone)]
pub struct MediaGroupHandle {
    pub media: MediaHandle,
    pub derived_medias: Vec<MediaHandle>,
}

impl MediaGroupHandle {
    pub fn new(media: MediaHandle, derived_medias: Vec<MediaHandle>) -> Self {
        Self {
            media,
            derived_medias,
        }
    }

    pub fn add_derived_media(&mut self, media: MediaHandle) {
        self.media
            .metadata
            .derived_medias
            .push(media.metadata.clone());
        self.derived_medias.push(media);
    }
}
