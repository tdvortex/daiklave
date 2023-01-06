use std::collections::HashSet;

use crate::{book_reference::BookReference, weapons::{WeaponWeightClass, mundane::{NaturalMundaneWeaponMemo, WornMundaneWeaponMemo, OneHandedMundaneWeaponMemo, TwoHandedMundaneWeaponMemo}, RangeBand, OptionalWeaponTag, range::WeaponRange, ability::WeaponAbility, base::{BaseWeaponMemo}, damage_type::WeaponDamageType, MundaneWeaponMemo}};

use super::{artifact::BaseArtifactWeaponInsert, handedness::WeaponHandedness};

/// A builder for a base weapon, either a base mundane weapon (like "sword")
/// or a base artifact weapon (like "daiklave"). Required fields must be
/// specified in order: name, weight class, handedness, damage type, and
/// primary attack Ability. Optional fields like book reference, weapon ranges,
/// and optional tags can be added at any time prior to the final build.
pub struct BaseWeaponBuilder {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) attack_range: WeaponRange,
    pub(crate) tags: HashSet<OptionalWeaponTag>,
}

impl BaseWeaponBuilder {
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

    /// Sets the weapon to be Light, Medium, or Heavy.
    pub fn weight_class(self, weight_class: WeaponWeightClass) -> BaseWeaponBuilderWithWeight {
        BaseWeaponBuilderWithWeight { 
            name: self.name,
            book_reference: self.book_reference,
            attack_range: self.attack_range,
            tags: self.tags,
            weight_class 
        }
    }
}

/// A base weapon builder after specifying weight class.
pub struct BaseWeaponBuilderWithWeight {
    name: String,
    book_reference: Option<BookReference>,
    attack_range: WeaponRange,
    tags: HashSet<OptionalWeaponTag>,
    weight_class: WeaponWeightClass,
}

impl BaseWeaponBuilderWithWeight {
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


    /// Defines the weapon to be Natural, part of the wielder's body.
    pub fn natural(self) -> BaseWeaponBuilderWithHandedness {
        BaseWeaponBuilderWithHandedness { 
            name: self.name, 
            book_reference: self.book_reference, 
            attack_range: self.attack_range, 
            tags: self.tags, 
            weight_class: self.weight_class,
            handedness: WeaponHandedness::Natural,
        }
    }

    /// Defines the weapon to be Worn, requiring no hands to wield.
    pub fn worn(self) -> BaseWeaponBuilderWithHandedness {
        BaseWeaponBuilderWithHandedness { 
            name: self.name, 
            book_reference: self.book_reference, 
            attack_range: self.attack_range, 
            tags: self.tags, 
            weight_class: self.weight_class,
            handedness: WeaponHandedness::Worn,
        }
    }

    /// Defines the weapon to be one-handed.
    pub fn one_handed(self) -> BaseWeaponBuilderWithHandedness {
        BaseWeaponBuilderWithHandedness { 
            name: self.name, 
            book_reference: self.book_reference, 
            attack_range: self.attack_range, 
            tags: self.tags, 
            weight_class: self.weight_class,
            handedness: WeaponHandedness::OneHanded,
        }
    }

    /// Defines the weapon to be two-handed.
    pub fn two_handed(self) -> BaseWeaponBuilderWithHandedness {
        BaseWeaponBuilderWithHandedness { 
            name: self.name, 
            book_reference: self.book_reference, 
            attack_range: self.attack_range, 
            tags: self.tags, 
            weight_class: self.weight_class,
            handedness: WeaponHandedness::TwoHanded,
        }
    }
}

/// A weapon builder, after being specified as natural, worn, one-handed,
/// or two-handed.
pub struct BaseWeaponBuilderWithHandedness {
    name: String,
    book_reference: Option<BookReference>,
    attack_range: WeaponRange,
    tags: HashSet<OptionalWeaponTag>,
    weight_class: WeaponWeightClass,
    handedness: WeaponHandedness,
}

impl<'build> BaseWeaponBuilderWithHandedness {
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
            damage_type: WeaponDamageType::Lethal
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
            damage_type: WeaponDamageType::Bashing
        }
    }
}

/// A base weapon builder after having its damage type specified.
pub struct BaseWeaponBuilderWithDamageType {
    name: String,
    book_reference: Option<BookReference>,
    attack_range: WeaponRange,
    tags: HashSet<OptionalWeaponTag>,
    weight_class: WeaponWeightClass,
    handedness: WeaponHandedness,
    damage_type: WeaponDamageType,
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

/// A base weapon builder after the primary attack skill is specified.
pub struct BaseWeaponBuilderWithAttack {
    name: String,
    book_reference: Option<BookReference>,
    attack_range: WeaponRange,
    tags: HashSet<OptionalWeaponTag>,
    weight_class: WeaponWeightClass,
    handedness: WeaponHandedness,
    damage_type: WeaponDamageType,
    primary_attack: WeaponAbility,
}

impl<'build> BaseWeaponBuilderWithAttack {
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
    pub fn build_mundane(self) -> MundaneWeaponMemo {
        match self.handedness {
            WeaponHandedness::Natural => MundaneWeaponMemo::Natural(NaturalMundaneWeaponMemo(BaseWeaponMemo {
                name: self.name,
                book_reference: self.book_reference,
                weight_class: self.weight_class,
                range_bands: self.attack_range,
                primary_ability: self.primary_attack,
                damage_type: self.damage_type,
                tags: self.tags,
            })),
            WeaponHandedness::Worn => MundaneWeaponMemo::Worn(WornMundaneWeaponMemo(BaseWeaponMemo{
                name: self.name,
                book_reference: self.book_reference,
                weight_class: self.weight_class,
                range_bands: self.attack_range,
                primary_ability: self.primary_attack,
                damage_type: self.damage_type,
                tags: self.tags,
            }), false),
            WeaponHandedness::OneHanded => MundaneWeaponMemo::OneHanded(OneHandedMundaneWeaponMemo(BaseWeaponMemo {
                name: self.name,
                book_reference: self.book_reference,
                weight_class: self.weight_class,
                range_bands: self.attack_range,
                primary_ability: self.primary_attack,
                damage_type: self.damage_type,
                tags: self.tags,
            }), None),
            WeaponHandedness::TwoHanded => MundaneWeaponMemo::TwoHanded(TwoHandedMundaneWeaponMemo(BaseWeaponMemo {
                name: self.name,
                book_reference: self.book_reference,
                weight_class: self.weight_class,
                range_bands: self.attack_range,
                primary_ability: self.primary_attack,
                damage_type: self.damage_type,
                tags: self.tags,
            }), false),
        }
    }

    /// Completes the builder process, returning a new 
    /// BaseArtifactWeaponInsert. 
    pub fn build_artifact(self) -> BaseArtifactWeaponInsert<'build> {
        todo!()
    }
}