use crate::{
    book_reference::BookReference,
    weapons::weapon::{
        base::BaseWeapon,
        builder::base::BaseWeaponBuilderWithAttack,
        OptionalWeaponTag, RangeBand, artifact::{AddBaseArtifactWeapon, BaseArtifactWeapon},
    },
};

/// A builder for a base artifact weapon, after the attack type has been specified.
pub struct BaseArtifactWeaponBuilderWithAttack(pub(crate) BaseWeaponBuilderWithAttack);

impl BaseArtifactWeaponBuilderWithAttack {
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

    /// Completes the builder process, returning a new
    /// AddBaseArtifactWeapon.
    pub fn build(self) -> AddBaseArtifactWeapon {
        AddBaseArtifactWeapon {
            name: self.0.name,
            weapon: BaseArtifactWeapon {
                handedness: self.0.handedness,
                base_weapon: BaseWeapon {
                    book_reference: self.0.book_reference,
                    weight_class: self.0.weight_class,
                    range_bands: self.0.attack_range,
                    primary_ability: self.0.primary_attack,
                    damage_type: self.0.damage_type,
                    tags: self.0.tags,
                },
            },
        }
    }
}
