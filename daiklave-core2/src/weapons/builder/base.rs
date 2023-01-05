use crate::{book_reference::BookReference, weapons::{WeaponWeightClass, mundane::MundaneWeapon, RangeBand, OtherWeaponTag}};

use super::artifact::BaseArtifactWeaponInsert;

/// A builder for a base weapon, either a base mundane weapon (like "sword")
/// or a base artifact weapon (like "daiklave"). Required fields must be
/// specified in order: name, weight class, handedness, damage type, and
/// primary attack Ability. Optional fields like book reference, weapon ranges,
/// and optional tags can be added at any time prior to the final build.
pub struct BaseWeaponBuilder<'build> {
    name: &'build str
}

impl<'build> BaseWeaponBuilder<'build> {
    /// The book reference for the base weapon. Note that, for artifacts,
    /// this is for the non-unique weapon (like "grand daiklave") not the
    /// page reference of the unique weapon (like "Volcano Cutter").
    pub fn book_reference(self, book_reference: BookReference) -> Self {
        todo!()
    }

    /// Sets the weapon to be usable up to a certain maximum range, using
    /// the Thrown accuracy curve. Note that this does NOT set the weapon to
    /// be usable using the Thrown skill; some unique weapons use the Thrown
    /// accuracy range but are Martial Arts only.
    pub fn thrown_range(self, max_range: RangeBand) -> Self {
        todo!()
    }

    /// Sets the weapon to be usable up to a certain maximum range, using
    /// the Archery accuracy curve. Note that this does NOT set the weapon to
    /// be usable using the Archery skill; some unique weapons use the Archery
    /// accuracy range but are Martial Arts only.
    pub fn archery_range(self, max_range: RangeBand) -> Self {
        todo!()
    }

    /// Adds a tag to the weapon, other than Lethal, Bashing, Brawl, Melee,
    /// Martial Arts, Thrown(range), Archery(range), One-Handed, or Two-Handed.
    /// The relevance of the tag is not enforced--irrelevant tags will be
    /// displayed but may not be mechanically represented.
    pub fn tag(self, tag: OtherWeaponTag) -> Self {
        todo!()
    }

    /// Sets the weapon to be Light, Medium, or Heavy.
    pub fn weight_class(self, weight_class: WeaponWeightClass) -> BaseWeaponBuilderWithWeight<'build> {
        todo!()
    }
}

/// A base weapon builder after specifying weight class.
pub struct BaseWeaponBuilderWithWeight<'build> {
    name: &'build str
}

impl<'build> BaseWeaponBuilderWithWeight<'build> {
    /// The book reference for the base weapon. Note that, for artifacts,
    /// this is for the non-unique weapon (like "grand daiklave") not the
    /// page reference of the unique weapon (like "Volcano Cutter").
    pub fn book_reference(self, book_reference: BookReference) -> Self {
        todo!()
    }

    /// Sets the weapon to be usable up to a certain maximum range, using
    /// the Thrown accuracy curve. Note that this does NOT set the weapon to
    /// be usable using the Thrown skill; some unique weapons use the Thrown
    /// accuracy range but are Martial Arts only.
    pub fn thrown_range(self, max_range: RangeBand) -> Self {
        todo!()
    }

    /// Sets the weapon to be usable up to a certain maximum range, using
    /// the Archery accuracy curve. Note that this does NOT set the weapon to
    /// be usable using the Archery skill; some unique weapons use the Archery
    /// accuracy range but are Martial Arts only.
    pub fn archery_range(self, max_range: RangeBand) -> Self {
        todo!()
    }

    /// Adds a tag to the weapon, other than Lethal, Bashing, Brawl, Melee,
    /// Martial Arts, Thrown(range), Archery(range), One-Handed, or Two-Handed.
    /// The relevance of the tag is not enforced--irrelevant tags will be
    /// displayed but may not be mechanically represented.
    pub fn tag(self, tag: OtherWeaponTag) -> Self {
        todo!()
    }


