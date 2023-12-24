use std::collections::HashMap;

use super::TransformationDescription;

enum TransformationName {
    Scale,
}

impl TransformationName {
    fn as_str(&self) -> &'static str {
        match *self {
            TransformationName::Scale => "scale",
        }
    }
}

pub struct TransformationDescriptionRegistry {
    transformation_strings: HashMap<String, TransformationDescription>,
}

impl TransformationDescriptionRegistry {
    pub fn new() -> Self {
        let mut reg = Self {
            transformation_strings: HashMap::new(),
        };
        reg.populate_registry();
        reg
    }

    pub fn populate_registry(&mut self) {
        self.transformation_strings.insert(
            TransformationName::Scale.as_str().to_string(),
            TransformationDescription::default()
                .with_name(TransformationName::Scale.as_str().to_string().to_string())
                .with_description("Scale the image to the given width and height".to_string())
                .with_arg(
                    "w".to_string(),
                    "The width to scale the image to".to_string(),
                )
                .with_arg(
                    "h".to_string(),
                    "The height to scale the image to".to_string(),
                ),
        );
    }

    pub fn get_all(&self) -> Vec<TransformationDescription> {
        return self
            .transformation_strings
            .values()
            .cloned()
            .collect::<Vec<TransformationDescription>>();
    }

    pub fn find_one(&self, transformation_string: &str) -> Option<TransformationDescription> {
        return self
            .transformation_strings
            .get(transformation_string)
            .cloned();
    }
}
