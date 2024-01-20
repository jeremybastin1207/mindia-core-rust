use std::error::Error;
use async_trait::async_trait;
use crate::{
    extractor::ExifExtractor,
    media::MediaHandle,
    pipeline::{PipelineContext, PipelineStep},
    transform::{
        PathGenerator, Scaler, TransformationDescriptorChain, WebpConverter,
    },
};

#[derive(Default, Clone)]
pub struct UploadMediaContext {
    pub media_handle: MediaHandle,
    pub transformations: TransformationDescriptorChain,
}

#[async_trait]
impl PipelineStep<UploadMediaContext> for ExifExtractor {
    async fn execute(
        &self,
        mut ctx: PipelineContext<UploadMediaContext>,
    ) -> Result<PipelineContext<UploadMediaContext>, Box<dyn Error>> {
        ctx.attributes.media_handle.metadata = self.extract(
            ctx.attributes.media_handle.metadata,
            ctx.attributes.media_handle.body.clone(),
        )?;

        Ok(ctx)
    }
}

#[async_trait]
impl PipelineStep<UploadMediaContext> for WebpConverter {
    async fn execute(
        &self,
        mut ctx: PipelineContext<UploadMediaContext>,
    ) -> Result<PipelineContext<UploadMediaContext>, Box<dyn Error>> {
        let (path, body) = self.transform(
            ctx.attributes.media_handle.metadata.path,
            ctx.attributes.media_handle.body.clone(),
        )?;

        ctx.attributes.media_handle.metadata.path = path;
        ctx.attributes.media_handle.body = body;

        Ok(ctx)
    }
}

#[async_trait]
impl PipelineStep<UploadMediaContext> for Scaler {
    async fn execute(
        &self,
        mut ctx: PipelineContext<UploadMediaContext>,
    ) -> Result<PipelineContext<UploadMediaContext>, Box<dyn Error>> {
        let body = self.transform(
            ctx.attributes.media_handle.metadata.path.clone(),
            ctx.attributes.media_handle.body.clone(),
        )?;

        ctx.attributes.media_handle.body = body;

        Ok(ctx)
    }
}

// #[async_trait]
// impl PipelineStep<UploadMediaContext> for Watermarker {
//     async fn execute(
//         &self,
//         mut ctx: PipelineContext<UploadMediaContext>,
//     ) -> Result<PipelineContext<UploadMediaContext>, Box<dyn Error>> {
//         let body = self.transform(ctx.attributes.media_handle.body.clone())?;
//
//         ctx.attributes.media_handle.body = body;
//
//         Ok(ctx)
//     }
// }

#[async_trait]
impl PipelineStep<UploadMediaContext> for PathGenerator {
    async fn execute(
        &self,
        mut ctx: PipelineContext<UploadMediaContext>,
    ) -> Result<PipelineContext<UploadMediaContext>, Box<dyn Error>> {
        let path = self.transform(
            &ctx.attributes.media_handle.metadata.path,
            &ctx.attributes.transformations.clone(),
        )?;

        ctx.attributes.media_handle.metadata.path = path;

        Ok(ctx)
    }
}
