use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::abilities::AbilityName;

use super::builder::SolarBuilderError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash, PartialOrd, Ord)]
pub(crate) enum EclipseAbility {
    Bureaucracy,
    Larceny,
    Linguistics,
    Occult,
    Presence,
    Ride,
    Sail,
    Socialize,
}

impl From<EclipseAbility> for AbilityName {
    fn from(value: EclipseAbility) -> Self {
        match value {
            EclipseAbility::Bureaucracy => Self::Bureaucracy,
            EclipseAbility::Larceny => Self::Larceny,
            EclipseAbility::Linguistics => Self::Linguistics,
            EclipseAbility::Occult => Self::Occult,
            EclipseAbility::Presence => Self::Presence,
            EclipseAbility::Ride => Self::Ride,
            EclipseAbility::Sail => Self::Sail,
            EclipseAbility::Socialize => Self::Socialize,
        }
    }
}

/// Caste traits for the Eclipse Caste Solar.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Eclipse {
    pub(crate) caste_not_supernal: [EclipseAbility; 4],
    pub(crate) supernal: EclipseAbility,
}

impl Eclipse {
    /// Builder method
    pub fn builder() -> EclipseBuilder {
        EclipseBuilder {
            caste_not_supernal: HashSet::new(),
            supernal: None,
        }
    }

    pub(crate) fn has_caste_ability(&self, ability: AbilityName) -> bool {
        if self
            .caste_not_supernal
            .iter()
            .any(|eclipse_ability| AbilityName::from(*eclipse_ability) == ability)
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
pub struct EclipseView {
    pub(crate) caste_not_supernal: [EclipseAbility; 4],
    pub(crate) supernal: EclipseAbility,
}

impl EclipseView {
    pub fn has_caste_ability(&self, ability: AbilityName) -> bool {
        if self
            .caste_not_supernal
            .iter()
            .any(|eclipse_ability| AbilityName::from(*eclipse_ability) == ability)
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

/// Builder struct for constructing Eclipse Caste traits.
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

    /// Completes the build process and returns an Eclipse struct if successful.
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

        let mut arr = option_arr.map(|opt| opt.unwrap());
        arr.sort();

        Ok(Eclipse {
            caste_not_supernal: arr,
            supernal,
        })
    }
}
