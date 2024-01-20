use bytes::BytesMut;
use std::collections::HashMap;
use std::error::Error;
use std::io::Cursor;

use crate::metadata::Metadata;

#[derive(Default)]
pub struct ExifExtractor {}

impl ExifExtractor {
    pub fn extract(
        &self,
        mut metadata: Metadata,
        body: BytesMut,
    ) -> Result<Metadata, Box<dyn Error>> {

        match exif::Reader::new().read_from_container(&mut Cursor::new(&body)) {
            Ok(exif) => {
                let mut fields_map = HashMap::new();
                for field in exif.fields() {
                    fields_map.insert(
                        field.tag.to_string(),
                        field.display_value().with_unit(&exif).to_string(),
                    );
                }

                for (key, value) in fields_map {
                    metadata.embedded_metadata.insert(key, value);
                }
            },
            Err(_) => {},
        }

        Ok(metadata)
    }
}
