use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::abilities::AbilityName;

use super::builder::SolarBuilderError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord, Hash)]
pub(crate) enum ZenithAbility {
    Athletics,
    Integrity,
    Performance,
    Lore,
    Presence,
    Resistance,
    Survival,
    War,
}

impl From<ZenithAbility> for AbilityName {
    fn from(value: ZenithAbility) -> Self {
        match value {
            ZenithAbility::Athletics => Self::Athletics,
            ZenithAbility::Integrity => Self::Integrity,
            ZenithAbility::Performance => Self::Performance,
            ZenithAbility::Lore => Self::Lore,
            ZenithAbility::Presence => Self::Presence,
            ZenithAbility::Resistance => Self::Resistance,
            ZenithAbility::Survival => Self::Survival,
            ZenithAbility::War => Self::War,
        }
    }
}

/// Caste traits for the Zenith Caste Solar.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Zenith {
    pub(crate) caste_not_supernal: [ZenithAbility; 4],
    pub(crate) supernal: ZenithAbility,
}

impl Zenith {
    /// Builder method
    pub fn builder() -> ZenithBuilder {
        ZenithBuilder {
            caste_not_supernal: HashSet::new(),
            supernal: None,
        }
    }

    pub(crate) fn has_caste_ability(&self, ability: AbilityName) -> bool {
        if self
            .caste_not_supernal
            .iter()
            .any(|zenith_caste_ability| AbilityName::from(*zenith_caste_ability) == ability)
        {
            true
        } else {
            AbilityName::from(self.supernal) == ability
        }
    }

    pub(crate) fn supernal_ability(&self) -> AbilityName {
        AbilityName::from(self.supernal)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZenithView {
    pub(crate) caste_not_supernal: [ZenithAbility; 4],
    pub(crate) supernal: ZenithAbility,
}

impl ZenithView {
    pub fn has_caste_ability(&self, ability: AbilityName) -> bool {
        if self
            .caste_not_supernal
            .iter()
            .any(|zenith_caste_ability| AbilityName::from(*zenith_caste_ability) == ability)
        {
            true
        } else {
            AbilityName::from(self.supernal) == ability
        }
    }

    pub fn supernal_ability(&self) -> AbilityName {
        AbilityName::from(self.supernal)
    }
}

/// Builder struct for constructing Zenith Caste traits.
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

    /// Completes the build process and returns a Zenith struct if successful.
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

        let mut arr = option_arr.map(|opt| opt.unwrap());
        arr.sort();

        Ok(Zenith {
            caste_not_supernal: arr,
            supernal,
        })
    }
}
