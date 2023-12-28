use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::TransformationTemplate;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TransformationName {
    Scale,
    Unset,
}

impl TransformationName {
    pub fn as_str(&self) -> &'static str {
        match *self {
            TransformationName::Scale => "c_scale",
            TransformationName::Unset => "",
        }
    }
}

pub struct TransformationTemplateRegistry {
    transformation_strings: HashMap<String, TransformationTemplate>,
}

impl TransformationTemplateRegistry {
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
            TransformationTemplate::new()
                .with_name(TransformationName::Scale)
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

    pub fn get_all(&self) -> Vec<TransformationTemplate> {
        return self
            .transformation_strings
            .values()
            .cloned()
            .collect::<Vec<TransformationTemplate>>();
    }

    pub fn find_one(&self, transformation_string: &str) -> Option<TransformationTemplate> {
        return self
            .transformation_strings
            .get(transformation_string)
            .cloned();
    }
}
