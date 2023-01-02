use std::collections::HashSet;

use crate::{
    abilities::AbilityName, exaltation::exalt::exalt_type::solar::builder_error::SolarBuilderError,
};

use super::{Eclipse, EclipseAbility};

/// Builder struct for constructing Eclipse Caste traits.
#[derive(Debug, Default)]
pub struct EclipseBuilder {
    caste_not_supernal: HashSet<EclipseAbility>,
    supernal: Option<EclipseAbility>,
}

impl EclipseBuilder {
    /// Adds a caste ability.
    pub fn add_caste_ability(
        &mut self,
        ability: AbilityName,
    ) -> Result<&mut Self, SolarBuilderError> {
        let duplicate: bool = !match ability {
            AbilityName::Bureaucracy => self.caste_not_supernal.insert(EclipseAbility::Bureaucracy),
            AbilityName::Larceny => self.caste_not_supernal.insert(EclipseAbility::Larceny),
            AbilityName::Linguistics => self.caste_not_supernal.insert(EclipseAbility::Linguistics),
            AbilityName::Occult => self.caste_not_supernal.insert(EclipseAbility::Occult),
            AbilityName::Presence => self.caste_not_supernal.insert(EclipseAbility::Presence),
            AbilityName::Ride => self.caste_not_supernal.insert(EclipseAbility::Ride),
            AbilityName::Sail => self.caste_not_supernal.insert(EclipseAbility::Sail),
            AbilityName::Socialize => self.caste_not_supernal.insert(EclipseAbility::Socialize),
            _ => return Err(SolarBuilderError::InvalidCasteAbility),
        };

        if duplicate {
            return Err(SolarBuilderError::UniqueCasteAndFavored);
        }

        if self.caste_not_supernal.len() > 5 {
            return Err(SolarBuilderError::CasteAndFavoredCount);
        }

        Ok(self)
    }

    /// Sets the Supernal ability.
    pub fn set_supernal_ability(
        &mut self,
        ability: AbilityName,
    ) -> Result<&mut Self, SolarBuilderError> {
        match ability {
            AbilityName::Bureaucracy => {
                self.supernal = Some(EclipseAbility::Bureaucracy);
            }
            AbilityName::Larceny => {
                self.supernal = Some(EclipseAbility::Larceny);
            }
            AbilityName::Linguistics => {
                self.supernal = Some(EclipseAbility::Linguistics);
            }
            AbilityName::Occult => {
                self.supernal = Some(EclipseAbility::Occult);
            }
            AbilityName::Presence => {
                self.supernal = Some(EclipseAbility::Presence);
            }
            AbilityName::Ride => {
                self.supernal = Some(EclipseAbility::Ride);
            }
            AbilityName::Sail => {
                self.supernal = Some(EclipseAbility::Sail);
            }
            AbilityName::Socialize => {
                self.supernal = Some(EclipseAbility::Socialize);
            }
            _ => return Err(SolarBuilderError::InvalidCasteAbility),
        };

        Ok(self)
    }

    /// Completes the build process and returns an EclipseView struct if successful.
    pub fn build(mut self) -> Result<Eclipse, SolarBuilderError> {
        if self.supernal.is_none() {
            return Err(SolarBuilderError::MissingField("supernal"));
        }

        let supernal = self.supernal.unwrap();
        self.caste_not_supernal.remove(&supernal);

        if self.caste_not_supernal.len() != 4 {
            return Err(SolarBuilderError::CasteAndFavoredCount);
        }

        let mut option_arr: [Option<EclipseAbility>; 4] = [None; 4];

        for (i, dawn_ability) in self.caste_not_supernal.into_iter().enumerate() {
            option_arr[i] = Some(dawn_ability);
        }

        let mut caste_not_supernal = option_arr.map(|opt| opt.unwrap());
        caste_not_supernal.sort();

        Ok(Eclipse::new(caste_not_supernal, supernal))
    }
}
