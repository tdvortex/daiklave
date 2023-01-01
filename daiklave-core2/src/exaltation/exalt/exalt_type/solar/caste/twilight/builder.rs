use std::collections::HashSet;

use crate::{
    abilities::AbilityName, exaltation::exalt::exalt_type::solar::builder_error::SolarBuilderError,
};

use super::{twilight_ability::TwilightAbility, twilight_view::TwilightView};

/// Builder struct for constructing Twilight Caste traits.
#[derive(Debug, Default)]
pub struct TwilightBuilder {
    caste_not_supernal: HashSet<TwilightAbility>,
    supernal: Option<TwilightAbility>,
}

impl TwilightBuilder {
    /// Adds a caste ability.
    pub fn add_caste_ability(
        &mut self,
        ability: AbilityName,
    ) -> Result<&mut Self, SolarBuilderError> {
        let duplicate: bool = !match ability {
            AbilityName::Bureaucracy => {
                self.caste_not_supernal.insert(TwilightAbility::Bureaucracy)
            }
            AbilityName::Craft => self.caste_not_supernal.insert(TwilightAbility::Craft),
            AbilityName::Integrity => self.caste_not_supernal.insert(TwilightAbility::Integrity),
            AbilityName::Investigation => self
                .caste_not_supernal
                .insert(TwilightAbility::Investigation),
            AbilityName::Linguistics => {
                self.caste_not_supernal.insert(TwilightAbility::Linguistics)
            }
            AbilityName::Lore => self.caste_not_supernal.insert(TwilightAbility::Lore),
            AbilityName::Medicine => self.caste_not_supernal.insert(TwilightAbility::Medicine),
            AbilityName::Occult => self.caste_not_supernal.insert(TwilightAbility::Occult),
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
            AbilityName::Bureaucracy => {
                self.supernal = Some(TwilightAbility::Bureaucracy);
            }
            AbilityName::Craft => {
                self.supernal = Some(TwilightAbility::Craft);
            }
            AbilityName::Integrity => {
                self.supernal = Some(TwilightAbility::Integrity);
            }
            AbilityName::Investigation => {
                self.supernal = Some(TwilightAbility::Investigation);
            }
            AbilityName::Linguistics => {
                self.supernal = Some(TwilightAbility::Linguistics);
            }
            AbilityName::Lore => {
                self.supernal = Some(TwilightAbility::Lore);
            }
            AbilityName::Medicine => {
                self.supernal = Some(TwilightAbility::Medicine);
            }
            AbilityName::Occult => {
                self.supernal = Some(TwilightAbility::Occult);
            }
            _ => return Err(SolarBuilderError::InvalidCasteAbility),
        };

        Ok(self)
    }

    /// Completes the build process and returns a Twilight struct if successful.
    pub fn build(mut self) -> Result<TwilightView, SolarBuilderError> {
        if self.supernal.is_none() {
            return Err(SolarBuilderError::MissingField("supernal"));
        }

        let supernal = self.supernal.unwrap();
        self.caste_not_supernal.remove(&supernal);

        if self.caste_not_supernal.len() != 4 {
            return Err(SolarBuilderError::CasteAndFavoredCount);
        }

        let mut option_arr: [Option<TwilightAbility>; 4] = [None; 4];

        for (i, dawn_ability) in self.caste_not_supernal.into_iter().enumerate() {
            option_arr[i] = Some(dawn_ability);
        }

        let mut caste_not_supernal = option_arr.map(|opt| opt.unwrap());
        caste_not_supernal.sort();

        Ok(TwilightView::new(caste_not_supernal, supernal))
    }
}
