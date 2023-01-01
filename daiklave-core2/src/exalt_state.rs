use serde::{Deserialize, Serialize};

/// Traits common to all Exalts
pub mod exalt;

/// Traits which are unique to mortals (or which function differently without
///  Essence)
pub mod mortal;
use exalt::{Exalt, ExaltView};
use mortal::{Mortal, MortalView};

use crate::{
    sorcery::{SolarSorcerer, SolarSorcererView},
    Character, CharacterMutationError, CharacterView,
};

use self::exalt::exalt_type::{ExaltType, ExaltTypeView};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum ExaltState {
    Mortal(Mortal),
    Exalt(Exalt),
}

impl Default for ExaltState {
    fn default() -> Self {
        Self::Mortal(Mortal::default())
    }
}

impl ExaltState {
    pub fn is_mortal(&self) -> bool {
        matches!(self, Self::Mortal(_))
    }

    pub fn is_exalted(&self) -> bool {
        !self.is_mortal()
    }

    pub fn check_set_mortal(&self) -> Result<(), CharacterMutationError> {
        Ok(())
    }

    pub fn set_mortal(&mut self) -> Result<&mut Self, CharacterMutationError> {
        if self.is_mortal() {
            return Ok(self);
        }

        let exalt = if let ExaltState::Exalt(exalt) = self {
            exalt
        } else {
            unreachable!()
        };

        // Preserve Terrestrial circle sorcery
        let sorcery = {
            match &exalt.exalt_type {
                ExaltType::Solar(solar) => {
                    if let Some(sorcery) = &solar.sorcery {
                        match sorcery {
                            SolarSorcerer::Terrestrial(terrestrial) => Some(terrestrial.clone()),
                            SolarSorcerer::Celestial(celestial) => Some(celestial.clone().into()),
                            SolarSorcerer::Solar(solar) => Some(solar.clone().into()),
                        }
                    } else {
                        None
                    }
                }
            }
        };

        // Preserve martial arts styles
        let martial_arts_styles = std::mem::take(&mut exalt.martial_arts_styles)
            .into_iter()
            .map(|(id, exalt_artist)| (id, exalt_artist.into()))
            .collect();

        *self = ExaltState::Mortal(Mortal {
            martial_arts_styles,
            sorcery,
        });

        Ok(self)
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

    /// Checks if character can be de-Exalted and set to be mortal.
    pub fn check_set_mortal(&self) -> Result<(), CharacterMutationError> {
        self.exalt_state.check_set_mortal()
    }

    /// De-Exalts character, setting them to be mortal. This also reduces their
    /// permanent willpower rating by 2 (reflecting the difference between
    /// mortal default and Exalt default).
    pub fn set_mortal(&mut self) -> Result<&mut Self, CharacterMutationError> {
        if self.is_mortal() {
            return Ok(self);
        }
        self.exalt_state.set_mortal()?;
        let new_willpower_rating = self.willpower().rating().max(2) - 2;
        self.set_willpower_rating(new_willpower_rating)?;
        Ok(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ExaltStateView<'source> {
    Mortal(MortalView<'source>),
    Exalt(ExaltView<'source>),
}

impl<'source> Default for ExaltStateView<'source> {
    fn default() -> Self {
        Self::Mortal(MortalView::default())
    }
}

impl<'source> ExaltStateView<'source> {
    pub fn is_mortal(&self) -> bool {
        matches!(self, Self::Mortal(_))
    }

    pub fn is_exalted(&self) -> bool {
        !self.is_mortal()
    }

    pub fn check_set_mortal(&self) -> Result<(), CharacterMutationError> {
        Ok(())
    }

    pub fn set_mortal(&mut self) -> Result<&mut Self, CharacterMutationError> {
        if self.is_mortal() {
            return Ok(self);
        }

        let exalt = if let ExaltStateView::Exalt(exalt) = self {
            exalt
        } else {
            unreachable!()
        };

        // Preserve Terrestrial circle sorcery
        let sorcery = {
            match &exalt.exalt_type {
                ExaltTypeView::Solar(solar) => {
                    if let Some(sorcery) = &solar.sorcery {
                        match sorcery {
                            SolarSorcererView::Terrestrial(terrestrial) => {
                                Some(terrestrial.clone())
                            }
                            SolarSorcererView::Celestial(celestial) => Some(celestial.into()),
                            SolarSorcererView::Solar(solar) => Some(solar.into()),
                        }
                    } else {
                        None
                    }
                }
            }
        };

        // Preserve martial arts styles
        let martial_arts_styles = std::mem::take(&mut exalt.martial_arts_styles)
            .into_iter()
            .map(|(id, exalt_artist)| (id, exalt_artist.into()))
            .collect();

        *self = ExaltStateView::Mortal(MortalView {
            martial_arts_styles,
            sorcery,
        });
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

    /// Checks if character can be de-Exalted and set to be mortal.
    pub fn check_set_mortal(&self) -> Result<(), CharacterMutationError> {
        self.exalt_state.check_set_mortal()
    }

    /// De-Exalts character, setting them to be mortal. This also reduces their
    /// permanent willpower rating by 2 (reflecting the difference between
    /// mortal default and Exalt default).
    pub fn set_mortal(&mut self) -> Result<&mut Self, CharacterMutationError> {
        if self.is_mortal() {
            return Ok(self);
        }
        self.exalt_state.set_mortal()?;
        let new_willpower_rating = self.willpower().rating().max(2) - 2;
        self.set_willpower_rating(new_willpower_rating)?;
        Ok(self)
    }
}
