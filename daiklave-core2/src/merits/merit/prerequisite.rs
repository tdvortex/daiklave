use serde::{Deserialize, Serialize};

use crate::{abilities::AbilityName, attributes::AttributeName};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub(crate) enum MeritPrerequisite {
    Ability(AbilityName, u8),
    Attribute(AttributeName, u8),
}
