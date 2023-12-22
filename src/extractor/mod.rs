pub mod context;
pub mod extractor_exif;
pub mod extractor_named_transformation;
pub mod extractor_trait;

pub use context::{ContextExtractor, ExtractorOutput};
pub use extractor_exif::ExifExtractor;
pub use extractor_named_transformation::NamedTransformationExtractor;
pub use extractor_trait::Extractor;
