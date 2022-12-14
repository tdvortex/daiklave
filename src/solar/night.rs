use crate::abilities::AbilityNameNoSubskill;
use eyre::{eyre, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum NightAbility {
    Athletics,
    Awareness,
    Dodge,
    Investigation,
    Larceny,
    Ride,
    Stealth,
    Socialize,
}

impl From<NightAbility> for AbilityNameNoSubskill {
    fn from(night_ability: NightAbility) -> Self {
        match night_ability {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NightTraits(NightAbility, [NightAbility; 4]);

impl NightTraits {
    pub fn builder() -> NightTraitsBuilder {
        NightTraitsBuilder::default()
    }

    pub fn supernal_ability(&self) -> AbilityNameNoSubskill {
        self.0.into()
    }

    pub fn caste_abilities(&self) -> [AbilityNameNoSubskill; 5] {
        self.1
            .iter()
            .map(|night_ability| (*night_ability).into())
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
            || self
                .caste_abilities()
                .into_iter()
                .find(|a| *a == ability)
                .is_some()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct NightTraitsBuilder {
    supernal_ability: Option<NightAbility>,
    caste_abilities: Vec<NightAbility>,
}

impl NightTraitsBuilder {
    pub fn with_supernal_ability(mut self, ability: NightAbility) -> Self {
        self.supernal_ability = Some(ability);
        self
    }

    pub fn with_caste_ability(mut self, ability: NightAbility) -> Self {
        self.caste_abilities.push(ability);
        self
    }

    pub fn build(mut self) -> Result<NightTraits> {
        if let Some(ability) = self.supernal_ability {
            self.caste_abilities.sort();
            self.caste_abilities.dedup();
            self.caste_abilities.retain(|a| *a != ability);
            if self.caste_abilities.len() != 4 {
                Err(eyre!("Must have exactly 5 caste abilities"))
            } else {
                Ok(NightTraits(
                    ability,
                    self.caste_abilities.into_iter().enumerate().fold(
                        [NightAbility::Athletics; 4],
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
