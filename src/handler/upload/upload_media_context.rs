use std::error::Error;
use futures::executor::block_on;

use crate::{
    extractor::ExifExtractor,
    media::MediaHandle,
    pipeline::{PipelineContext, PipelineStep},
    transform::{
        PathGenerator, Scaler, TransformationDescriptorChain, Watermarker, WebpConverter,
    },
};
use crate::transform::Colorizer;

#[derive(Default, Clone)]
pub struct UploadMediaContext {
    pub media_handle: MediaHandle,
    pub transformations: TransformationDescriptorChain,
}

impl PipelineStep<UploadMediaContext> for ExifExtractor {
    fn execute(
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

impl PipelineStep<UploadMediaContext> for WebpConverter {
    fn execute(
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

impl PipelineStep<UploadMediaContext> for Scaler {
    fn execute(
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

impl PipelineStep<UploadMediaContext> for Watermarker {
    fn execute(
        &self,
        mut ctx: PipelineContext<UploadMediaContext>,
    ) -> Result<PipelineContext<UploadMediaContext>, Box<dyn Error>> {
        let body = self.transform(ctx.attributes.media_handle.body.clone())?;

        ctx.attributes.media_handle.body = body;

        Ok(ctx)
    }
}

impl PipelineStep<UploadMediaContext> for PathGenerator {
    fn execute(
        &self,
        mut ctx: PipelineContext<UploadMediaContext>,
    ) -> Result<PipelineContext<UploadMediaContext>, Box<dyn Error>> {
        let path = self.transform(
            ctx.attributes.media_handle.metadata.path,
            ctx.attributes.transformations.clone(),
        )?;

        ctx.attributes.media_handle.metadata.path = path;

        Ok(ctx)
    }
}
