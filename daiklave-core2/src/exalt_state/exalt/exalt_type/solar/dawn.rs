use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::abilities::AbilityName;

use super::builder_error::SolarBuilderError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash, PartialOrd, Ord)]
pub(crate) enum DawnCasteAbility {
    Archery,
    Awareness,
    Brawl,
    Dodge,
    Melee,
    Resistance,
    Thrown,
    War,
}

impl From<DawnCasteAbility> for AbilityName {
    fn from(value: DawnCasteAbility) -> Self {
        match value {
            DawnCasteAbility::Archery => Self::Archery,
            DawnCasteAbility::Awareness => Self::Awareness,
            DawnCasteAbility::Brawl => Self::Brawl,
            DawnCasteAbility::Dodge => Self::Dodge,
            DawnCasteAbility::Melee => Self::Melee,
            DawnCasteAbility::Resistance => Self::Resistance,
            DawnCasteAbility::Thrown => Self::Thrown,
            DawnCasteAbility::War => Self::War,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum DawnSupernalAbility {
    Archery,
    Awareness,
    Brawl,
    Dodge,
    MartialArts,
    Melee,
    Resistance,
    Thrown,
    War,
}

impl From<DawnSupernalAbility> for AbilityName {
    fn from(value: DawnSupernalAbility) -> Self {
        match value {
            DawnSupernalAbility::Archery => Self::Archery,
            DawnSupernalAbility::Awareness => Self::Awareness,
            DawnSupernalAbility::Brawl => Self::Brawl,
            DawnSupernalAbility::Dodge => Self::Dodge,
            DawnSupernalAbility::MartialArts => Self::MartialArts,
            DawnSupernalAbility::Melee => Self::Melee,
            DawnSupernalAbility::Resistance => Self::Resistance,
            DawnSupernalAbility::Thrown => Self::Thrown,
            DawnSupernalAbility::War => Self::War,
        }
    }
}

/// Caste traits for the Dawn Caste Solar. Note that because of
/// Brawl/MartialArts, Dawns have 5 possible Caste abilities but 6 possible
/// Supernal abilities.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Dawn {
    pub(crate) caste_not_supernal: [DawnCasteAbility; 4],
    pub(crate) supernal: DawnSupernalAbility,
}

impl Dawn {
    /// Builder method
    pub fn builder() -> DawnBuilder {
        DawnBuilder {
            caste_not_supernal: HashSet::new(),
            supernal: None,
        }
    }

    pub(crate) fn has_caste_ability(&self, ability: AbilityName) -> bool {
        if self
            .caste_not_supernal
            .iter()
            .any(|dawn_caste_ability| AbilityName::from(*dawn_caste_ability) == ability)
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
pub struct DawnView {
    pub(crate) caste_not_supernal: [DawnCasteAbility; 4],
    pub(crate) supernal: DawnSupernalAbility,
}

impl DawnView {
    pub fn has_caste_ability(&self, ability: AbilityName) -> bool {
        if self
            .caste_not_supernal
            .iter()
            .any(|dawn_caste_ability| AbilityName::from(*dawn_caste_ability) == ability)
        {
            true
        } else {
            AbilityName::from(self.supernal) == ability
        }
    }

    pub fn supernal_ability(&self) -> AbilityName {
        AbilityName::from(self.supernal)
    }

    pub fn into_owned(self) -> Dawn {
        Dawn {
            caste_not_supernal: self.caste_not_supernal,
            supernal: self.supernal,
        }
    }
}

/// Builder struct for constructing Dawn Caste traits.
pub struct DawnBuilder {
    caste_not_supernal: HashSet<DawnCasteAbility>,
    supernal: Option<DawnSupernalAbility>,
}

impl DawnBuilder {
    /// Adds a caste ability to the Dawn. MartialArts is **not** valid here.
    pub fn add_caste_ability(
        &mut self,
        ability: AbilityName,
    ) -> Result<&mut Self, SolarBuilderError> {
        if ability == AbilityName::MartialArts {
            return Err(SolarBuilderError::MartialArts);
        }

        let duplicate: bool = !match ability {
            AbilityName::Archery => self.caste_not_supernal.insert(DawnCasteAbility::Archery),
            AbilityName::Awareness => self.caste_not_supernal.insert(DawnCasteAbility::Awareness),
            AbilityName::Brawl => self.caste_not_supernal.insert(DawnCasteAbility::Brawl),
            AbilityName::Dodge => self.caste_not_supernal.insert(DawnCasteAbility::Dodge),
            AbilityName::Melee => self.caste_not_supernal.insert(DawnCasteAbility::Melee),
            AbilityName::Resistance => self.caste_not_supernal.insert(DawnCasteAbility::Resistance),
            AbilityName::Thrown => self.caste_not_supernal.insert(DawnCasteAbility::Thrown),
            AbilityName::War => self.caste_not_supernal.insert(DawnCasteAbility::War),
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

    /// Sets the Supernal ability to the specified value. MartialArts **is**
    /// valid here, provided that Brawl was a Caste ability.
    pub fn set_supernal_ability(
        &mut self,
        ability: AbilityName,
    ) -> Result<&mut Self, SolarBuilderError> {
        match ability {
            AbilityName::Archery => {
                self.supernal = Some(DawnSupernalAbility::Archery);
            }
            AbilityName::Awareness => {
                self.supernal = Some(DawnSupernalAbility::Awareness);
            }
            AbilityName::Brawl => {
                self.supernal = Some(DawnSupernalAbility::Brawl);
            }
            AbilityName::Dodge => {
                self.supernal = Some(DawnSupernalAbility::Dodge);
            }
            AbilityName::MartialArts => {
                self.supernal = Some(DawnSupernalAbility::MartialArts);
            }
            AbilityName::Melee => {
                self.supernal = Some(DawnSupernalAbility::Melee);
            }
            AbilityName::Resistance => {
                self.supernal = Some(DawnSupernalAbility::Resistance);
            }
            AbilityName::Thrown => {
                self.supernal = Some(DawnSupernalAbility::Thrown);
            }
            AbilityName::War => {
                self.supernal = Some(DawnSupernalAbility::War);
            }
            _ => return Err(SolarBuilderError::InvalidCasteAbility),
        };

        Ok(self)
    }

    /// Completes the build process and returns a DawnView struct if successful.
    pub fn build(mut self) -> Result<DawnView, SolarBuilderError> {
        if self.supernal.is_none() {
            return Err(SolarBuilderError::MissingField("supernal"));
        }

        let supernal = self.supernal.unwrap();

        match supernal {
            DawnSupernalAbility::Archery => {
                self.caste_not_supernal.remove(&DawnCasteAbility::Archery);
            }
            DawnSupernalAbility::Awareness => {
                self.caste_not_supernal.remove(&DawnCasteAbility::Awareness);
            }
            DawnSupernalAbility::Brawl => {
                self.caste_not_supernal.remove(&DawnCasteAbility::Brawl);
            }
            DawnSupernalAbility::Dodge => {
                self.caste_not_supernal.remove(&DawnCasteAbility::Dodge);
            }
            DawnSupernalAbility::MartialArts => {
                self.caste_not_supernal.remove(&DawnCasteAbility::Brawl);
            }
            DawnSupernalAbility::Melee => {
                self.caste_not_supernal.remove(&DawnCasteAbility::Melee);
            }
            DawnSupernalAbility::Resistance => {
                self.caste_not_supernal
                    .remove(&DawnCasteAbility::Resistance);
            }
            DawnSupernalAbility::Thrown => {
                self.caste_not_supernal.remove(&DawnCasteAbility::Thrown);
            }
            DawnSupernalAbility::War => {
                self.caste_not_supernal.remove(&DawnCasteAbility::War);
            }
        };

        if self.caste_not_supernal.len() != 4 {
            return Err(SolarBuilderError::CasteAndFavoredCount);
        }

        let mut option_arr: [Option<DawnCasteAbility>; 4] = [None; 4];

        for (i, dawn_ability) in self.caste_not_supernal.into_iter().enumerate() {
            option_arr[i] = Some(dawn_ability);
        }

        let mut arr = option_arr.map(|opt| opt.unwrap());
        arr.sort();

        Ok(DawnView {
            caste_not_supernal: arr,
            supernal,
        })
    }
}