    /// Defines the weapon to be Natural, part of the wielder's body.
    pub fn natural(self) -> BaseWeaponBuilderWithHandedness<'build> {
        todo!()
    }

    /// Defines the weapon to be Worn, requiring no hands to wield.
    pub fn worn(self) -> BaseWeaponBuilderWithHandedness<'build> {
        todo!()
    }

    /// Defines the weapon to be one-handed.
    pub fn one_handed(self) -> BaseWeaponBuilderWithHandedness<'build> {
        todo!()
    }

    /// Defines the weapon to be two-handed.
    pub fn two_handed(self) -> BaseWeaponBuilderWithHandedness<'build> {
        todo!()
    }
}

/// A weapon builder, after being specified as natural, worn, one-handed,
/// or two-handed.
pub struct BaseWeaponBuilderWithHandedness<'build> {
    name: &'build str
}

impl<'build> BaseWeaponBuilderWithHandedness<'build> {
    /// The book reference for the base weapon. Note that, for artifacts,
    /// this is for the non-unique weapon (like "grand daiklave") not the
    /// page reference of the unique weapon (like "Volcano Cutter").
    pub fn book_reference(self, book_reference: BookReference) -> Self {
        todo!()
    }

    /// Sets the weapon to be usable up to a certain maximum range, using
    /// the Thrown accuracy curve. Note that this does NOT set the weapon to
    /// be usable using the Thrown skill; some unique weapons use the Thrown
    /// accuracy range but are Martial Arts only.
    pub fn thrown_range(self, max_range: RangeBand) -> Self {
        todo!()
    }

    /// Sets the weapon to be usable up to a certain maximum range, using
    /// the Archery accuracy curve. Note that this does NOT set the weapon to
    /// be usable using the Archery skill; some unique weapons use the Archery
    /// accuracy range but are Martial Arts only.
    pub fn archery_range(self, max_range: RangeBand) -> Self {
        todo!()
    }

    /// Adds a tag to the weapon, other than Lethal, Bashing, Brawl, Melee,
    /// Martial Arts, Thrown(range), Archery(range), One-Handed, or Two-Handed.
    /// The relevance of the tag is not enforced--irrelevant tags will be
    /// displayed but may not be mechanically represented.
    pub fn tag(self, tag: OtherWeaponTag) -> Self {
        todo!()
    }

    /// Sets the weapon to deal Lethal damage by default. Typical for bladed or
    /// piercing weapons.
    pub fn lethal(self) -> BaseWeaponBuilderWithDamageType<'build> {
        todo!()
    }

    /// Sets the weapon to deal Bashing damage by default. Typical for blunt
    /// weapons.
    pub fn bashing(self) -> BaseWeaponBuilderWithDamageType<'build> {
        todo!()
    }
}

/// A base weapon builder after having its damage type specified.
pub struct BaseWeaponBuilderWithDamageType<'build> {
    name: &'build str
}

impl<'build> BaseWeaponBuilderWithDamageType<'build> {
    /// The book reference for the base weapon. Note that, for artifacts,
    /// this is for the non-unique weapon (like "grand daiklave") not the
    /// page reference of the unique weapon (like "Volcano Cutter").
    pub fn book_reference(self, book_reference: BookReference) -> Self {
        todo!()
    }

    /// Sets the weapon to be usable up to a certain maximum range, using
    /// the Thrown accuracy curve. Note that this does NOT set the weapon to
    /// be usable using the Thrown skill; some unique weapons use the Thrown
    /// accuracy range but are Martial Arts only.
    pub fn thrown_range(self, max_range: RangeBand) -> Self {
        todo!()
    }

    /// Sets the weapon to be usable up to a certain maximum range, using
    /// the Archery accuracy curve. Note that this does NOT set the weapon to
    /// be usable using the Archery skill; some unique weapons use the Archery
    /// accuracy range but are Martial Arts only.
    pub fn archery_range(self, max_range: RangeBand) -> Self {
        todo!()
    }

    /// Adds a tag to the weapon, other than Lethal, Bashing, Brawl, Melee,
    /// Martial Arts, Thrown(range), Archery(range), One-Handed, or Two-Handed.
    /// The relevance of the tag is not enforced--irrelevant tags will be
    /// displayed but may not be mechanically represented.
    pub fn tag(self, tag: OtherWeaponTag) -> Self {
        todo!()
    }

