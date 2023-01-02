use std::collections::HashSet;

use crate::{
    abilities::AbilityName, exaltation::exalt::exalt_type::solar::builder_error::SolarBuilderError,
};

use super::{Zenith, ZenithAbility};

/// Builder struct for constructing Zenith Caste traits.
#[derive(Debug, Default)]
pub struct ZenithBuilder {
    caste_not_supernal: HashSet<ZenithAbility>,
    supernal: Option<ZenithAbility>,
}

impl ZenithBuilder {
    /// Adds a caste ability.
    pub fn add_caste_ability(
        &mut self,
        ability: AbilityName,
    ) -> Result<&mut Self, SolarBuilderError> {
        let duplicate: bool = !match ability {
            AbilityName::Athletics => self.caste_not_supernal.insert(ZenithAbility::Athletics),
            AbilityName::Integrity => self.caste_not_supernal.insert(ZenithAbility::Integrity),
            AbilityName::Performance => self.caste_not_supernal.insert(ZenithAbility::Performance),
            AbilityName::Lore => self.caste_not_supernal.insert(ZenithAbility::Lore),
            AbilityName::Presence => self.caste_not_supernal.insert(ZenithAbility::Presence),
            AbilityName::Resistance => self.caste_not_supernal.insert(ZenithAbility::Resistance),
            AbilityName::Survival => self.caste_not_supernal.insert(ZenithAbility::Survival),
            AbilityName::War => self.caste_not_supernal.insert(ZenithAbility::War),
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
                self.supernal = Some(ZenithAbility::Athletics);
            }
            AbilityName::Integrity => {
                self.supernal = Some(ZenithAbility::Integrity);
            }
            AbilityName::Performance => {
                self.supernal = Some(ZenithAbility::Performance);
            }
            AbilityName::Lore => {
                self.supernal = Some(ZenithAbility::Lore);
            }
            AbilityName::Presence => {
                self.supernal = Some(ZenithAbility::Presence);
            }
            AbilityName::Resistance => {
                self.supernal = Some(ZenithAbility::Resistance);
            }
            AbilityName::Survival => {
                self.supernal = Some(ZenithAbility::Survival);
            }
            AbilityName::War => {
                self.supernal = Some(ZenithAbility::War);
            }
            _ => return Err(SolarBuilderError::InvalidCasteAbility),
        };

        Ok(self)
    }

    /// Completes the build process and returns a ZenithView struct if successful.
    pub fn build(mut self) -> Result<Zenith, SolarBuilderError> {
        if self.supernal.is_none() {
            return Err(SolarBuilderError::MissingField("supernal"));
        }

        let supernal = self.supernal.unwrap();
        self.caste_not_supernal.remove(&supernal);

        if self.caste_not_supernal.len() != 4 {
            return Err(SolarBuilderError::CasteAndFavoredCount);
        }

        let mut option_arr: [Option<ZenithAbility>; 4] = [None; 4];

        for (i, dawn_ability) in self.caste_not_supernal.into_iter().enumerate() {
            option_arr[i] = Some(dawn_ability);
        }

        let mut caste_not_supernal = option_arr.map(|opt| opt.unwrap());
        caste_not_supernal.sort();

        Ok(Zenith::new(caste_not_supernal, supernal))
    }
}
