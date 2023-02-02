use crate::{
    abilities::AbilityName,
    exaltation::exalt::{
        exalt_type::solar::{
            caste::{
                dawn::{
                    Dawn, DawnCasteAbility, DawnCasteAbilityNoBrawl, DawnSupernalAbility,
                    DawnSupernalLayout,
                },
                SolarCasteMemo,
            },
            SetSolar, SolarError, SolarMemo,
        },
        limit::LimitTrigger,
        LimitMemo,
    },
    experience::ExperiencePool,
};

/// A builder for a Dawn caste Solar.
pub struct DawnBuilder {
    pub(crate) caste_abilities: Vec<DawnCasteAbility>,
    pub(crate) supernal_ability: Option<DawnSupernalAbility>,
    pub(crate) favored_abilities: Vec<AbilityName>,
    pub(crate) limit_trigger: Option<LimitTrigger>,
}

impl DawnBuilder {
    /// Adds a Caste ability to the Dawn. Martial Arts cannot be a Caste
    /// ability.
    pub fn caste_ability(mut self, caste_ability: DawnCasteAbility) -> Self {
        self.caste_abilities.push(caste_ability);
        self
    }

    /// Sets the Supernal ability of the Dawn. If it wasn't already a Caste
    /// ability, it is also added as a Caste ability.
    pub fn supernal_ability(mut self, supernal_ability: DawnSupernalAbility) -> Self {
        let caste_insert = match supernal_ability {
            DawnSupernalAbility::Archery => DawnCasteAbility::Archery,
            DawnSupernalAbility::Awareness => DawnCasteAbility::Awareness,
            DawnSupernalAbility::Brawl => DawnCasteAbility::Brawl,
            DawnSupernalAbility::Dodge => DawnCasteAbility::Dodge,
            DawnSupernalAbility::MartialArts => DawnCasteAbility::Brawl,
            DawnSupernalAbility::Melee => DawnCasteAbility::Melee,
            DawnSupernalAbility::Resistance => DawnCasteAbility::Resistance,
            DawnSupernalAbility::Thrown => DawnCasteAbility::Thrown,
            DawnSupernalAbility::War => DawnCasteAbility::War,
        };
        self.caste_abilities.push(caste_insert);
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
        let limit_trigger = self.limit_trigger.ok_or(SolarError::LimitTriggerRequired)?;
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

        let favored_abilities = self.favored_abilities.into_iter().enumerate().fold(
            [AbilityName::Archery; 5],
            |mut arr, (i, ability)| {
                arr[i] = ability;
                arr
            },
        );

        let layout = match supernal {
            DawnSupernalAbility::Brawl => {
                let (no_brawl, inserted) = self
                    .caste_abilities
                    .into_iter()
                    .filter_map(|caste_ability| match caste_ability {
                        DawnCasteAbility::Archery => Some(DawnCasteAbilityNoBrawl::Archery),
                        DawnCasteAbility::Awareness => Some(DawnCasteAbilityNoBrawl::Awareness),
                        DawnCasteAbility::Brawl => None,
                        DawnCasteAbility::Dodge => Some(DawnCasteAbilityNoBrawl::Dodge),
                        DawnCasteAbility::Melee => Some(DawnCasteAbilityNoBrawl::Melee),
                        DawnCasteAbility::Resistance => Some(DawnCasteAbilityNoBrawl::Resistance),
                        DawnCasteAbility::Thrown => Some(DawnCasteAbilityNoBrawl::Thrown),
                        DawnCasteAbility::War => Some(DawnCasteAbilityNoBrawl::War),
                    })
                    .take(4)
                    .fold(
                        ([DawnCasteAbilityNoBrawl::Archery; 4], 0),
                        |(mut acc, i), ability| {
                            acc[i] = ability;
                            (acc, i + 1)
                        },
                    );
                if inserted < 4 {
                    return Err(SolarError::FiveCasteAbilities);
                } else {
                    DawnSupernalLayout::Brawl(no_brawl)
                }
            }
            DawnSupernalAbility::MartialArts => {
                let (no_brawl, inserted) = self
                    .caste_abilities
                    .into_iter()
                    .filter_map(|caste_ability| match caste_ability {
                        DawnCasteAbility::Archery => Some(DawnCasteAbilityNoBrawl::Archery),
                        DawnCasteAbility::Awareness => Some(DawnCasteAbilityNoBrawl::Awareness),
                        DawnCasteAbility::Brawl => None,
                        DawnCasteAbility::Dodge => Some(DawnCasteAbilityNoBrawl::Dodge),
                        DawnCasteAbility::Melee => Some(DawnCasteAbilityNoBrawl::Melee),
                        DawnCasteAbility::Resistance => Some(DawnCasteAbilityNoBrawl::Resistance),
                        DawnCasteAbility::Thrown => Some(DawnCasteAbilityNoBrawl::Thrown),
                        DawnCasteAbility::War => Some(DawnCasteAbilityNoBrawl::War),
                    })
                    .take(4)
                    .fold(
                        ([DawnCasteAbilityNoBrawl::Archery; 4], 0),
                        |(mut acc, i), ability| {
                            acc[i] = ability;
                            (acc, i + 1)
                        },
                    );
                if inserted < 4 {
                    return Err(SolarError::FiveCasteAbilities);
                } else {
                    DawnSupernalLayout::MartialArts(no_brawl)
                }
            }
            DawnSupernalAbility::Archery => {
                let (caste, inserted) = self
                    .caste_abilities
                    .into_iter()
                    .filter(|caste_ability| caste_ability == &DawnCasteAbility::Archery)
                    .take(4)
                    .fold(
                        ([DawnCasteAbility::Archery; 4], 0),
                        |(mut acc, i), ability| {
                            acc[i] = ability;
                            (acc, i + 1)
                        },
                    );
                if inserted < 4 {
                    return Err(SolarError::FiveCasteAbilities);
                } else {
                    DawnSupernalLayout::Other(caste, DawnCasteAbilityNoBrawl::Archery)
                }
            }
            DawnSupernalAbility::Awareness => {
                let (caste, inserted) = self
                    .caste_abilities
                    .into_iter()
                    .filter(|caste_ability| caste_ability == &DawnCasteAbility::Awareness)
                    .take(4)
                    .fold(
                        ([DawnCasteAbility::Archery; 4], 0),
                        |(mut acc, i), ability| {
                            acc[i] = ability;
                            (acc, i + 1)
                        },
                    );
                if inserted < 4 {
                    return Err(SolarError::FiveCasteAbilities);
                } else {
                    DawnSupernalLayout::Other(caste, DawnCasteAbilityNoBrawl::Awareness)
                }
            }
            DawnSupernalAbility::Dodge => {
                let (caste, inserted) = self
                    .caste_abilities
                    .into_iter()
                    .filter(|caste_ability| caste_ability == &DawnCasteAbility::Dodge)
                    .take(4)
                    .fold(
                        ([DawnCasteAbility::Archery; 4], 0),
                        |(mut acc, i), ability| {
                            acc[i] = ability;
                            (acc, i + 1)
                        },
                    );
                if inserted < 4 {
                    return Err(SolarError::FiveCasteAbilities);
                } else {
                    DawnSupernalLayout::Other(caste, DawnCasteAbilityNoBrawl::Dodge)
                }
            }
            DawnSupernalAbility::Melee => {
                let (caste, inserted) = self
                    .caste_abilities
                    .into_iter()
                    .filter(|caste_ability| caste_ability == &DawnCasteAbility::Melee)
                    .take(4)
                    .fold(
                        ([DawnCasteAbility::Archery; 4], 0),
                        |(mut acc, i), ability| {
                            acc[i] = ability;
                            (acc, i + 1)
                        },
                    );
                if inserted < 4 {
                    return Err(SolarError::FiveCasteAbilities);
                } else {
                    DawnSupernalLayout::Other(caste, DawnCasteAbilityNoBrawl::Melee)
                }
            }
            DawnSupernalAbility::Resistance => {
                let (caste, inserted) = self
                    .caste_abilities
                    .into_iter()
                    .filter(|caste_ability| caste_ability == &DawnCasteAbility::Resistance)
                    .take(4)
                    .fold(
                        ([DawnCasteAbility::Archery; 4], 0),
                        |(mut acc, i), ability| {
                            acc[i] = ability;
                            (acc, i + 1)
                        },
                    );
                if inserted < 4 {
                    return Err(SolarError::FiveCasteAbilities);
                } else {
                    DawnSupernalLayout::Other(caste, DawnCasteAbilityNoBrawl::Resistance)
                }
            }
            DawnSupernalAbility::Thrown => {
                let (caste, inserted) = self
                    .caste_abilities
                    .into_iter()
                    .filter(|caste_ability| caste_ability == &DawnCasteAbility::Thrown)
                    .take(4)
                    .fold(
                        ([DawnCasteAbility::Archery; 4], 0),
                        |(mut acc, i), ability| {
                            acc[i] = ability;
                            (acc, i + 1)
                        },
                    );
                if inserted < 4 {
                    return Err(SolarError::FiveCasteAbilities);
                } else {
                    DawnSupernalLayout::Other(caste, DawnCasteAbilityNoBrawl::Thrown)
                }
            }
            DawnSupernalAbility::War => {
                let (caste, inserted) = self
                    .caste_abilities
                    .into_iter()
                    .filter(|caste_ability| caste_ability == &DawnCasteAbility::War)
                    .take(4)
                    .fold(
                        ([DawnCasteAbility::Archery; 4], 0),
                        |(mut acc, i), ability| {
                            acc[i] = ability;
                            (acc, i + 1)
                        },
                    );
                if inserted < 4 {
                    return Err(SolarError::FiveCasteAbilities);
                } else {
                    DawnSupernalLayout::Other(caste, DawnCasteAbilityNoBrawl::War)
                }
            }
        };

        Ok(SetSolar(Box::new(SolarMemo {
            caste: SolarCasteMemo::Dawn(Dawn { layout }),
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