    /// Sets the weapon to be usable with the Brawl skill at close range. May
    /// also be used with applicable Martial Arts styles. If the weapon has a
    /// range definition (uncommon; most Brawl weapons are melee-only), will 
    /// use either Thrown or Archery (or an applicable Martial Art). This also
    /// allows the weapon to be used to parry.
    pub fn brawl(self) -> BaseWeaponBuilderWithAttack<'build> {
        todo!()
    }

    /// Sets the weapon to be usable with the Melee skill at close range. May
    /// also be used with applicable Martial Arts styles. If the weapon has a
    /// range definition, will use either Thrown or Archery (or Martial Arts).
    /// Melee + Thrown is substantially more common than Melee + Archery. This
    /// also allows the weapon to be used to parry.
    pub fn melee(self) -> BaseWeaponBuilderWithAttack<'build> {
        todo!()
    }

    /// Sets the weapon to be usable with the Archery skill. May also be used
    /// with applicable Martial Arts styles (such as Righteous Devil style).
    /// Note that this does not give the weapon any range characteristics; use
    /// .archery_range() to specify its range. Archery weapons cannot be used
    /// to parry and cannot be used for melee attacks. If a weapon can be used
    /// for both melee and archery attacks (uncommon), use 
    /// .melee().archery_range() instead.
    pub fn archery(self) -> BaseWeaponBuilderWithAttack<'build> {
        todo!()
    }

    /// Sets the weapon to be usable with the Thrown skill (or applicable 
    /// Martial Arts) **only**. Note that this does not give the weapon any 
    /// range characteristics; use.thrown_range() to specify its range. 
    /// Thrown-only weapons cannot be used to parry and cannot be used in 
    /// melee. If a weapon is both melee and thrown, use 
    /// .melee().thrown_range() instead.
    pub fn thrown(self) -> BaseWeaponBuilderWithAttack<'build> {
        todo!()
    }

    /// Sets the weapon to be usable with the Martial Arts skill **only**.
    /// Martial Arts weapons are usable in melee and can be used to parry.
    /// By default, Martial Arts weapons are not usable at range, but this
    /// can be adjusted with the .thrown_range() or rarely the .archery_range()
    /// methods. (In the very rare case that there is a weapon that is not
    /// usable to parry, this should be modeled as a unique .thrown() weapon.)
    pub fn martial_arts(self) -> BaseWeaponBuilderWithAttack<'build> {
        todo!()
    }
}

/// A base weapon builder after the primary attack skill is specified.
pub struct BaseWeaponBuilderWithAttack<'build> {
    name: &'build str
}

impl<'build> BaseWeaponBuilderWithAttack<'build> {
    /// The book reference for the base weapon. Note that, for artifacts,
    /// this is for the non-unique weapon (like "grand daiklave") not the
    /// page reference of the unique weapon (like "Volcano Cutter").
    pub fn book_reference(self, book_reference: BookReference) -> Self {
        todo!()
    }

    /// Sets the weapon to be usable up to a certain maximum range, using
    /// the Thrown accuracy curve. Note that this does NOT set the weapon to
    /// be usable using the Thrown skill; some unique weapons use the Thrown
    /// accuracy range but are Martial Arts only.
    pub fn thrown_range(self, max_range: RangeBand) -> Self {
        todo!()
    }

    /// Sets the weapon to be usable up to a certain maximum range, using
    /// the Archery accuracy curve. Note that this does NOT set the weapon to
    /// be usable using the Archery skill; some unique weapons use the Archery
    /// accuracy range but are Martial Arts only.
    pub fn archery_range(self, max_range: RangeBand) -> Self {
        todo!()
    }

    /// Adds a tag to the weapon, other than Lethal, Bashing, Brawl, Melee,
    /// Martial Arts, Thrown(range), Archery(range), One-Handed, or Two-Handed.
    /// The relevance of the tag is not enforced--irrelevant tags will be
    /// displayed but may not be mechanically represented.
    pub fn tag(self, tag: OtherWeaponTag) -> Self {
        todo!()
    }

    /// Completes the builder process, returning a new MundaneWeapon. This is
    /// a borrowed copy but can be immediately memoized with .as_memo() if 
    /// needed.
    pub fn build_mundane(self) -> MundaneWeapon<'build> {
        todo!()
    }

    /// Completes the builder process, returning a new 
    /// BaseArtifactWeaponInsert. 
    pub fn build_artifact(self) -> BaseArtifactWeaponInsert<'build> {
        todo!()
    }
}