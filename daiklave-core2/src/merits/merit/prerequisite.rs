use crate::{abilities::AbilityName, attributes::AttributeName};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum MeritPrerequisite {
    Ability(AbilityName, u8),
    Attribute(AttributeName, u8),
}