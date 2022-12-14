use crate::abilities::AbilityNameNoSubskill;
use eyre::{eyre, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum TwilightAbility {
    Bureaucracy,
    Craft,
    Integrity,
    Investigation,
    Linguistics,
    Lore,
    Medicine,
    Occult,
}

impl From<TwilightAbility> for AbilityNameNoSubskill {
    fn from(twilight_ability: TwilightAbility) -> Self {
        match twilight_ability {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TwilightTraits(TwilightAbility, Vec<TwilightAbility>);

impl TwilightTraits {
    pub fn builder() -> TwilightTraitsBuilder {
        TwilightTraitsBuilder::default()
    }

    pub fn supernal_ability(&self) -> AbilityNameNoSubskill {
        self.0.into()
    }

    pub fn caste_abilities(&self) -> Vec<AbilityNameNoSubskill> {
        self.1
            .iter()
            .map(|twilight_ability| (*twilight_ability).into())
            .chain(std::iter::once(self.0.into()))
            .collect()
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
pub struct TwilightTraitsBuilder {
    supernal_ability: Option<TwilightAbility>,
    caste_abilities: Vec<TwilightAbility>,
}

impl TwilightTraitsBuilder {
    pub fn with_supernal_ability(mut self, ability: TwilightAbility) -> Self {
        self.supernal_ability = Some(ability);
        self
    }

    pub fn with_caste_ability(mut self, ability: TwilightAbility) -> Self {
        self.caste_abilities.push(ability);
        self
    }

    pub fn build(mut self) -> Result<TwilightTraits> {
        if let Some(ability) = self.supernal_ability {
            self.caste_abilities.sort();
            self.caste_abilities.dedup();
            self.caste_abilities.retain(|a| *a != ability);
            if self.caste_abilities.len() != 4 {
                Err(eyre!("Must have exactly 5 caste abilities"))
            } else {
                Ok(TwilightTraits(ability, self.caste_abilities))
            }
        } else {
            Err(eyre!("Must specify a supernal ability"))
        }
    }
}
