use std::collections::HashSet;

use crate::{
    armor::armor_item::{
        ArmorTag, ArmorWeightClass,
    },
    book_reference::BookReference,
};

use super::{MundaneArmorBuilderWithWeightClass, BaseArtifactArmorBuilderWithWeightClass};

/// A base armor item builder after the weight class has been set.
pub struct BaseArmorItemBuilderWithWeightClass {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) tags: HashSet<ArmorTag>,
    pub(crate) weight_class: ArmorWeightClass,
}

impl BaseArmorItemBuilderWithWeightClass {
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

    /// Sets the base armor item to be mundane.
    pub fn mundane(self) -> MundaneArmorBuilderWithWeightClass {
        MundaneArmorBuilderWithWeightClass {
            name: self.name.into(),
            book_reference: self.book_reference,
            tags: self.tags,
            weight_class: self.weight_class,
        }
    }

    /// Sets the base armor item to be an artifact.
    pub fn artifact(self) -> BaseArtifactArmorBuilderWithWeightClass {
        BaseArtifactArmorBuilderWithWeightClass {
            name: self.name,
            book_reference: self.book_reference,
            tags: self.tags,
            weight_class: self.weight_class,
        }
    }
}
