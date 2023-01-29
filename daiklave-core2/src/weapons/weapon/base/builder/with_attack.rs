use std::{collections::HashSet, num::NonZeroU8};

use crate::{
    book_reference::BookReference,
    weapons::weapon::{
        ability::WeaponAbility,
        artifact::{AddBaseArtifactWeapon, BaseArtifactWeapon},
        base::BaseWeapon,
        damage_type::WeaponDamageType,
        handedness::WeaponHandedness,
        mundane::{
            MundaneWeapon, MundaneWeaponHandedness, NaturalMundaneWeapon, OneHandedMundaneWeaponMemo,
            TwoHandedMundaneWeapon, WornMundaneWeapon, AddMundaneWeapon,
        },
        range::{RangeBand, WeaponRange},
        tag::OptionalWeaponTag,
        weight_class::WeaponWeightClass,
    },
};

/// A base weapon builder after the primary attack skill is specified.
pub struct BaseWeaponBuilderWithAttack {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) attack_range: WeaponRange,
    pub(crate) tags: HashSet<OptionalWeaponTag>,
    pub(crate) weight_class: WeaponWeightClass,
    pub(crate) handedness: WeaponHandedness,
    pub(crate) damage_type: WeaponDamageType,
    pub(crate) primary_attack: WeaponAbility,
}

impl BaseWeaponBuilderWithAttack {
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

    /// Completes the builder process, returning a new AddMundaneWeapon. This
    /// defaults to a single copy, but can be edited to insert multiple copies
    /// simultaneously.
    pub fn build_mundane(self) -> AddMundaneWeapon {
        match self.handedness {
            WeaponHandedness::Natural => AddMundaneWeapon {
                name: self.name.into(),
                weapon: MundaneWeapon(MundaneWeaponHandedness::Natural(NaturalMundaneWeapon(
                    BaseWeapon {
                        book_reference: self.book_reference,
                        weight_class: self.weight_class,
                        range_bands: self.attack_range,
                        primary_ability: self.primary_attack,
                        damage_type: self.damage_type,
                        tags: self.tags,
                    },
                ))),
                quantity: NonZeroU8::new(1).unwrap(),
            },
            WeaponHandedness::Worn => AddMundaneWeapon {
                name: self.name.into(),
                weapon: MundaneWeapon(MundaneWeaponHandedness::Worn(
                    WornMundaneWeapon(BaseWeapon {
                        book_reference: self.book_reference,
                        weight_class: self.weight_class,
                        range_bands: self.attack_range,
                        primary_ability: self.primary_attack,
                        damage_type: self.damage_type,
                        tags: self.tags,
                    }),
                    false,
                )),
                quantity: NonZeroU8::new(1).unwrap(),
            },
            WeaponHandedness::OneHanded => AddMundaneWeapon {
                name: self.name.into(),
                weapon: MundaneWeapon(MundaneWeaponHandedness::OneHanded(
                    OneHandedMundaneWeaponMemo(BaseWeapon {
                        book_reference: self.book_reference,
                        weight_class: self.weight_class,
                        range_bands: self.attack_range,
                        primary_ability: self.primary_attack,
                        damage_type: self.damage_type,
                        tags: self.tags,
                    }),
                    None,
                )),
                quantity: NonZeroU8::new(1).unwrap(),
            },
            WeaponHandedness::TwoHanded => AddMundaneWeapon {
                name: self.name.into(),
                weapon: MundaneWeapon(MundaneWeaponHandedness::TwoHanded(
                    TwoHandedMundaneWeapon(BaseWeapon {
                        book_reference: self.book_reference,
                        weight_class: self.weight_class,
                        range_bands: self.attack_range,
                        primary_ability: self.primary_attack,
                        damage_type: self.damage_type,
                        tags: self.tags,
                    }),
                    false,
                )),
                quantity: NonZeroU8::new(1).unwrap(),
            },
        }
    }

    /// Completes the builder process, returning a new
    /// AddBaseArtifactWeapon.
    pub fn build_artifact(self) -> AddBaseArtifactWeapon {
        AddBaseArtifactWeapon {
            name: self.name,
            weapon: BaseArtifactWeapon {
                handedness: self.handedness,
                base_weapon: BaseWeapon {
                    book_reference: self.book_reference,
                    weight_class: self.weight_class,
                    range_bands: self.attack_range,
                    primary_ability: self.primary_attack,
                    damage_type: self.damage_type,
                    tags: self.tags,
                }
            },
        }
    }
}
