use crate::CharacterMutation;

use super::{SolarMemo, builder::SolarBuilder};

/// Solar traits to be added to a character, overriding any previous Exaltation
/// (even if it was Solar).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetSolar(pub(crate) Box<SolarMemo>);

impl SetSolar {
    pub fn builder() -> SolarBuilder {
        SolarBuilder {
            limit_trigger: None,
        }
    }
}

impl From<SetSolar> for CharacterMutation {
    fn from(set_solar: SetSolar) -> Self {
        CharacterMutation::SetSolar(set_solar)
    }
}