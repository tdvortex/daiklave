use crate::{
    abilities::AbilityName,
    exaltation::exalt::{
        exalt_type::solar::{
            caste::{
                night::{Night, NightAbility},
                SolarCasteMemo,
            },
            SetSolar, SolarError, SolarMemo,
        },
        limit::LimitTrigger,
        LimitMemo,
    },
    experience::ExperiencePool,
};

/// A builder for a Night caste Solar.
#[derive(Default)]
pub struct NightBuilder {
    pub(crate) caste_abilities: Vec<NightAbility>,
    pub(crate) supernal_ability: Option<NightAbility>,
    pub(crate) favored_abilities: Vec<AbilityName>,
    pub(crate) limit_trigger: Option<LimitTrigger>,
}

impl NightBuilder {
    /// Creates a new Night builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a Caste ability to the Night.
    pub fn caste_ability(mut self, caste_ability: NightAbility) -> Self {
        self.caste_abilities.push(caste_ability);
        self
    }

    /// Sets the Night's supernal ability.
    pub fn supernal_ability(mut self, supernal_ability: NightAbility) -> Self {
        self.caste_abilities.push(supernal_ability);
        self.supernal_ability = Some(supernal_ability);
        self
    }

    /// Adds a Favored ability to the Solar.
    pub fn favored_ability(mut self, favored_ability: AbilityName) -> Self {
        self.favored_abilities.push(favored_ability);
        self
    }

    /// Sets the Solar's Limit Trigger.
    pub fn limit_trigger(mut self, limit_trigger: impl Into<LimitTrigger>) -> Self {
        self.limit_trigger = Some(limit_trigger.into());
        self
    }

    /// Finishes the builder, returning a NewSolar object if successful or an
    /// error if some validation failed.
    pub fn build(mut self) -> Result<SetSolar, SolarError> {
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

        Ok(SetSolar(Box::new(SolarMemo {
            caste: SolarCasteMemo::Night(Night {
                caste_not_supernal,
                supernal,
            }),
            favored_abilities,
            sorcery: None,
            limit: LimitMemo {
                track: 0,
                trigger: limit_trigger,
            },
            solar_charms: Vec::new(),
            experience: ExperiencePool::default(),
        })))
    }
}
