mod with_weight_class;
pub use with_weight_class::BaseArtifactArmorBuilderWithWeightClass;

use std::collections::HashSet;

use crate::{book_reference::BookReference, armor::armor_item::{ArmorTag, ArmorWeightClass}};

/// A builder to construct a basic piece of artifact armor. This is a 
/// generic base item like "Silken Armor" or "Articulated Plate (Artifact)",
/// not a named item.
pub struct BaseArtifactArmorBuilder {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) tags: HashSet<ArmorTag>,
}

impl BaseArtifactArmorBuilder {
    /// Creates a new builder with the given name.
    pub fn name(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            book_reference: Default::default(),
            tags: Default::default(),
        }
    }

    /// The book reference for the armor item.
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    /// Adds a tag to the armor item.
    pub fn tag(mut self, tag: ArmorTag) -> Self {
        self.tags.insert(tag);
        self
    }

    /// Sets the weight class of the armor item.
    pub fn weight_class(
        self,
        weight_class: ArmorWeightClass,
    ) -> BaseArtifactArmorBuilderWithWeightClass {
        BaseArtifactArmorBuilderWithWeightClass {
            name: self.name,
            book_reference: self.book_reference,
            tags: self.tags,
            weight_class,
        }
    }
}