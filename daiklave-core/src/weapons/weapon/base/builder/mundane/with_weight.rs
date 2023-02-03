use std::num::NonZeroU8;

use crate::{
    book_reference::BookReference,
    weapons::weapon::{builder::base::BaseWeaponBuilderWithWeight, OptionalWeaponTag, RangeBand},
};

use super::MundaneWeaponBuilderWithHandedness;

/// A mundane weapon builder after the weight class has been specified.
pub struct MundaneWeaponBuilderWithWeight(
    pub(crate) BaseWeaponBuilderWithWeight,
    pub(crate) NonZeroU8,
);

impl MundaneWeaponBuilderWithWeight {
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

    /// Defines the weapon to be Natural, part of the wielder's body.
    pub fn natural(self) -> MundaneWeaponBuilderWithHandedness {
        MundaneWeaponBuilderWithHandedness(self.0.natural(), self.1)
    }

    /// Defines the weapon to be Worn, requiring no hands to wield.
    pub fn worn(self) -> MundaneWeaponBuilderWithHandedness {
        MundaneWeaponBuilderWithHandedness(self.0.worn(), self.1)
    }

    /// Defines the weapon to be one-handed.
    pub fn one_handed(self) -> MundaneWeaponBuilderWithHandedness {
        MundaneWeaponBuilderWithHandedness(self.0.one_handed(), self.1)
    }

    /// Defines the weapon to be two-handed.
    pub fn two_handed(self) -> MundaneWeaponBuilderWithHandedness {
        MundaneWeaponBuilderWithHandedness(self.0.two_handed(), self.1)
    }
}
