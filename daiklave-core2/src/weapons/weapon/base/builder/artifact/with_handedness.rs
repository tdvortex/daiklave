use crate::{
    book_reference::BookReference,
    weapons::weapon::{
        builder::base::BaseWeaponBuilderWithHandedness, OptionalWeaponTag, RangeBand,
    },
};

use super::BaseArtifactWeaponBuilderWithDamageType;

/// A builder for a base artifact weapon, after the handedness has been specified.
pub struct BaseArtifactWeaponBuilderWithHandedness(pub(crate) BaseWeaponBuilderWithHandedness);

impl BaseArtifactWeaponBuilderWithHandedness {
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

    /// Sets the weapon to deal Lethal damage by default. Typical for bladed or
    /// piercing weapons.
    pub fn lethal(self) -> BaseArtifactWeaponBuilderWithDamageType {
        BaseArtifactWeaponBuilderWithDamageType(self.0.lethal())
    }

    /// Sets the weapon to deal Bashing damage by default. Typical for blunt
    /// weapons.
    pub fn bashing(self) -> BaseArtifactWeaponBuilderWithDamageType {
        BaseArtifactWeaponBuilderWithDamageType(self.0.bashing())
    }
}
