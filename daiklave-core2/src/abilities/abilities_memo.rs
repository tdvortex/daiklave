use serde::{Deserialize, Serialize};

use super::{ability_memo::AbilityMemo};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct AbilitiesMemo {
    archery: AbilityMemo,
    athletics: AbilityMemo,
    awareness: AbilityMemo,
    brawl: AbilityMemo,
    bureaucracy: AbilityMemo,
    dodge: AbilityMemo,
    integrity: AbilityMemo,
    investigation: AbilityMemo,
    larceny: AbilityMemo,
    linguistics: AbilityMemo,
    lore: AbilityMemo,
    medicine: AbilityMemo,
    melee: AbilityMemo,
    occult: AbilityMemo,
    performance: AbilityMemo,
    presence: AbilityMemo,
    resistance: AbilityMemo,
    ride: AbilityMemo,
    sail: AbilityMemo,
    socialize: AbilityMemo,
    stealth: AbilityMemo,
    survival: AbilityMemo,
    thrown: AbilityMemo,
    war: AbilityMemo,
}