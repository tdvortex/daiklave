mod with_attack;
mod with_damage_type;
mod with_handedness;
mod with_weight;

pub use with_attack::BaseArtifactWeaponBuilderWithAttack;
pub use with_damage_type::BaseArtifactWeaponBuilderWithDamageType;
pub use with_handedness::BaseArtifactWeaponBuilderWithHandedness;
pub use with_weight::BaseArtifactWeaponBuilderWithWeight;

use crate::{
    book_reference::BookReference,
    weapons::weapon::{OptionalWeaponTag, RangeBand, WeaponWeightClass},
};

use super::BaseWeaponBuilder;

/// A builder for a base artifact weapon.
pub struct BaseArtifactWeaponBuilder(pub(crate) BaseWeaponBuilder);

impl BaseArtifactWeaponBuilder {
    /// Starts building a new base artifact weapon with the given name.
    pub fn name(name: impl Into<String>) -> Self {
        BaseWeaponBuilder::name(name).artifact()
    }

    /// Sets the book reference for the base artifact weapon.
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self = Self(self.0.book_reference(book_reference));
        self
    }

    /// Sets the weapon to be usable up to a certain maximum range, using
    /// the Thrown accuracy curve. Note that this does NOT set the weapon to
    /// be usable using the Thrown skill; some unique weapons use the Thrown
    /// accuracy range but are Martial Arts only.
    pub fn thrown_range(mut self, max_range: RangeBand) -> Self {
        self = Self(self.0.thrown_range(max_range));
        self
    }

    /// Sets the weapon to be usable up to a certain maximum range, using
    /// the Archery accuracy curve. Note that this does NOT set the weapon to
    /// be usable using the Archery skill; some unique weapons use the Archery
    /// accuracy range but are Martial Arts only.
    pub fn archery_range(mut self, max_range: RangeBand) -> Self {
        self = Self(self.0.archery_range(max_range));
        self
    }

    /// Adds a tag to the weapon, other than Lethal, Bashing, Brawl, Melee,
    /// Martial Arts, Thrown(range), Archery(range), One-Handed, or Two-Handed.
    /// The relevance of the tag is not enforced--irrelevant tags will be
    /// displayed but may not be mechanically represented.
    pub fn tag(mut self, tag: OptionalWeaponTag) -> Self {
        self = Self(self.0.tag(tag));
        self
    }

    /// Sets the weapon to be Light, Medium, or Heavy.
    pub fn weight_class(
        self,
        weight_class: WeaponWeightClass,
    ) -> BaseArtifactWeaponBuilderWithWeight {
        BaseArtifactWeaponBuilderWithWeight(self.0.weight_class(weight_class))
    }
}
