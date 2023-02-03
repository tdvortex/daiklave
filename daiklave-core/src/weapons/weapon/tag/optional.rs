use serde::{Deserialize, Serialize};

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
    /// For a ranged weapon (like a crossbow) does +3 damage at Close range.
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
