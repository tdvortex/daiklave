use serde::{Deserialize, Serialize};
use super::{twilight_ability::TwilightAbility};

/// An owned copy of Twilight Solar traits
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TwilightMemo {
    caste_not_supernal: [TwilightAbility; 4],
    supernal: TwilightAbility,
}