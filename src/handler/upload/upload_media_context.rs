use std::error::Error;

use crate::extractor::ExifExtractor;
use crate::media::MediaHandle;
use crate::pipeline::{PipelineContext, PipelineStep};
use crate::transform::{
    PathGenerator, Scaler, TransformationDescriptorChain, Watermarker, WebpConverter,
};

#[derive(Default, Clone)]
pub struct UploadMediaContext {
    pub media_handle: MediaHandle,
    pub transformations: TransformationDescriptorChain,
}

impl PipelineStep<UploadMediaContext> for ExifExtractor {
    fn execute(
        &self,
        mut context: PipelineContext<UploadMediaContext>,
    ) -> Result<PipelineContext<UploadMediaContext>, Box<dyn Error>> {
        context.attributes.media_handle.metadata = self.extract(
            context.attributes.media_handle.metadata,
            context.attributes.media_handle.body.clone(),
        )?;

        Ok(context)
    }
}

impl PipelineStep<UploadMediaContext> for WebpConverter {
    fn execute(
        &self,
        mut context: PipelineContext<UploadMediaContext>,
    ) -> Result<PipelineContext<UploadMediaContext>, Box<dyn Error>> {
        let (path, body) = self.transform(
            context.attributes.media_handle.metadata.path,
            context.attributes.media_handle.body.clone(),
        )?;

        context.attributes.media_handle.metadata.path = path;
        context.attributes.media_handle.body = body;

        Ok(context)
    }
}

impl PipelineStep<UploadMediaContext> for Scaler {
    fn execute(
        &self,
        mut context: PipelineContext<UploadMediaContext>,
    ) -> Result<PipelineContext<UploadMediaContext>, Box<dyn Error>> {
        let body = self.transform(
            context.attributes.media_handle.metadata.path.clone(),
            context.attributes.media_handle.body.clone(),
        )?;

        context.attributes.media_handle.body = body;

        Ok(context)
    }
}

impl PipelineStep<UploadMediaContext> for Watermarker {
    fn execute(
        &self,
        mut context: PipelineContext<UploadMediaContext>,
    ) -> Result<PipelineContext<UploadMediaContext>, Box<dyn Error>> {
        let body = self.transform(context.attributes.media_handle.body.clone())?;

        context.attributes.media_handle.body = body;

        Ok(context)
    }
}

impl PipelineStep<UploadMediaContext> for PathGenerator {
    fn execute(
        &self,
        mut context: PipelineContext<UploadMediaContext>,
    ) -> Result<PipelineContext<UploadMediaContext>, Box<dyn Error>> {
        let path = self.transform(
            context.attributes.media_handle.metadata.path,
            context.attributes.transformations.clone(),
        )?;

        context.attributes.media_handle.metadata.path = path;

        Ok(context)
    }
}
