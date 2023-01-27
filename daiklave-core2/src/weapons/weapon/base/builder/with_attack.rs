use std::collections::HashSet;

use crate::{
    book_reference::BookReference,
    weapons::weapon::{
        ability::WeaponAbility,
        artifact::{BaseArtifactWeapon, AddBaseArtifactWeapon},
        base::BaseWeapon,
        damage_type::WeaponDamageType,
        handedness::WeaponHandedness,
        mundane::{
            MundaneWeapon, MundaneWeaponHandedness, NaturalMundaneWeapon, OneHandedMundaneWeapon,
            TwoHandedMundaneWeapon, WornMundaneWeapon,
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

    /// Completes the builder process, returning a new MundaneWeapon. This is
    /// a borrowed copy but can be immediately memoized with .as_memo() if
    /// needed.
    pub fn build_mundane(self) -> (String, MundaneWeapon) {
        match self.handedness {
            WeaponHandedness::Natural => (self.name, MundaneWeapon(MundaneWeaponHandedness::Natural(
                NaturalMundaneWeapon(BaseWeapon {
                    book_reference: self.book_reference,
                    weight_class: self.weight_class,
                    range_bands: self.attack_range,
                    primary_ability: self.primary_attack,
                    damage_type: self.damage_type,
                    tags: self.tags,
                })),
            )),
            WeaponHandedness::Worn => (self.name, MundaneWeapon(MundaneWeaponHandedness::Worn(
                WornMundaneWeapon(BaseWeapon {
                    book_reference: self.book_reference,
                    weight_class: self.weight_class,
                    range_bands: self.attack_range,
                    primary_ability: self.primary_attack,
                    damage_type: self.damage_type,
                    tags: self.tags,
                }),
                false),
            )),
            WeaponHandedness::OneHanded => (self.name, MundaneWeapon(MundaneWeaponHandedness::OneHanded(
                OneHandedMundaneWeapon(BaseWeapon {
                    book_reference: self.book_reference,
                    weight_class: self.weight_class,
                    range_bands: self.attack_range,
                    primary_ability: self.primary_attack,
                    damage_type: self.damage_type,
                    tags: self.tags,
                }),
                None),
            )),
            WeaponHandedness::TwoHanded => (self.name, MundaneWeapon(MundaneWeaponHandedness::TwoHanded(
                TwoHandedMundaneWeapon(BaseWeapon {
                    book_reference: self.book_reference,
                    weight_class: self.weight_class,
                    range_bands: self.attack_range,
                    primary_ability: self.primary_attack,
                    damage_type: self.damage_type,
                    tags: self.tags,
                }),
                false),
            )),
        }
    }

    /// Completes the builder process, returning a new
    /// BaseArtifactWeapon.
    pub fn build_artifact(self) -> AddBaseArtifactWeapon {
        let base_weapon = BaseWeapon {
            book_reference: self.book_reference,
            weight_class: self.weight_class,
            range_bands: self.attack_range,
            primary_ability: self.primary_attack,
            damage_type: self.damage_type,
            tags: self.tags,
        };

        (self.name, BaseArtifactWeapon {
            handedness: self.handedness,
            base_weapon,
        })
    }
}
