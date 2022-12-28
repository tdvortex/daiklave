use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    essence::{Essence, EssenceView, MoteCommitmentView, MoteState, Motes, MotesView},
    Character, CharacterMutationError, CharacterView, CommittedMotesId,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum ExaltState {
    Mortal,
    Exalted(ExaltType),
}

impl Default for ExaltState {
    fn default() -> Self {
        Self::Mortal
    }
}

impl ExaltState {
    pub fn is_mortal(&self) -> bool {
        matches!(self, Self::Mortal)
    }

    pub fn is_exalted(&self) -> bool {
        !self.is_mortal()
    }

    pub fn is_solar(&self) -> bool {
        if let Self::Exalted(exalt_type) = self {
            exalt_type.is_solar()
        } else {
            false
        }
    }

    pub fn check_set_mortal(&self) -> Result<(), CharacterMutationError> {
        Ok(())
    }

    pub fn set_mortal(&mut self) -> Result<&mut Self, CharacterMutationError> {
        *self = ExaltState::Mortal;
        Ok(self)
    }

    pub fn check_set_solar(
        &self,
        _solar_traits: &SolarTraits,
    ) -> Result<(), CharacterMutationError> {
        Ok(())
    }

    pub fn set_solar(
        &mut self,
        solar_traits: &SolarTraits,
    ) -> Result<&mut Self, CharacterMutationError> {
        *self = Self::Exalted(ExaltType::Solar(solar_traits.clone()));
        Ok(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ExaltStateView<'source> {
    Mortal,
    Exalted(ExaltTypeView<'source>),
}

impl<'source> Default for ExaltStateView<'source> {
    fn default() -> Self {
        Self::Mortal
    }
}

impl<'source> ExaltStateView<'source> {
    pub fn is_mortal(&self) -> bool {
        matches!(self, Self::Mortal)
    }

    pub fn is_exalted(&self) -> bool {
        !self.is_mortal()
    }

    pub fn is_solar(&self) -> bool {
        if let Self::Exalted(exalt_type) = self {
            exalt_type.is_solar()
        } else {
            false
        }
    }

    pub fn check_set_mortal(&self) -> Result<(), CharacterMutationError> {
        Ok(())
    }

    pub fn set_mortal(&mut self) -> Result<&mut Self, CharacterMutationError> {
        *self = ExaltStateView::Mortal;
        Ok(self)
    }

    pub fn check_set_solar(
        &self,
        _solar_traits: &'source SolarTraits,
    ) -> Result<(), CharacterMutationError> {
        Ok(())
    }

    pub fn set_solar(
        &mut self,
        solar_traits: &'source SolarTraits,
    ) -> Result<&mut Self, CharacterMutationError> {
        let rating = solar_traits.essence.rating();

        let peripheral = {
            let available = solar_traits.essence.motes().peripheral().available();
            let spent = solar_traits.essence.motes().peripheral().spent();
            MoteState { available, spent }
        };

        let personal = {
            let available = solar_traits.essence.motes().personal().available();
            let spent = solar_traits.essence.motes().personal().spent();
            MoteState { available, spent }
        };

        let commitments = solar_traits
            .essence
            .motes()
            .committed()
            .map(|(id, name, peripheral, personal)| {
                (
                    id,
                    MoteCommitmentView {
                        name,
                        peripheral,
                        personal,
                    },
                )
            })
            .collect::<HashMap<CommittedMotesId, MoteCommitmentView>>();

        let motes = MotesView {
            peripheral,
            personal,
            commitments,
        };

        let essence = EssenceView { rating, motes };

        let solar_traits_view = SolarTraitsView { essence };

        *self = Self::Exalted(ExaltTypeView::Solar(solar_traits_view));
        Ok(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum ExaltType {
    Solar(SolarTraits),
}

impl ExaltType {
    pub fn is_solar(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ExaltTypeView<'source> {
    Solar(SolarTraitsView<'source>),
}

impl<'source> ExaltTypeView<'source> {
    pub fn is_solar(&self) -> bool {
        true
    }
}

/// Traits which are unique to being a Solar Exalted.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SolarTraits {
    pub(crate) essence: Essence,
}

/// Traits which are unique to being a Solar Exalted, with &str
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SolarTraitsView<'source> {
    pub(crate) essence: EssenceView<'source>,
}

pub struct SolarTraitsBuilder;

impl SolarTraits {
    /// Creates a builder to construct SolarTraits.
    pub fn builder() -> SolarTraitsBuilder {
        SolarTraitsBuilder
    }
}

impl SolarTraitsBuilder {
    /// Consumes the builder to finalize Solar Traits.
    pub fn build(self) -> SolarTraits {
        SolarTraits {
            essence: Essence {
                rating: 1,
                motes: Motes {
                    peripheral: MoteState {
                        available: 33,
                        spent: 0,
                    },
                    personal: MoteState {
                        available: 13,
                        spent: 0,
                    },
                    commitments: HashMap::new(),
                },
            },
        }
    }
}

impl Character {
    /// Returns true if character is not Exalted.
    pub fn is_mortal(&self) -> bool {
        self.exalt_state.is_mortal()
    }

    /// Returns true if character is an Exalt.
    pub fn is_exalted(&self) -> bool {
        self.exalt_state.is_exalted()
    }

    /// Returns true if character is a Solar.
    pub fn is_solar(&self) -> bool {
        self.exalt_state.is_solar()
    }

    /// Checks if character can be de-Exalted and set to be mortal.
    pub fn check_set_mortal(&self) -> Result<(), CharacterMutationError> {
        self.exalt_state.check_set_mortal()
    }

    /// Checks if character can be turned into a Solar Exalted with given
    /// traits.
    pub fn check_set_solar(
        &self,
        solar_traits: &SolarTraits,
    ) -> Result<(), CharacterMutationError> {
        self.exalt_state.check_set_solar(solar_traits)
    }

    /// De-Exalts character, setting them to be mortal.
    pub fn set_mortal(&mut self) -> Result<&mut Self, CharacterMutationError> {
        self.exalt_state.set_mortal()?;
        Ok(self)
    }

    /// Sets a character's Exaltation to be the given Solar exaltation.
    pub fn set_solar(
        &mut self,
        solar_traits: &SolarTraits,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exalt_state.set_solar(solar_traits)?;
        Ok(self)
    }
}

impl<'source> CharacterView<'source> {
    /// Returns true if character is not Exalted.
    pub fn is_mortal(&self) -> bool {
        self.exalt_state.is_mortal()
    }

    /// Returns true if character is an Exalt.
    pub fn is_exalted(&self) -> bool {
        self.exalt_state.is_exalted()
    }

    /// Returns true if character is a Solar.
    pub fn is_solar(&self) -> bool {
        self.exalt_state.is_solar()
    }

    /// Checks if character can be de-Exalted and set to be mortal.
    pub fn check_set_mortal(&self) -> Result<(), CharacterMutationError> {
        self.exalt_state.check_set_mortal()
    }

    /// Checks if character can be turned into a Solar Exalted with given
    /// traits.
    pub fn check_set_solar(
        &self,
        solar_traits: &SolarTraits,
    ) -> Result<(), CharacterMutationError> {
        self.exalt_state.check_set_solar(solar_traits)
    }

    /// De-Exalts character, setting them to be mortal.
    pub fn set_mortal(&mut self) -> Result<&mut Self, CharacterMutationError> {
        self.exalt_state.set_mortal()?;
        Ok(self)
    }

    /// Sets a character's Exaltation to be the given Solar exaltation.
    pub fn set_solar(
        &mut self,
        solar_traits: &'source SolarTraits,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exalt_state.set_solar(solar_traits)?;
        Ok(self)
    }
}
