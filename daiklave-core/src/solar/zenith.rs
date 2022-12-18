use crate::abilities::AbilityNameNoSubskill;
use eyre::{eyre, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum ZenithAbility {
    Athletics,
    Integrity,
    Performance,
    Lore,
    Presence,
    Resistance,
    Survival,
    War,
}

impl From<ZenithAbility> for AbilityNameNoSubskill {
    fn from(zenith_ability: ZenithAbility) -> Self {
        match zenith_ability {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ZenithTraits(ZenithAbility, [ZenithAbility; 4]);

impl ZenithTraits {
    pub fn builder() -> ZenithTraitsBuilder {
        ZenithTraitsBuilder::default()
    }

    pub fn supernal_ability(&self) -> AbilityNameNoSubskill {
        self.0.into()
    }

    pub fn caste_abilities(&self) -> [AbilityNameNoSubskill; 5] {
        self.1
            .iter()
            .map(|zenith_ability| (*zenith_ability).into())
            .chain(std::iter::once(self.0.into()))
            .enumerate()
            .fold(
                [AbilityNameNoSubskill::Archery; 5],
                |mut arr, (index, ability)| {
                    arr[index] = ability;
                    arr
                },
            )
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
pub struct ZenithTraitsBuilder {
    supernal_ability: Option<ZenithAbility>,
    caste_abilities: Vec<ZenithAbility>,
}

impl ZenithTraitsBuilder {
    pub fn with_supernal_ability(mut self, ability: ZenithAbility) -> Self {
        self.supernal_ability = Some(ability);
        self
    }

    pub fn with_caste_ability(mut self, ability: ZenithAbility) -> Self {
        self.caste_abilities.push(ability);
        self
    }

    pub fn build(mut self) -> Result<ZenithTraits> {
        if let Some(ability) = self.supernal_ability {
            self.caste_abilities.sort();
            self.caste_abilities.dedup();
            self.caste_abilities.retain(|a| *a != ability);
            if self.caste_abilities.len() != 4 {
                Err(eyre!("Must have exactly 5 caste abilities"))
            } else {
                Ok(ZenithTraits(
                    ability,
                    self.caste_abilities.into_iter().enumerate().fold(
                        [ZenithAbility::Athletics; 4],
                        |mut arr, (index, ability)| {
                            arr[index] = ability;
                            arr
                        },
                    ),
                ))
            }
        } else {
            Err(eyre!("Must specify a supernal ability"))
        }
    }
}
