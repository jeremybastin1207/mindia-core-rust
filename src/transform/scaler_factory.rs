use std::error::Error;

use crate::transform::scaler::Scaler;
use crate::transform::transformation::Transformation;
use crate::types::Size;

pub fn create_scaler(transformation: Transformation) -> Result<Box<Scaler>, Box<dyn Error>> {
    let height = transformation
        .args
        .get("h")
        .ok_or("Key 'h' not found")?
        .parse::<u32>()
        .map_err(|_| "Failed to parse 'h' value to u32")?;

    let width = transformation
        .args
        .get("w")
        .ok_or("Key 'w' not found")?
        .parse::<u32>()
        .map_err(|_| "Failed to parse 'w' value to u32")?;

    Ok(Box::new(Scaler::new(Size::new(height, width))))
}
