use serde::{Deserialize, Serialize};

use super::range::RangeBand;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub(in crate::weapons) enum OtherWeaponTag {
    Balanced,
    Chopping,
    Concealable,
    Crossbow,
    Cutting,
    Disarming,
    Flame,
    Flexible,
    Grappling,
    Improvised,
    Mounted,
    Piercing,
    Poisonable,
    Powerful,
    Reaching,
    Shield,
    Slow,
    Smashing,
    Special,
    Subtle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum WeaponTag {
    Bashing,
    Lethal,
    Archery(RangeBand),
    Brawl,
    Melee,
    Thrown(RangeBand),
    MartialArts,
    Balanced,
    Chopping,
    Concealable,
    Crossbow,
    Cutting,
    Disarming,
    Flame,
    Flexible,
    Grappling,
    Improvised,
    Mounted,
    Natural,
    OneHanded,
    Piercing,
    Poisonable,
    Powerful,
    Reaching,
    Shield,
    Slow,
    Smashing,
    Special,
    Subtle,
    TwoHanded,
    Worn,
}

impl From<OtherWeaponTag> for WeaponTag {
    fn from(tag: OtherWeaponTag) -> Self {
        match tag {
            OtherWeaponTag::Balanced => Self::Balanced,
            OtherWeaponTag::Chopping => Self::Chopping,
            OtherWeaponTag::Concealable => Self::Concealable,
            OtherWeaponTag::Crossbow => Self::Crossbow,
            OtherWeaponTag::Cutting => Self::Cutting,
            OtherWeaponTag::Disarming => Self::Disarming,
            OtherWeaponTag::Flame => Self::Flame,
            OtherWeaponTag::Flexible => Self::Flexible,
            OtherWeaponTag::Grappling => Self::Grappling,
            OtherWeaponTag::Improvised => Self::Improvised,
            OtherWeaponTag::Mounted => Self::Mounted,
            OtherWeaponTag::Piercing => Self::Piercing,
            OtherWeaponTag::Poisonable => Self::Poisonable,
            OtherWeaponTag::Powerful => Self::Powerful,
            OtherWeaponTag::Reaching => Self::Reaching,
            OtherWeaponTag::Shield => Self::Shield,
            OtherWeaponTag::Slow => Self::Slow,
            OtherWeaponTag::Smashing => Self::Smashing,
            OtherWeaponTag::Special => Self::Special,
            OtherWeaponTag::Subtle => Self::Subtle,
        }
    }
}
