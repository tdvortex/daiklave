use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::AbilityName;

use super::SolarBuilderError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord, Hash)]
pub(crate) enum TwilightAbility {
    Bureaucracy,
    Craft,
    Integrity,
    Investigation,
    Linguistics,
    Lore,
    Medicine,
    Occult,
}

impl From<TwilightAbility> for AbilityName {
    fn from(value: TwilightAbility) -> Self {
        match value {
            TwilightAbility::Bureaucracy => Self::Bureaucracy,
            TwilightAbility::Craft => Self::Craft,
            TwilightAbility::Integrity => Self::Integrity,
            TwilightAbility::Investigation => Self::Investigation,
            TwilightAbility::Linguistics => Self::Linguistics,
            TwilightAbility::Lore => Self::Lore,
            TwilightAbility::Medicine => Self::Medicine,
            TwilightAbility::Occult => Self::Occult,
        }
    }
}

/// Caste traits for the Twilight Caste Solar.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Twilight {
    pub(crate) caste_not_supernal: [TwilightAbility; 4],
    pub(crate) supernal: TwilightAbility,
}

impl Twilight {
    /// Builder method
    pub fn builder() -> TwilightBuilder {
        TwilightBuilder {
            caste_not_supernal: HashSet::new(),
            supernal: None,
        }
    }

    pub(crate) fn has_caste_ability(&self, ability: AbilityName) -> bool {
        if self
            .caste_not_supernal
            .iter()
            .any(|twilight_ability| AbilityName::from(*twilight_ability) == ability)
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
pub struct TwilightView {
    pub(crate) caste_not_supernal: [TwilightAbility; 4],
    pub(crate) supernal: TwilightAbility,
}

impl TwilightView {
    pub fn has_caste_ability(&self, ability: AbilityName) -> bool {
        if self
            .caste_not_supernal
            .iter()
            .any(|twilight_ability| AbilityName::from(*twilight_ability) == ability)
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

/// Builder struct for constructing Twilight Caste traits.
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
    pub fn build(mut self) -> Result<Twilight, SolarBuilderError> {
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

        let mut arr = option_arr.map(|opt| opt.unwrap());
        arr.sort();

        Ok(Twilight {
            caste_not_supernal: arr,
            supernal,
        })
    }
}
