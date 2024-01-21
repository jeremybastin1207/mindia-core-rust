use std::error::Error;
use async_trait::async_trait;
use crate::extractor::{ContentInfoExtractor, ExifExtractor};
use crate::media::MediaHandle;
use crate::pipeline::PipelineStep;
use crate::transform::{PathGenerator, Scaler, TransformationDescriptorChain, Watermarker, WebpConverter};

#[derive(Default, Clone)]
pub struct UploadMediaContext {
    pub media_handle: MediaHandle,
    pub transformations: TransformationDescriptorChain,
}

#[async_trait]
impl PipelineStep<UploadMediaContext> for ExifExtractor {
    async fn execute(
        &self,
        mut ctx: UploadMediaContext,
    ) -> Result<UploadMediaContext, Box<dyn Error>> {
        ctx.media_handle.metadata = self.extract(
            ctx.media_handle.metadata,
            ctx.media_handle.body.clone(),
        )?;

        Ok(ctx)
    }
}

#[async_trait]
impl PipelineStep<UploadMediaContext> for ContentInfoExtractor {
    async fn execute(
        &self,
        mut ctx: UploadMediaContext,
    ) -> Result<UploadMediaContext, Box<dyn Error>> {
        ctx.media_handle.metadata = self.extract(
            ctx.media_handle.metadata,
            ctx.media_handle.body.clone(),
        )?;

        Ok(ctx)
    }
}

#[async_trait]
impl PipelineStep<UploadMediaContext> for WebpConverter {
    async fn execute(
        &self,
        mut ctx: UploadMediaContext,
    ) -> Result<UploadMediaContext, Box<dyn Error>> {
        let (path, body) = self.transform(
            ctx.media_handle.metadata.path,
            ctx.media_handle.body.clone(),
        )?;

        ctx.media_handle.metadata.path = path;
        ctx.media_handle.metadata.content_type = Some("image/webp".to_string());
        ctx.media_handle.body = body;

        Ok(ctx)
    }
}

#[async_trait]
impl PipelineStep<UploadMediaContext> for Scaler {
    async fn execute(
        &self,
        mut ctx: UploadMediaContext,
    ) -> Result<UploadMediaContext, Box<dyn Error>> {
        let body = self.transform(
            ctx.media_handle.metadata.path.clone(),
            ctx.media_handle.body.clone(),
        )?;

        ctx.media_handle.body = body;

        Ok(ctx)
    }
}

#[async_trait]
impl PipelineStep<UploadMediaContext> for Watermarker {
    async fn execute(
        &self,
        mut ctx: UploadMediaContext,
    ) -> Result<UploadMediaContext, Box<dyn Error>> {
        let body = self.transform(ctx.media_handle.body.clone()).await?;

        ctx.media_handle.body = body;

        Ok(ctx)
    }
}

#[async_trait]
impl PipelineStep<UploadMediaContext> for PathGenerator {
    async fn execute(
        &self,
        mut ctx: UploadMediaContext,
    ) -> Result<UploadMediaContext, Box<dyn Error>> {
        let path = self.transform(
            &ctx.media_handle.metadata.path,
            &ctx.transformations.clone(),
        )?;

        ctx.media_handle.metadata.path = path;

        Ok(ctx)
    }
}
