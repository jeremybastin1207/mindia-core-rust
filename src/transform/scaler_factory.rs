use std::error::Error;

use super::{Scaler, TransformationDescriptor};
use crate::types::Size;

pub fn create_scaler(
    transformation_descriptor: TransformationDescriptor,
) -> Result<Box<Scaler>, Box<dyn Error>> {
    let height = transformation_descriptor
        .arg_values
        .get("h")
        .ok_or("Key 'h' not found")?
        .parse::<u32>()
        .map_err(|_| "Failed to parse 'h' value to u32")?;

    let width = transformation_descriptor
        .arg_values
        .get("w")
        .ok_or("Key 'w' not found")?
        .parse::<u32>()
        .map_err(|_| "Failed to parse 'w' value to u32")?;

    Ok(Box::new(Scaler::new(Size::new(height, width))))
}
