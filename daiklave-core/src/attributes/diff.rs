use super::AttributeName;
use crate::attributes::Attributes;

#[derive(Debug, Default)]
pub struct AttributesDiff {
    pub updated_attributes: Vec<(AttributeName, u8)>,
}

impl Attributes {
    pub fn compare_newer(&self, newer: &Self) -> AttributesDiff {
        let mut diff = AttributesDiff::default();

        newer.iter().for_each(|attribute| {
            if attribute.dots() != self.get(attribute.name()).dots() {
                diff.updated_attributes.push((attribute.name(), attribute.dots()))
            }
        });

        diff
    }
}
