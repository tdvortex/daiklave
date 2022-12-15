use crate::abilities::AbilityNameNoSubskill;
use eyre::{eyre, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum EclipseAbility {
    Bureaucracy,
    Larceny,
    Linguistics,
    Occult,
    Presence,
    Ride,
    Sail,
    Socialize,
}

impl From<EclipseAbility> for AbilityNameNoSubskill {
    fn from(eclipse_ability: EclipseAbility) -> Self {
        match eclipse_ability {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EclipseTraits(EclipseAbility, [EclipseAbility; 4]);

impl EclipseTraits {
    pub fn builder() -> EclipseTraitsBuilder {
        EclipseTraitsBuilder::default()
    }

    pub fn supernal_ability(&self) -> AbilityNameNoSubskill {
        self.0.into()
    }

    pub fn caste_abilities(&self) -> [AbilityNameNoSubskill; 5] {
        self.1
            .iter()
            .map(|eclipse_ability| (*eclipse_ability).into())
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
pub struct EclipseTraitsBuilder {
    supernal_ability: Option<EclipseAbility>,
    caste_abilities: Vec<EclipseAbility>,
}

impl EclipseTraitsBuilder {
    pub fn with_supernal_ability(mut self, ability: EclipseAbility) -> Self {
        self.supernal_ability = Some(ability);
        self
    }

    pub fn with_caste_ability(mut self, ability: EclipseAbility) -> Self {
        self.caste_abilities.push(ability);
        self
    }

    pub fn build(mut self) -> Result<EclipseTraits> {
        if let Some(ability) = self.supernal_ability {
            self.caste_abilities.sort();
            self.caste_abilities.dedup();
            self.caste_abilities.retain(|a| *a != ability);
            if self.caste_abilities.len() != 4 {
                Err(eyre!("Must have exactly 5 caste abilities"))
            } else {
                Ok(EclipseTraits(
                    ability,
                    self.caste_abilities.into_iter().enumerate().fold(
                        [EclipseAbility::Bureaucracy; 4],
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
