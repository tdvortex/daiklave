use crate::CharacterMutation;

use super::{builder::{SolarBuilder, DawnBuilder, ZenithBuilder, TwilightBuilder, NightBuilder, EclipseBuilder}, SolarMemo};

/// A character mutation to set the character to be a Solar with the given
/// traits, overriding any previous Exaltation in the process.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetSolar(pub(crate) Box<SolarMemo>);

impl SetSolar {
    /// Starts the builder process as a Dawn Caste Solar.
    pub fn dawn() -> DawnBuilder {
        DawnBuilder::new()
    }

    /// Starts the builder process as a Zenith Caste Solar.
    pub fn zenith() -> ZenithBuilder {
        ZenithBuilder::new()
    }

    /// Starts the builder process as a Twilight Caste Solar.
    pub fn twilight() -> TwilightBuilder {
        TwilightBuilder::new()
    }

    /// Starts the builder process as a Night Caste Solar.
    pub fn night() -> NightBuilder {
        NightBuilder::new()
    }

    /// Starts the builder process as an Eclipse Caste Solar.
    pub fn eclipse() -> EclipseBuilder {
        EclipseBuilder::new()
    }

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
