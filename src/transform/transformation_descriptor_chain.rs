use super::TransformationDescriptor;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct TransformationDescriptorChain {
    transformation_descriptors: Vec<TransformationDescriptor>,
}

impl TransformationDescriptorChain {
    pub fn new() -> Self {
        Self {
            transformation_descriptors: Vec::new(),
        }
    }

    pub fn set(&mut self, transformation_descriptors: Vec<TransformationDescriptor>) {
        self.transformation_descriptors = transformation_descriptors;
    }

    pub fn add(&mut self, transformation_descriptor: TransformationDescriptor) {
        self.transformation_descriptors
            .push(transformation_descriptor);
    }

    pub fn get_trasnfomation_descriptors(&self) -> &Vec<TransformationDescriptor> {
        &self.transformation_descriptors
    }

    pub fn is_empty(&self) -> bool {
        self.transformation_descriptors.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &TransformationDescriptor> {
        self.transformation_descriptors.iter()
    }

    pub fn as_str(&self) -> String {
        self.transformation_descriptors
            .iter()
            .map(|transformation_descriptor| transformation_descriptor.as_str())
            .collect::<Vec<_>>()
            .join(",")
    }
}
