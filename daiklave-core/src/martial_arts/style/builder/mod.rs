mod with_description;
mod with_weapons;
pub use with_description::MartialArtsStyleBuilderWithDescription;
pub use with_weapons::MartialArtsStyleBuilderWithWeapons;

use crate::{armor::armor_item::ArmorWeightClass, book_reference::BookReference};

use super::MartialArtsStyleName;

/// A builder to construct a new Martial Arts style. Required fields are name
/// (already specified), description, and at least one weapon. Additional
/// weapons are optional, as is a maximum armor weight and book reference.
pub struct MartialArtsStyleBuilder {
    pub(crate) name: MartialArtsStyleName,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) max_armor_weight: Option<ArmorWeightClass>,
}

impl MartialArtsStyleBuilder {
    /// Starts the builder with the given name.
    pub fn name(name: impl Into<MartialArtsStyleName>) -> Self {
        Self {
            name: name.into(),
            book_reference: None,
            max_armor_weight: None,
        }
    }

    /// The book reference for the style.
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    /// If the style is compatible with armor, the maximum usable weight.
    pub fn max_armor_weight(mut self, weight: ArmorWeightClass) -> Self {
        self.max_armor_weight = Some(weight);
        self
    }

    /// The description of the style's practices.
    pub fn description(
        self,
        description: impl Into<String>,
    ) -> MartialArtsStyleBuilderWithDescription {
        MartialArtsStyleBuilderWithDescription {
            name: self.name,
            description: description.into(),
            book_reference: self.book_reference,
            max_armor_weight: self.max_armor_weight,
        }
    }
}
