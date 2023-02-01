use std::collections::HashSet;

use crate::{book_reference::BookReference, armor::armor_item::{ArmorTag, ArmorWeightClass, artifact::{AddBaseArtifactArmor, BaseArtifactArmor, ArtifactArmorName, builder::{ArtifactArmorItemBuilderWithBaseArmor, ArtifactArmorItemBuilder}}, base::BaseArmor}};

/// A base artifact armor builder after the weight class has been specified.
pub struct BaseArtifactArmorBuilderWithWeightClass {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) tags: HashSet<ArmorTag>,
    pub(crate) weight_class: ArmorWeightClass,
}

impl BaseArtifactArmorBuilderWithWeightClass {
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

    /// Completes the builder, returning an AddBaseArtifactArmor struct. This
    /// is not a complete item but can be used in constructing a true piece
    /// of artifact armor.
    pub fn build(self) -> AddBaseArtifactArmor {
        AddBaseArtifactArmor {
            name: self.name,
            armor: BaseArtifactArmor(BaseArmor {
                book_reference: self.book_reference,
                weight_class: self.weight_class,
                tags: self.tags,
            })
        }
    }

    /// Continues the builder into a named piece of artifact armor.
    pub fn unique_name(self, name: impl Into<ArtifactArmorName>) -> ArtifactArmorItemBuilderWithBaseArmor {
        ArtifactArmorItemBuilder::name(name).base_artifact(self.build())
    }
}