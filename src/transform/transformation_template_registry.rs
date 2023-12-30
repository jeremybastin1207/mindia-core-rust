use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use super::TransformationTemplate;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransformationName {
    Scale,
    Watermark,
    Unset,
}

impl TransformationName {
    pub fn as_str(&self) -> &'static str {
        match *self {
            TransformationName::Scale => "c_scale",
            TransformationName::Watermark => "c_watermark",
            TransformationName::Unset => "",
        }
    }
}

pub struct TransformationTemplateRegistry {
    transformation_strings: Arc<RwLock<HashMap<String, TransformationTemplate>>>,
}

impl TransformationTemplateRegistry {
    pub fn new() -> Self {
        let mut reg = Self {
            transformation_strings: Arc::new(RwLock::new(HashMap::new())),
        };
        reg.populate_registry();
        reg
    }

    pub fn populate_registry(&mut self) {
        let mut transformation_strings = self.transformation_strings.write().unwrap();

        transformation_strings.insert(
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
        transformation_strings.insert(
            TransformationName::Watermark.as_str().to_string(),
            TransformationTemplate::new()
                .with_name(TransformationName::Watermark)
                .with_description("Scale the image to the given width and height".to_string())
                .with_arg(
                    "p".to_string(),
                    "The padding to add to the watermark to".to_string(),
                )
                .with_arg(
                    "a".to_string(),
                    "The anchor to apply to the watermark in regard to the image".to_string(),
                )
                .with_arg(
                    "w".to_string(),
                    "The width to scale the watermark to".to_string(),
                )
                .with_arg(
                    "h".to_string(),
                    "The height to scale the watermark to".to_string(),
                )
                .with_arg("f".to_string(), "The path to the watermark".to_string()),
        );
    }

    pub fn get_all(&self) -> Vec<TransformationTemplate> {
        return self
            .transformation_strings
            .read()
            .unwrap()
            .values()
            .cloned()
            .collect::<Vec<TransformationTemplate>>();
    }

    pub fn find_one(&self, transformation_string: &str) -> Option<TransformationTemplate> {
        return self
            .transformation_strings
            .read()
            .unwrap()
            .get(transformation_string)
            .cloned();
    }
}
