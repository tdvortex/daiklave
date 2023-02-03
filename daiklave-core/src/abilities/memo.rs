use serde::{Deserialize, Serialize};

use super::ability::AbilityRatingMemo;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct AbilitiesMemo {
    pub(in crate::abilities) archery: AbilityRatingMemo,
    pub(in crate::abilities) athletics: AbilityRatingMemo,
    pub(in crate::abilities) awareness: AbilityRatingMemo,
    pub(in crate::abilities) brawl: AbilityRatingMemo,
    pub(in crate::abilities) bureaucracy: AbilityRatingMemo,
    pub(in crate::abilities) dodge: AbilityRatingMemo,
    pub(in crate::abilities) integrity: AbilityRatingMemo,
    pub(in crate::abilities) investigation: AbilityRatingMemo,
    pub(in crate::abilities) larceny: AbilityRatingMemo,
    pub(in crate::abilities) linguistics: AbilityRatingMemo,
    pub(in crate::abilities) lore: AbilityRatingMemo,
    pub(in crate::abilities) medicine: AbilityRatingMemo,
    pub(in crate::abilities) melee: AbilityRatingMemo,
    pub(in crate::abilities) occult: AbilityRatingMemo,
    pub(in crate::abilities) performance: AbilityRatingMemo,
    pub(in crate::abilities) presence: AbilityRatingMemo,
    pub(in crate::abilities) resistance: AbilityRatingMemo,
    pub(in crate::abilities) ride: AbilityRatingMemo,
    pub(in crate::abilities) sail: AbilityRatingMemo,
    pub(in crate::abilities) socialize: AbilityRatingMemo,
    pub(in crate::abilities) stealth: AbilityRatingMemo,
    pub(in crate::abilities) survival: AbilityRatingMemo,
    pub(in crate::abilities) thrown: AbilityRatingMemo,
    pub(in crate::abilities) war: AbilityRatingMemo,
}
