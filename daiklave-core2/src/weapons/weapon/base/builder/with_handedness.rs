use std::{collections::HashSet, num::NonZeroU8};

use crate::{
    book_reference::BookReference,
    weapons::weapon::{
        damage_type::WeaponDamageType,
        handedness::WeaponHandedness,
        range::{RangeBand, WeaponRange},
        tag::OptionalWeaponTag,
        weight_class::WeaponWeightClass,
    },
};

use super::{with_damage_type::BaseWeaponBuilderWithDamageType, mundane::MundaneWeaponBuilderWithHandedness, BaseArtifactWeaponBuilderWithHandedness};

/// A weapon builder, after being specified as natural, worn, one-handed,
/// or two-handed.
pub struct BaseWeaponBuilderWithHandedness {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) attack_range: WeaponRange,
    pub(crate) tags: HashSet<OptionalWeaponTag>,
    pub(crate) weight_class: WeaponWeightClass,
    pub(crate) handedness: WeaponHandedness,
}

impl BaseWeaponBuilderWithHandedness {
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

    /// Sets the weapon to be a mundane weapon.
    pub fn mundane(self) -> MundaneWeaponBuilderWithHandedness {
        MundaneWeaponBuilderWithHandedness(self, NonZeroU8::new(1).unwrap())
    }

    /// Sets the weapon to be a base artifact weapon.
    pub fn artifact(self) -> BaseArtifactWeaponBuilderWithHandedness {
        BaseArtifactWeaponBuilderWithHandedness(self)
    }

    /// Sets the weapon to deal Lethal damage by default. Typical for bladed or
    /// piercing weapons.
    pub fn lethal(self) -> BaseWeaponBuilderWithDamageType {
        BaseWeaponBuilderWithDamageType {
            name: self.name,
            book_reference: self.book_reference,
            attack_range: self.attack_range,
            tags: self.tags,
            weight_class: self.weight_class,
            handedness: self.handedness,
            damage_type: WeaponDamageType::Lethal,
        }
    }

    /// Sets the weapon to deal Bashing damage by default. Typical for blunt
    /// weapons.
    pub fn bashing(self) -> BaseWeaponBuilderWithDamageType {
        BaseWeaponBuilderWithDamageType {
            name: self.name,
            book_reference: self.book_reference,
            attack_range: self.attack_range,
            tags: self.tags,
            weight_class: self.weight_class,
            handedness: self.handedness,
            damage_type: WeaponDamageType::Bashing,
        }
    }
}
