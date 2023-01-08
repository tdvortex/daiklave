use std::collections::HashSet;

use crate::{
    armor::armor_item::{ArmorTag, ArmorWeightClass},
    book_reference::BookReference,
};

use self::with_weight_class::BaseArmorItemBuilderWithWeightClass;

mod with_weight_class;

pub struct BaseArmorItemBuilder {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) tags: HashSet<ArmorTag>,
}

impl BaseArmorItemBuilder {
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    pub fn tag(mut self, tag: ArmorTag) -> Self {
        self.tags.insert(tag);
        self
    }

    pub fn weight_class(
        self,
        weight_class: ArmorWeightClass,
    ) -> BaseArmorItemBuilderWithWeightClass {
        BaseArmorItemBuilderWithWeightClass {
            name: self.name,
            book_reference: self.book_reference,
            tags: self.tags,
            weight_class,
        }
    }
}
