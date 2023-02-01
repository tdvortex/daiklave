use std::num::NonZeroU8;

use crate::{weapons::weapon::{builder::base::BaseWeaponBuilderWithAttack, RangeBand, OptionalWeaponTag, mundane::{AddMundaneWeapon, MundaneWeapon, MundaneWeaponHandedness, NaturalMundaneWeapon, WornMundaneWeapon, OneHandedMundaneWeaponMemo, TwoHandedMundaneWeapon}, handedness::WeaponHandedness, base::BaseWeapon}, book_reference::BookReference};

/// A mundane weapon builder after the attack ability has been specified.
pub struct MundaneWeaponBuilderWithAttack(pub(crate) BaseWeaponBuilderWithAttack, pub(crate) NonZeroU8);

impl MundaneWeaponBuilderWithAttack {
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

    /// Completes the builder process, returning a new AddMundaneWeapon. 
    pub fn build_mundane(self) -> AddMundaneWeapon {
        match self.0.handedness {
            WeaponHandedness::Natural => AddMundaneWeapon {
                name: self.0.name.into(),
                weapon: MundaneWeapon(MundaneWeaponHandedness::Natural(NaturalMundaneWeapon(
                    BaseWeapon {
                        book_reference: self.0.book_reference,
                        weight_class: self.0.weight_class,
                        range_bands: self.0.attack_range,
                        primary_ability: self.0.primary_attack,
                        damage_type: self.0.damage_type,
                        tags: self.0.tags,
                    },
                ))),
                quantity: self.1,
            },
            WeaponHandedness::Worn => AddMundaneWeapon {
                name: self.0.name.into(),
                weapon: MundaneWeapon(MundaneWeaponHandedness::Worn(
                    WornMundaneWeapon(BaseWeapon {
                        book_reference: self.0.book_reference,
                        weight_class: self.0.weight_class,
                        range_bands: self.0.attack_range,
                        primary_ability: self.0.primary_attack,
                        damage_type: self.0.damage_type,
                        tags: self.0.tags,
                    }),
                    false,
                )),
                quantity: self.1,
            },
            WeaponHandedness::OneHanded => AddMundaneWeapon {
                name: self.0.name.into(),
                weapon: MundaneWeapon(MundaneWeaponHandedness::OneHanded(
                    OneHandedMundaneWeaponMemo(BaseWeapon {
                        book_reference: self.0.book_reference,
                        weight_class: self.0.weight_class,
                        range_bands: self.0.attack_range,
                        primary_ability: self.0.primary_attack,
                        damage_type: self.0.damage_type,
                        tags: self.0.tags,
                    }),
                    None,
                )),
                quantity: self.1,
            },
            WeaponHandedness::TwoHanded => AddMundaneWeapon {
                name: self.0.name.into(),
                weapon: MundaneWeapon(MundaneWeaponHandedness::TwoHanded(
                    TwoHandedMundaneWeapon(BaseWeapon {
                        book_reference: self.0.book_reference,
                        weight_class: self.0.weight_class,
                        range_bands: self.0.attack_range,
                        primary_ability: self.0.primary_attack,
                        damage_type: self.0.damage_type,
                        tags: self.0.tags,
                    }),
                    false,
                )),
                quantity: self.1,
            },
        }
    }

}