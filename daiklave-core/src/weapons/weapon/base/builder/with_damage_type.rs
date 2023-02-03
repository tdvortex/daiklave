use std::{collections::HashSet, num::NonZeroU8};

use crate::{
    book_reference::BookReference,
    weapons::weapon::{
        ability::WeaponAbility,
        damage_type::WeaponDamageType,
        handedness::WeaponHandedness,
        range::{RangeBand, WeaponRange},
        tag::OptionalWeaponTag,
        weight_class::WeaponWeightClass,
    },
};

use super::{
    mundane::MundaneWeaponBuilderWithDamageType, with_attack::BaseWeaponBuilderWithAttack,
    BaseArtifactWeaponBuilderWithDamageType,
};

/// A base weapon builder after having its damage type specified.
pub struct BaseWeaponBuilderWithDamageType {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) attack_range: WeaponRange,
    pub(crate) tags: HashSet<OptionalWeaponTag>,
    pub(crate) weight_class: WeaponWeightClass,
    pub(crate) handedness: WeaponHandedness,
    pub(crate) damage_type: WeaponDamageType,
}

impl BaseWeaponBuilderWithDamageType {
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
    pub fn mundane(self) -> MundaneWeaponBuilderWithDamageType {
        MundaneWeaponBuilderWithDamageType(self, NonZeroU8::new(1).unwrap())
    }

    /// Sets the weapon to be a base artifact weapon.
    pub fn artifact(self) -> BaseArtifactWeaponBuilderWithDamageType {
        BaseArtifactWeaponBuilderWithDamageType(self)
    }

    /// Sets the weapon to be usable with the Brawl skill at close range. May
    /// also be used with applicable Martial Arts styles. If the weapon has a
    /// range definition (uncommon; most Brawl weapons are melee-only), will
    /// use either Thrown or Archery (or an applicable Martial Art). This also
    /// allows the weapon to be used to parry.
    pub fn brawl(self) -> BaseWeaponBuilderWithAttack {
        BaseWeaponBuilderWithAttack {
            name: self.name,
            book_reference: self.book_reference,
            attack_range: self.attack_range,
            tags: self.tags,
            primary_attack: WeaponAbility::Brawl,
            weight_class: self.weight_class,
            handedness: self.handedness,
            damage_type: self.damage_type,
        }
    }

    /// Sets the weapon to be usable with the Melee skill at close range. May
    /// also be used with applicable Martial Arts styles. If the weapon has a
    /// range definition, will use either Thrown or Archery (or Martial Arts).
    /// Melee + Thrown is substantially more common than Melee + Archery. This
    /// also allows the weapon to be used to parry.
    pub fn melee(self) -> BaseWeaponBuilderWithAttack {
        BaseWeaponBuilderWithAttack {
            name: self.name,
            book_reference: self.book_reference,
            attack_range: self.attack_range,
            tags: self.tags,
            primary_attack: WeaponAbility::Melee,
            weight_class: self.weight_class,
            handedness: self.handedness,
            damage_type: self.damage_type,
        }
    }

    /// Sets the weapon to be usable with the Archery skill. May also be used
    /// with applicable Martial Arts styles (such as Righteous Devil style).
    /// Note that this does not give the weapon any range characteristics; use
    /// .archery_range() to specify its range. Archery weapons cannot be used
    /// to parry and cannot be used for melee attacks. If a weapon can be used
    /// for both melee and archery attacks (uncommon), use
    /// .melee().archery_range() instead.
    pub fn archery(self) -> BaseWeaponBuilderWithAttack {
        BaseWeaponBuilderWithAttack {
            name: self.name,
            book_reference: self.book_reference,
            attack_range: self.attack_range,
            tags: self.tags,
            primary_attack: WeaponAbility::Archery,
            weight_class: self.weight_class,
            handedness: self.handedness,
            damage_type: self.damage_type,
        }
    }

    /// Sets the weapon to be usable with the Thrown skill (or applicable
    /// Martial Arts) **only**. Note that this does not give the weapon any
    /// range characteristics; use.thrown_range() to specify its range.
    /// Thrown-only weapons cannot be used to parry and cannot be used in
    /// melee. If a weapon is both melee and thrown, use
    /// .melee().thrown_range() instead.
    pub fn thrown(self) -> BaseWeaponBuilderWithAttack {
        BaseWeaponBuilderWithAttack {
            name: self.name,
            book_reference: self.book_reference,
            attack_range: self.attack_range,
            tags: self.tags,
            primary_attack: WeaponAbility::Thrown,
            weight_class: self.weight_class,
            handedness: self.handedness,
            damage_type: self.damage_type,
        }
    }

    /// Sets the weapon to be usable with the Martial Arts skill **only**.
    /// Martial Arts weapons are usable in melee and can be used to parry.
    /// By default, Martial Arts weapons are not usable at range, but this
    /// can be adjusted with the .thrown_range() or rarely the .archery_range()
    /// methods. (In the very rare case that there is a weapon that is not
    /// usable to parry, this should be modeled as a unique .thrown() weapon.)
    pub fn martial_arts(self) -> BaseWeaponBuilderWithAttack {
        BaseWeaponBuilderWithAttack {
            name: self.name,
            book_reference: self.book_reference,
            attack_range: self.attack_range,
            tags: self.tags,
            primary_attack: WeaponAbility::MartialArts,
            weight_class: self.weight_class,
            handedness: self.handedness,
            damage_type: self.damage_type,
        }
    }
}
