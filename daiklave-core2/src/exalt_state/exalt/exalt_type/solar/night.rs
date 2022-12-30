use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::abilities::AbilityName;

use super::builder::SolarBuilderError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash, PartialOrd, Ord)]
pub(crate) enum NightAbility {
    Athletics,
    Awareness,
    Dodge,
    Investigation,
    Larceny,
    Ride,
    Stealth,
    Socialize,
}

impl From<NightAbility> for AbilityName {
    fn from(value: NightAbility) -> Self {
        match value {
            NightAbility::Athletics => Self::Athletics,
            NightAbility::Awareness => Self::Awareness,
            NightAbility::Dodge => Self::Dodge,
            NightAbility::Investigation => Self::Investigation,
            NightAbility::Larceny => Self::Larceny,
            NightAbility::Ride => Self::Ride,
            NightAbility::Stealth => Self::Stealth,
            NightAbility::Socialize => Self::Socialize,
        }
    }
}

/// Caste traits for the Night Caste Solar.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Night {
    pub(crate) caste_not_supernal: [NightAbility; 4],
    pub(crate) supernal: NightAbility,
}

impl Night {
    /// Builder method
    pub fn builder() -> NightBuilder {
        NightBuilder {
            caste_not_supernal: HashSet::new(),
            supernal: None,
        }
    }

    pub(crate) fn has_caste_ability(&self, ability: AbilityName) -> bool {
        if self
            .caste_not_supernal
            .iter()
            .any(|night_ability| AbilityName::from(*night_ability) == ability)
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
pub struct NightView {
    pub(crate) caste_not_supernal: [NightAbility; 4],
    pub(crate) supernal: NightAbility,
}

impl NightView {
    pub fn has_caste_ability(&self, ability: AbilityName) -> bool {
        if self
            .caste_not_supernal
            .iter()
            .any(|night_ability| AbilityName::from(*night_ability) == ability)
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

/// Builder struct for constructing Night Caste traits.
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
    pub fn build(mut self) -> Result<Night, SolarBuilderError> {
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

        Ok(Night {
            caste_not_supernal: arr,
            supernal,
        })
    }
}
