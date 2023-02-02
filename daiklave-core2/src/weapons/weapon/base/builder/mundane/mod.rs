mod with_attack;
mod with_damage_type;
mod with_handedness;
mod with_weight;
use std::num::NonZeroU8;

pub use with_attack::MundaneWeaponBuilderWithAttack;
pub use with_damage_type::MundaneWeaponBuilderWithDamageType;
pub use with_handedness::MundaneWeaponBuilderWithHandedness;
pub use with_weight::MundaneWeaponBuilderWithWeight;

use crate::{
    book_reference::BookReference,
    weapons::weapon::{OptionalWeaponTag, RangeBand, WeaponWeightClass},
};

use super::BaseWeaponBuilder;

/// A mundane weapon builder.
pub struct MundaneWeaponBuilder(pub(crate) BaseWeaponBuilder, pub(crate) NonZeroU8);

impl MundaneWeaponBuilder {
    /// Starts building a new mundane weapon with the given name.
    pub fn name(name: impl Into<String>) -> Self {
        BaseWeaponBuilder::name(name).mundane()
    }

    /// Sets the book reference for the mundane weapon.
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self = Self(self.0.book_reference(book_reference), self.1);
        self
    }

    /// Sets the weapon to be usable up to a certain maximum range, using
    /// the Thrown accuracy curve. Note that this does NOT set the weapon to
    /// be usable using the Thrown skill; some unique weapons use the Thrown
    /// accuracy range but are Martial Arts only.
    pub fn thrown_range(mut self, max_range: RangeBand) -> Self {
        self = Self(self.0.thrown_range(max_range), self.1);
        self
    }

    /// Sets the weapon to be usable up to a certain maximum range, using
    /// the Archery accuracy curve. Note that this does NOT set the weapon to
    /// be usable using the Archery skill; some unique weapons use the Archery
    /// accuracy range but are Martial Arts only.
    pub fn archery_range(mut self, max_range: RangeBand) -> Self {
        self = Self(self.0.archery_range(max_range), self.1);
        self
    }

    /// Adds a tag to the weapon, other than Lethal, Bashing, Brawl, Melee,
    /// Martial Arts, Thrown(range), Archery(range), One-Handed, or Two-Handed.
    /// The relevance of the tag is not enforced--irrelevant tags will be
    /// displayed but may not be mechanically represented.
    pub fn tag(mut self, tag: OptionalWeaponTag) -> Self {
        self = Self(self.0.tag(tag), self.1);
        self
    }

    /// Sets the number of copies of this mundane weapon to add. Defaults to 1.
    pub fn quantity(mut self, quantity: NonZeroU8) -> Self {
        self.1 = quantity;
        self
    }

    /// Sets the weapon to be Light, Medium, or Heavy.
    pub fn weight_class(self, weight_class: WeaponWeightClass) -> MundaneWeaponBuilderWithWeight {
        MundaneWeaponBuilderWithWeight(self.0.weight_class(weight_class), self.1)
    }
}
