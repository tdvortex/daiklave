use std::num::NonZeroU8;

use serde::{Serialize, Deserialize};

use crate::{abilities::AbilityName, attributes::AttributeName};

/// A prerequisite to purchase a merit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MeritPrerequisite {
    /// Merit is purchasable if the given ability is at or above this level.
    /// For Craft or Martial Arts, this is satisfied if any Craft or Martial
    /// Arts ability is at or above this level.
    Ability(AbilityName, NonZeroU8),
    /// Merit is purchasable if the given attribute is at or above this level.
    Attribute(AttributeName, NonZeroU8),
}