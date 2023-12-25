use bytes::BytesMut;
use std::error::Error;

use crate::extractor::ExifExtractor;
use crate::media::Path;
use crate::metadata::Metadata;
use crate::pipeline::{PipelineContext, PipelineStep};
use crate::transform::{PathGenerator, Scaler, TransformationDescriptorChain, WebpConverter};

#[derive(Default, Clone)]
pub struct UploadMediaContext {
    pub path: Path,
    pub body: BytesMut,
    pub metadata: Metadata,
    pub transformations: TransformationDescriptorChain,
}

impl PipelineStep<UploadMediaContext> for ExifExtractor {
    fn execute(
        &self,
        mut context: PipelineContext<UploadMediaContext>,
    ) -> Result<PipelineContext<UploadMediaContext>, Box<dyn Error>> {
        context.attributes.metadata =
            self.extract(context.attributes.metadata, context.attributes.body.clone())?;

        Ok(context)
    }
}

impl PipelineStep<UploadMediaContext> for WebpConverter {
    fn execute(
        &self,
        mut context: PipelineContext<UploadMediaContext>,
    ) -> Result<PipelineContext<UploadMediaContext>, Box<dyn Error>> {
        let (path, body) =
            self.transform(context.attributes.path, context.attributes.body.clone())?;

        context.attributes.path = path;
        context.attributes.body = body;

        Ok(context)
    }
}

impl PipelineStep<UploadMediaContext> for Scaler {
    fn execute(
        &self,
        mut context: PipelineContext<UploadMediaContext>,
    ) -> Result<PipelineContext<UploadMediaContext>, Box<dyn Error>> {
        let body = self.transform(
            context.attributes.path.clone(),
            context.attributes.body.clone(),
        )?;

        context.attributes.body = body;

        Ok(context)
    }
}

impl PipelineStep<UploadMediaContext> for PathGenerator {
    fn execute(
        &self,
        mut context: PipelineContext<UploadMediaContext>,
    ) -> Result<PipelineContext<UploadMediaContext>, Box<dyn Error>> {
        let path = self.transform(
            &context.attributes.path,
            context.attributes.transformations.clone(),
        )?;

        context.attributes.path = path;

        Ok(context)
    }
}
