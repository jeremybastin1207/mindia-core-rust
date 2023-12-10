pub mod storage_filesystem;
pub mod storage_s3;
pub mod storage_trait;

pub use storage_filesystem::FilesystemStorage;
pub use storage_s3::S3Storage;
pub use storage_trait::FileStorage;
