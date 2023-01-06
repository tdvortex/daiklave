use serde::{Deserialize, Serialize};

use super::range::RangeBand;

/// Optional weapon tags. Weapons may have some or none of these.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum OptionalWeaponTag {
    /// Increases the Overwhelming rating by 1.
    Balanced,
    /// Enables a Chopping attack.
    Chopping,
    /// Can be concealed.
    Concealable,
    /// Weapon damage pools are calculated with 4 instead of the user's
    /// Strength score.
    Crossbow,
    /// For a thrown weapon, can be used to cut ropes and fabric.
    Cutting,
    /// Reduces the difficult and Initiative cost of Disarm gambits by 1.
    Disarming,
    /// Weapon damage pools are calculated with 4 instead of the user's
    /// Strength score. Accuracy is not penalized at close range.
    Flame,
    /// Ignores Full Defense defense bonus.
    Flexible,
    /// Can be used to make Grappling gambits.
    Grappling,
    /// Costs 1 initiative to use per attack.
    Improvised,
    /// Can be used mounted.
    Mounted,
    /// Enables a Piercing attack.
    Piercing,
    /// Can convey poison to the target.
    Poisonable,
    /// For a ranged weapon (like a crossbow) does +4 damage at Close range.
    Powerful,
    /// Ignores the benefits of the opponent being mounted.
    Reaching,
    /// Lets you flurry Full Defense with a move action, but -2 damage rating.
    Shield,
    /// Requires a miscellaneous action to reload (which can be flurried).
    Slow,
    /// Enables a Smashing attack.
    Smashing,
    /// Has some unique property not otherwise communicated, like a boomerang's
    /// ability to return after being thrown.
    Special,
    /// Does no damage (but can still convey poison).
    Subtle,
}

/// The full list of Weapon tags as displayed in the book.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum WeaponTag {
    /// The weapon deals Bashing damage. 
    Bashing,
    /// The weapon deals Lethal damage.
    Lethal,
    /// The weapon uses the Archery range curve and ability, up to its 
    /// max range.
    Archery(RangeBand),
    /// The weapon is usable in melee and for parrying with the Brawl ability.
    Brawl,
    /// The weapon is usable in melee and for parrying with the Melee ability.
    Melee,
    /// The weapon uses the Thrown range curve and ability, up to its 
    /// max range.
    Thrown(RangeBand),
    /// The weapon is usable with Martial Arts **only**.
    MartialArts,
    /// Increases the Overwhelming rating by 1.
    Balanced,
    /// Enables a Chopping attack.
    Chopping,
    /// Can be concealed.
    Concealable,
    /// Weapon damage pools are calculated with 4 instead of the user's
    /// Strength score.
    Crossbow,
    /// For a thrown weapon, can be used to cut ropes and fabric.
    Cutting,
    /// Reduces the difficult and Initiative cost of Disarm gambits by 1.
    Disarming,
    /// Weapon damage pools are calculated with 4 instead of the user's
    /// Strength score. Accuracy is not penalized at close range.
    Flame,
    /// Ignores Full Defense defense bonus.
    Flexible,
    /// Can be used to make Grappling gambits.
    Grappling,
    /// Costs 1 initiative to use per attack.
    Improvised,
    /// Can be used mounted.
    Mounted,
    /// The weapon is part of the wielders body and cannot be unequipped or 
    /// disarmed, and is always readied for an attack.
    Natural,
    /// The weapon requires only one hand to wield. Note: this is the unstated
    /// default for melee and thrown weapons. Archery weapons are two-handed
    /// unless they have this tag.
    OneHanded,
    /// Enables a Piercing attack.
    Piercing,
    /// Can convey poison to the target.
    Poisonable,
    /// For a ranged weapon (like a crossbow) does +4 damage at Close range.
    Powerful,
    /// Ignores the benefits of the opponent being mounted.
    Reaching,
    /// Lets you flurry Full Defense with a move action, but -2 damage rating.
    Shield,
    /// Requires a miscellaneous action to reload (which can be flurried).
    Slow,
    /// Enables a Smashing attack.
    Smashing,
    /// Has some unique property not otherwise communicated, like a boomerang's
    /// ability to return after being thrown.
    Special,
    /// Does no damage (but can still convey poison).
    Subtle,
    /// The weapon requires two hands to wield. Note: this is the unstated 
    /// default for Archery weapons. Brawl, Melee, and Martial Arts weapons
    /// are one-handed unless they have this tag.
    TwoHanded,
    /// The weapon can be equipped or unequipped, but while equipped, behaves
    /// as a Natural weapon (can't be disarmed, requires no hands to use).
    Worn,
}

impl From<OptionalWeaponTag> for WeaponTag {
    fn from(tag: OptionalWeaponTag) -> Self {
        match tag {
            OptionalWeaponTag::Balanced => Self::Balanced,
            OptionalWeaponTag::Chopping => Self::Chopping,
            OptionalWeaponTag::Concealable => Self::Concealable,
            OptionalWeaponTag::Crossbow => Self::Crossbow,
            OptionalWeaponTag::Cutting => Self::Cutting,
            OptionalWeaponTag::Disarming => Self::Disarming,
            OptionalWeaponTag::Flame => Self::Flame,
            OptionalWeaponTag::Flexible => Self::Flexible,
            OptionalWeaponTag::Grappling => Self::Grappling,
            OptionalWeaponTag::Improvised => Self::Improvised,
            OptionalWeaponTag::Mounted => Self::Mounted,
            OptionalWeaponTag::Piercing => Self::Piercing,
            OptionalWeaponTag::Poisonable => Self::Poisonable,
            OptionalWeaponTag::Powerful => Self::Powerful,
            OptionalWeaponTag::Reaching => Self::Reaching,
            OptionalWeaponTag::Shield => Self::Shield,
            OptionalWeaponTag::Slow => Self::Slow,
            OptionalWeaponTag::Smashing => Self::Smashing,
            OptionalWeaponTag::Special => Self::Special,
            OptionalWeaponTag::Subtle => Self::Subtle,
        }
    }
}
