use serde::{Deserialize, Serialize};

use crate::abilities::AbilityNameNoSubskill;
use eyre::{eyre, Result};

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum DawnAbility {
    Archery,
    Awareness,
    Brawl,
    Dodge,
    Melee,
    Resistance,
    Thrown,
    War,
}

impl From<DawnAbility> for AbilityNameNoSubskill {
    fn from(dawn_ability: DawnAbility) -> Self {
        match dawn_ability {
            DawnAbility::Archery => Self::Archery,
            DawnAbility::Awareness => Self::Awareness,
            DawnAbility::Brawl => Self::Brawl,
            DawnAbility::Dodge => Self::Dodge,
            DawnAbility::Melee => Self::Melee,
            DawnAbility::Resistance => Self::Resistance,
            DawnAbility::Thrown => Self::Thrown,
            DawnAbility::War => Self::War,
        }
    }
}

/// Dawn Solars can't choose MartialArts as a caste ability, but can choose it
/// as their Supernal if and only if Brawl is one of their five caste abilities
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DawnTraits {
    MartialArtsSupernal([DawnAbility; 4]),
    NotMartialArtsSupernal(DawnAbility, [DawnAbility; 4]),
}

impl DawnTraits {
    pub fn builder() -> DawnTraitsBuilder {
        DawnTraitsBuilder::default()
    }

    pub fn supernal_ability(&self) -> AbilityNameNoSubskill {
        match self {
            DawnTraits::MartialArtsSupernal(_) => AbilityNameNoSubskill::MartialArts,
            DawnTraits::NotMartialArtsSupernal(dawn_ability, _) => (*dawn_ability).into(),
        }
    }

    pub fn caste_abilities(&self) -> [AbilityNameNoSubskill; 5] {
        let mut output: [AbilityNameNoSubskill; 5] = match self {
            DawnTraits::MartialArtsSupernal(list) => list
                .iter()
                .map(|dawn_ability| (*dawn_ability).into())
                .chain(std::iter::once(AbilityNameNoSubskill::Brawl))
                .enumerate()
                .fold(
                    [AbilityNameNoSubskill::Archery; 5],
                    |mut arr, (index, ability)| {
                        arr[index] = ability;
                        arr
                    },
                ),
            DawnTraits::NotMartialArtsSupernal(supernal, list) => list
                .iter()
                .map(|dawn_ability| (*dawn_ability).into())
                .chain(std::iter::once((*supernal).into()))
                .enumerate()
                .fold(
                    [AbilityNameNoSubskill::Archery; 5],
                    |mut arr, (index, ability)| {
                        arr[index] = ability;
                        arr
                    },
                ),
        };

        output.sort();
        output
    }

    pub fn has_supernal_ability(&self, ability: AbilityNameNoSubskill) -> bool {
        self.supernal_ability() == ability
    }

    pub fn has_caste_ability(&self, ability: AbilityNameNoSubskill) -> bool {
        self.has_supernal_ability(ability)
            || self.caste_abilities().into_iter().any(|a| a == ability)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct DawnTraitsBuilder {
    is_martial_arts_supernal: bool,
    supernal_ability: Option<DawnAbility>,
    caste_abilities: Vec<DawnAbility>,
}

impl DawnTraitsBuilder {
    pub fn with_non_supernal_caste_ability(mut self, ability: DawnAbility) -> Self {
        self.caste_abilities.push(ability);
        self
    }

    pub fn with_supernal_ability(mut self, ability: DawnAbility) -> Self {
        if self.is_martial_arts_supernal {
            self.is_martial_arts_supernal = false;
        }
        self.is_martial_arts_supernal = false;
        self.caste_abilities.retain(|a| *a != ability);
        self.supernal_ability = Some(ability);
        self
    }

    pub fn with_martial_arts_supernal(mut self) -> Self {
        self.is_martial_arts_supernal = true;
        self.supernal_ability = None;
        self.caste_abilities.retain(|a| *a != DawnAbility::Brawl);
        self
    }

    pub fn build(mut self) -> Result<DawnTraits> {
        if self.is_martial_arts_supernal && self.supernal_ability.is_some() {
            return Err(eyre!("Cannot have multiple supernal abilities"));
        }

        if !self.is_martial_arts_supernal && self.supernal_ability.is_none() {
            return Err(eyre!("Must specify a supernal ability"));
        }

        self.caste_abilities.sort();
        self.caste_abilities.dedup();

        if self.is_martial_arts_supernal {
            self.caste_abilities.retain(|a| *a != DawnAbility::Brawl);
        } else {
            self.caste_abilities
                .retain(|a| Some(a) != self.supernal_ability.as_ref());
        }

        if self.caste_abilities.len() != 4 {
            if self.is_martial_arts_supernal {
                return Err(eyre!("Martial Arts Supernal requires five Caste abilities, one of which must be Brawl"));
            } else {
                return Err(eyre!("Must have exactly 5 caste abilities"));
            }
        }

        if self.is_martial_arts_supernal {
            Ok(DawnTraits::MartialArtsSupernal(
                self.caste_abilities.into_iter().enumerate().fold(
                    [DawnAbility::Archery; 4],
                    |mut arr, (index, ability)| {
                        arr[index] = ability;
                        arr
                    },
                ),
            ))
        } else {
            Ok(DawnTraits::NotMartialArtsSupernal(
                self.supernal_ability.unwrap(),
                self.caste_abilities.into_iter().enumerate().fold(
                    [DawnAbility::Archery; 4],
                    |mut arr, (index, ability)| {
                        arr[index] = ability;
                        arr
                    },
                ),
            ))
        }
    }
}
