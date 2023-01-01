use std::collections::HashSet;

use crate::{
    abilities::AbilityName, exaltation::exalt::exalt_type::solar::builder_error::SolarBuilderError,
};

use super::{night_ability::NightAbility, night_view::NightView};

/// Builder struct for constructing Night Caste traits.
#[derive(Debug, Default)]
pub struct NightBuilder {
    caste_not_supernal: HashSet<NightAbility>,
    supernal: Option<NightAbility>,
}

impl NightBuilder {
    /// Adds a caste ability.
    pub fn add_caste_ability(
        &mut self,
        ability: AbilityName,
    ) -> Result<&mut Self, SolarBuilderError> {
        let duplicate: bool = !match ability {
            AbilityName::Athletics => self.caste_not_supernal.insert(NightAbility::Athletics),
            AbilityName::Awareness => self.caste_not_supernal.insert(NightAbility::Awareness),
            AbilityName::Dodge => self.caste_not_supernal.insert(NightAbility::Dodge),
            AbilityName::Investigation => {
                self.caste_not_supernal.insert(NightAbility::Investigation)
            }
            AbilityName::Larceny => self.caste_not_supernal.insert(NightAbility::Larceny),
            AbilityName::Ride => self.caste_not_supernal.insert(NightAbility::Ride),
            AbilityName::Stealth => self.caste_not_supernal.insert(NightAbility::Stealth),
            AbilityName::Socialize => self.caste_not_supernal.insert(NightAbility::Socialize),
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

    /// Sets the Supernal ability
    pub fn set_supernal_ability(
        &mut self,
        ability: AbilityName,
    ) -> Result<&mut Self, SolarBuilderError> {
        match ability {
            AbilityName::Athletics => {
                self.supernal = Some(NightAbility::Athletics);
            }
            AbilityName::Awareness => {
                self.supernal = Some(NightAbility::Awareness);
            }
            AbilityName::Dodge => {
                self.supernal = Some(NightAbility::Dodge);
            }
            AbilityName::Investigation => {
                self.supernal = Some(NightAbility::Investigation);
            }
            AbilityName::Larceny => {
                self.supernal = Some(NightAbility::Larceny);
            }
            AbilityName::Ride => {
                self.supernal = Some(NightAbility::Ride);
            }
            AbilityName::Stealth => {
                self.supernal = Some(NightAbility::Stealth);
            }
            AbilityName::Socialize => {
                self.supernal = Some(NightAbility::Socialize);
            }
            _ => return Err(SolarBuilderError::InvalidCasteAbility),
        };

        Ok(self)
    }

    /// Completes the build process and returns a Night struct if successful.
    pub fn build(mut self) -> Result<NightView, SolarBuilderError> {
        if self.supernal.is_none() {
            return Err(SolarBuilderError::MissingField("supernal"));
        }

        let supernal = self.supernal.unwrap();
        self.caste_not_supernal.remove(&supernal);

        if self.caste_not_supernal.len() != 4 {
            return Err(SolarBuilderError::CasteAndFavoredCount);
        }

        let mut option_arr: [Option<NightAbility>; 4] = [None; 4];

        for (i, dawn_ability) in self.caste_not_supernal.into_iter().enumerate() {
            option_arr[i] = Some(dawn_ability);
        }

        let mut arr = option_arr.map(|opt| opt.unwrap());
        arr.sort();

        Ok(NightView {
            caste_not_supernal: arr,
            supernal,
        })
    }
}
