mod with_attack;
mod with_damage_type;
mod with_handedness;
mod with_weight;

pub use with_attack::BaseWeaponBuilderWithAttack;
pub use with_damage_type::BaseWeaponBuilderWithDamageType;
pub use with_handedness::BaseWeaponBuilderWithHandedness;
pub use with_weight::BaseWeaponBuilderWithWeight;

use std::collections::HashSet;

use crate::{
    book_reference::BookReference,
    weapons::weapon::{
        range::{RangeBand, WeaponRange},
        tag::OptionalWeaponTag,
        weight_class::WeaponWeightClass,
    },
};

/// A builder for a base weapon, either a base mundane weapon (like "sword")
/// or a base artifact weapon (like "daiklave"). Required fields must be
/// specified in order: name, weight class, handedness, damage type, and
/// primary attack Ability. Optional fields like book reference, weapon ranges,
/// and optional tags can be added at any time prior to the final build.
pub struct BaseWeaponBuilder {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) attack_range: WeaponRange,
    pub(crate) tags: HashSet<OptionalWeaponTag>,
}

impl BaseWeaponBuilder {
    /// The book reference for the base weapon. Note that, for artifacts,
    /// this is for the non-unique weapon (like "grand daiklave") not the
    /// page reference of the unique weapon (like "Volcano Cutter").
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    /// Sets the weapon to be usable up to a certain maximum range, using
    /// the Thrown accuracy curve. Note that this does NOT set the weapon to
    /// be usable using the Thrown skill; some unique weapons use the Thrown
    /// accuracy range but are Martial Arts only.
    pub fn thrown_range(mut self, max_range: RangeBand) -> Self {
        self.attack_range = WeaponRange::Throwable(max_range);
        self
    }

    /// Sets the weapon to be usable up to a certain maximum range, using
    /// the Archery accuracy curve. Note that this does NOT set the weapon to
    /// be usable using the Archery skill; some unique weapons use the Archery
    /// accuracy range but are Martial Arts only.
    pub fn archery_range(mut self, max_range: RangeBand) -> Self {
        self.attack_range = WeaponRange::Archery(max_range);
        self
    }

    /// Adds a tag to the weapon, other than Lethal, Bashing, Brawl, Melee,
    /// Martial Arts, Thrown(range), Archery(range), One-Handed, or Two-Handed.
    /// The relevance of the tag is not enforced--irrelevant tags will be
    /// displayed but may not be mechanically represented.
    pub fn tag(mut self, tag: OptionalWeaponTag) -> Self {
        self.tags.insert(tag);
        self
    }

    /// Sets the weapon to be Light, Medium, or Heavy.
    pub fn weight_class(self, weight_class: WeaponWeightClass) -> BaseWeaponBuilderWithWeight {
        BaseWeaponBuilderWithWeight {
            name: self.name,
            book_reference: self.book_reference,
            attack_range: self.attack_range,
            tags: self.tags,
            weight_class,
        }
    }
}
