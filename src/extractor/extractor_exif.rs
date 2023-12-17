use std::collections::HashMap;
use std::error::Error;
use std::io::Cursor;

use crate::extractor::extractor_trait::Extractor;
use crate::extractor::ContextExtractor;

pub struct ExifExtractor {}

impl ExifExtractor {
    pub fn new() -> Self {
        Self {}
    }
}

impl Extractor for ExifExtractor {
    fn extract(&self, mut context: ContextExtractor) -> Result<ContextExtractor, Box<dyn Error>> {
        let reader = exif::Reader::new();
        let exif = reader.read_from_container(&mut Cursor::new(&context.file))?;

        let mut fields_map = HashMap::new();

        for field in exif.fields() {
            fields_map.insert(
                field.tag.to_string(),
                field.display_value().with_unit(&exif).to_string(),
            );
        }

        for (key, value) in fields_map {
            context.output.metadata.embedded_metadata.insert(key, value);
        }

        Ok(context)
    }
}
