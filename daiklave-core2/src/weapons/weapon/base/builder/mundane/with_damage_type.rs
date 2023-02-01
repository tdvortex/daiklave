use std::num::NonZeroU8;

use crate::{weapons::weapon::{builder::base::BaseWeaponBuilderWithDamageType, RangeBand, OptionalWeaponTag}, book_reference::BookReference};

use super::MundaneWeaponBuilderWithAttack;

/// A mundane weapon builder after the damage type has been specified.
pub struct MundaneWeaponBuilderWithDamageType(pub(crate) BaseWeaponBuilderWithDamageType, pub(crate) NonZeroU8);

impl MundaneWeaponBuilderWithDamageType {
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

    /// Sets the weapon to be usable with the Brawl skill at close range. May
    /// also be used with applicable Martial Arts styles. If the weapon has a
    /// range definition (uncommon; most Brawl weapons are melee-only), will
    /// use either Thrown or Archery (or an applicable Martial Art). This also
    /// allows the weapon to be used to parry.
    pub fn brawl(self) -> MundaneWeaponBuilderWithAttack {
        MundaneWeaponBuilderWithAttack(self.0.brawl(), self.1)
    }

    /// Sets the weapon to be usable with the Melee skill at close range. May
    /// also be used with applicable Martial Arts styles. If the weapon has a
    /// range definition, will use either Thrown or Archery (or Martial Arts).
    /// Melee + Thrown is substantially more common than Melee + Archery. This
    /// also allows the weapon to be used to parry.
    pub fn melee(self) -> MundaneWeaponBuilderWithAttack {
        MundaneWeaponBuilderWithAttack(self.0.melee(), self.1)
    }

    /// Sets the weapon to be usable with the Archery skill. May also be used
    /// with applicable Martial Arts styles (such as Righteous Devil style).
    /// Note that this does not give the weapon any range characteristics; use
    /// .archery_range() to specify its range. Archery weapons cannot be used
    /// to parry and cannot be used for melee attacks. If a weapon can be used
    /// for both melee and archery attacks (uncommon), use
    /// .melee().archery_range() instead.
    pub fn archery(self) -> MundaneWeaponBuilderWithAttack {
        MundaneWeaponBuilderWithAttack(self.0.archery(), self.1)
    }

    /// Sets the weapon to be usable with the Thrown skill (or applicable
    /// Martial Arts) **only**. Note that this does not give the weapon any
    /// range characteristics; use.thrown_range() to specify its range.
    /// Thrown-only weapons cannot be used to parry and cannot be used in
    /// melee. If a weapon is both melee and thrown, use
    /// .melee().thrown_range() instead.
    pub fn thrown(self) -> MundaneWeaponBuilderWithAttack {
        MundaneWeaponBuilderWithAttack(self.0.thrown(), self.1)
    }

    /// Sets the weapon to be usable with the Martial Arts skill **only**.
    /// Martial Arts weapons are usable in melee and can be used to parry.
    /// By default, Martial Arts weapons are not usable at range, but this
    /// can be adjusted with the .thrown_range() or rarely the .archery_range()
    /// methods. (In the very rare case that there is a weapon that is not
    /// usable to parry, this should be modeled as a unique .thrown() weapon.)
    pub fn martial_arts(self) -> MundaneWeaponBuilderWithAttack {
        MundaneWeaponBuilderWithAttack(self.0.martial_arts(), self.1)
    }

}
