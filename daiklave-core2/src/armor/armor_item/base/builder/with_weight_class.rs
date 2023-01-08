use std::collections::HashSet;

use crate::{armor::{armor_item::{ArmorTag, mundane::MundaneArmorMemo, ArmorWeightClass, artifact::BaseArtifactArmor, base::BaseArmor}}, book_reference::BookReference};

pub struct BaseArmorItemBuilderWithWeightClass {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) tags: HashSet<ArmorTag>,
    pub(crate) weight_class: ArmorWeightClass,
}

impl BaseArmorItemBuilderWithWeightClass {
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    pub fn tag(mut self, tag: ArmorTag) -> Self {
        self.tags.insert(tag);
        self
    }

    pub fn build_mundane(self) -> MundaneArmorMemo {
        MundaneArmorMemo(BaseArmor { 
            name: self.name,
            book_reference: self.book_reference,
            tags: self.tags,
            weight_class: self.weight_class,
        })
    }

    pub fn build_artifact(self) -> BaseArtifactArmor {
        BaseArtifactArmor(BaseArmor { 
            name: self.name,
            book_reference: self.book_reference,
            tags: self.tags,
            weight_class: self.weight_class,
        })
    }
}