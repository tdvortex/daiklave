use std::collections::HashSet;

use crate::{
    armor::armor_item::{
        artifact::BaseArtifactArmor,
        base::BaseArmor,
        mundane::{AddMundaneArmor, MundaneArmor},
        ArmorTag, ArmorWeightClass,
    },
    book_reference::BookReference,
};

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

    /// Completes the build process as a mundane armor item.
    pub fn build_mundane(self) -> AddMundaneArmor {
        AddMundaneArmor {
            name: self.name.into(),
            armor: MundaneArmor(BaseArmor {
                book_reference: self.book_reference,
                tags: self.tags,
                weight_class: self.weight_class,
            }),
        }
    }

    /// Completes the build process as a base artifact armor item.
    /// This is **not** usable directly but is added to an artifact armor item
    /// build.
    pub fn build_artifact(self) -> (String, BaseArtifactArmor) {
        (
            self.name,
            BaseArtifactArmor(BaseArmor {
                book_reference: self.book_reference,
                tags: self.tags,
                weight_class: self.weight_class,
            }),
        )
    }
}
