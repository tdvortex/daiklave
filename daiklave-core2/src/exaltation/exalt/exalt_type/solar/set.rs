use crate::CharacterMutation;

use super::{builder::SolarBuilder, SolarMemo};

/// A character mutation to set the character to be a Solar with the given
/// traits, overriding any previous Exaltation in the process.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetSolar(pub(crate) Box<SolarMemo>);

impl SetSolar {
    /// Starts constructing the traits for a Solar Exalt.
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
