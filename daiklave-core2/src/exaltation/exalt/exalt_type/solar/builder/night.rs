use crate::{
    abilities::AbilityName,
    exaltation::exalt::{
        exalt_type::solar::{
            caste::{
                night::{NightAbility, NightMemo},
                SolarCasteMemo,
            },
            NewSolar, SolarError, SolarMemo,
        },
        LimitMemo,
    },
};

pub struct NightBuilder {
    pub(crate) caste_abilities: Vec<NightAbility>,
    pub(crate) supernal_ability: Option<NightAbility>,
    pub(crate) favored_abilities: Vec<AbilityName>,
    pub(crate) limit_trigger: Option<String>,
}

impl NightBuilder {
    pub fn caste_ability(mut self, caste_ability: NightAbility) -> Self {
        self.caste_abilities.push(caste_ability);
        self
    }

    pub fn supernal_ability(mut self, supernal_ability: NightAbility) -> Self {
        self.caste_abilities.push(supernal_ability);
        self.supernal_ability = Some(supernal_ability);
        self
    }

    pub fn favored_ability(mut self, favored_ability: AbilityName) -> Self {
        self.favored_abilities.push(favored_ability);
        self
    }

    pub fn limit_trigger(mut self, limit_trigger: String) -> Self {
        self.limit_trigger = Some(limit_trigger);
        self
    }

    pub fn build(mut self) -> Result<NewSolar, SolarError> {
        let supernal = self.supernal_ability.ok_or(SolarError::SupernalRequired)?;

        self.caste_abilities.sort();
        self.caste_abilities.dedup();
        if self.caste_abilities.len() != 5 {
            return Err(SolarError::FiveCasteAbilities);
        }

        self.favored_abilities.sort();
        self.favored_abilities.dedup();
        self.favored_abilities
            .retain(|ability| ability != &AbilityName::MartialArts);
        if self.favored_abilities.len() != 5 {
            return Err(SolarError::FiveFavoredAbilities);
        }

        for caste_ability in self.caste_abilities.iter() {
            if self.favored_abilities.contains(&(*caste_ability).into()) {
                return Err(SolarError::CasteAndFavoredUnique);
            }
        }

        self.caste_abilities.retain(|caste| caste != &supernal);

        let caste_not_supernal = self.caste_abilities.into_iter().enumerate().fold(
            [NightAbility::Athletics; 4],
            |mut arr, (i, ability)| {
                arr[i] = ability;
                arr
            },
        );
        let favored_abilities = self.favored_abilities.into_iter().enumerate().fold(
            [AbilityName::Archery; 5],
            |mut arr, (i, ability)| {
                arr[i] = ability;
                arr
            },
        );

        let limit_trigger = self.limit_trigger.ok_or(SolarError::LimitTriggerRequired)?;

        Ok(NewSolar(Box::new(SolarMemo {
            caste: SolarCasteMemo::Night(NightMemo {
                caste_not_supernal,
                supernal,
            }),
            favored_abilities,
            sorcery: None,
            limit: LimitMemo {
                track: 0,
                trigger: limit_trigger,
            },
        })))
    }
}
