use serde::{Deserialize, Serialize};

use crate::exaltation::exalt::exalt_type::solar::caste::dawn::{
    caste_ability::DawnCasteAbilityNoBrawl, DawnCasteAbility,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum DawnSupernalLayout {
    MartialArts([DawnCasteAbilityNoBrawl; 4]),
    Brawl([DawnCasteAbilityNoBrawl; 4]),
    Other([DawnCasteAbility; 4], DawnCasteAbilityNoBrawl),
}
