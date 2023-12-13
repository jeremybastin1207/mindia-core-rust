use bytes::Bytes;
use std::collections::HashMap;
use std::error::Error;
use std::io::Cursor;

use crate::extractor::extractor_trait::Extractor;

pub struct ExifExtractor {}

impl ExifExtractor {
    pub fn new() -> Self {
        Self {}
    }
}

impl Extractor for ExifExtractor {
    type Output = HashMap<String, String>;

    fn extract(&self, data: Bytes) -> Result<Self::Output, Box<dyn Error>> {
        let reader = exif::Reader::new();
        let exif = reader.read_from_container(&mut Cursor::new(&data))?;

        let mut fields_map = HashMap::new();

        for field in exif.fields() {
            fields_map.insert(
                field.tag.to_string(),
                field.display_value().with_unit(&exif).to_string(),
            );
        }

        Ok(fields_map)
    }
}
