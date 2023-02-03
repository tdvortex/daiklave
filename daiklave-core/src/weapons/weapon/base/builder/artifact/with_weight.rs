use crate::{
    book_reference::BookReference,
    weapons::weapon::{builder::base::BaseWeaponBuilderWithWeight, OptionalWeaponTag, RangeBand},
};

use super::BaseArtifactWeaponBuilderWithHandedness;

/// A builder for a base artifact weapon, after the weight class has been specified.
pub struct BaseArtifactWeaponBuilderWithWeight(pub(crate) BaseWeaponBuilderWithWeight);

impl BaseArtifactWeaponBuilderWithWeight {
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

    /// Defines the weapon to be Natural, part of the wielder's body.
    pub fn natural(self) -> BaseArtifactWeaponBuilderWithHandedness {
        BaseArtifactWeaponBuilderWithHandedness(self.0.natural())
    }

    /// Defines the weapon to be Worn, requiring no hands to wield.
    pub fn worn(self) -> BaseArtifactWeaponBuilderWithHandedness {
        BaseArtifactWeaponBuilderWithHandedness(self.0.worn())
    }

    /// Defines the weapon to be one-handed.
    pub fn one_handed(self) -> BaseArtifactWeaponBuilderWithHandedness {
        BaseArtifactWeaponBuilderWithHandedness(self.0.one_handed())
    }

    /// Defines the weapon to be two-handed.
    pub fn two_handed(self) -> BaseArtifactWeaponBuilderWithHandedness {
        BaseArtifactWeaponBuilderWithHandedness(self.0.two_handed())
    }
}
