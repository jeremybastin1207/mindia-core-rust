use bytes::BytesMut;
use std::collections::HashMap;
use std::io::Cursor;

pub struct ExifExtractor {}

impl ExifExtractor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn extract(
        &self,
        picture_data: BytesMut,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let reader = exif::Reader::new();
        let exif = reader.read_from_container(&mut Cursor::new(&picture_data))?;

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
