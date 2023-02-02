use std::collections::HashSet;

use crate::{
    armor::armor_item::{
        base::BaseArmor,
        mundane::{AddMundaneArmor, MundaneArmor, MundaneArmorName},
        ArmorTag, ArmorWeightClass,
    },
    book_reference::BookReference,
    CharacterMutation,
};

/// A mundane armor builder after the weight class has been specified.
pub struct MundaneArmorBuilderWithWeightClass {
    pub(crate) name: MundaneArmorName,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) tags: HashSet<ArmorTag>,
    pub(crate) weight_class: ArmorWeightClass,
}

impl MundaneArmorBuilderWithWeightClass {
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

    /// Completes the builder, returning an AddMundaneArmor mutation.
    pub fn build(self) -> AddMundaneArmor {
        AddMundaneArmor {
            name: self.name,
            armor: MundaneArmor(BaseArmor {
                book_reference: self.book_reference,
                weight_class: self.weight_class,
                tags: self.tags,
            }),
        }
    }
}

impl From<MundaneArmorBuilderWithWeightClass> for CharacterMutation {
    fn from(builder: MundaneArmorBuilderWithWeightClass) -> Self {
        builder.build().into()
    }
}
