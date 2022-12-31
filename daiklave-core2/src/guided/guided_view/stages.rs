use crate::{
    guided::{error::GuidedError, ExaltationChoice, GuidedMutation, GuidedStage},
    CharacterMutation, abilities::AbilityNameVanilla,
};

use super::GuidedView;

impl<'source> GuidedView<'source> {
    pub(in crate::guided) fn validate_correct_stage(
        &self,
        guided_mutation: &GuidedMutation,
    ) -> Result<(), GuidedError> {
        if match guided_mutation {
            GuidedMutation::CharacterMutation(mutation) => match mutation {
                CharacterMutation::SetName(_)
                | CharacterMutation::SetConcept(_)
                | CharacterMutation::RemoveConcept => {
                    self.stage == GuidedStage::ChooseNameAndConcept
                }
                CharacterMutation::SetAttribute(_, _) => {
                    self.stage == GuidedStage::ChooseAttributes
                }
                _ => false,
            },
            GuidedMutation::AdvanceStage => true,
            GuidedMutation::SetExaltation(_) => self.stage == GuidedStage::ChooseExaltation,
            GuidedMutation::AddSolarCasteAbility(_)
            | GuidedMutation::RemoveSolarCasteAbility(_) => {
                self.stage == GuidedStage::ChooseSolarCasteAbilities
            }
            GuidedMutation::SetSolarSupernalAbility(_) => {
                self.stage == GuidedStage::ChooseSolarSupernalAbility
            }
            GuidedMutation::AddSolarFavoredAbility(_)
            | GuidedMutation::RemoveSolarFavoredAbility(_) => {
                self.stage == GuidedStage::ChooseSolarFavoredAbilities
            }
            GuidedMutation::AddMartialArtsStyle(_, _)
            | GuidedMutation::RemoveMartialArtsStyle(_) => {
                self.stage == GuidedStage::ChooseMartialArtsStyles
            }
            GuidedMutation::SetSorceryArchetype(_, _)
            | GuidedMutation::SetShapingRitual(_, _)
            | GuidedMutation::SetControlSpell(_, _) => self.stage == GuidedStage::ChooseSorcery,
        } {
            Ok(())
        } else {
            Err(GuidedError::StageOrderError)
        }
    }

    pub(in crate::guided) fn validate_stage_complete(&self) -> Result<(), GuidedError> {
        if !match self.stage {
            GuidedStage::ChooseNameAndConcept => true,
            GuidedStage::ChooseExaltation => self.exaltation_choice.is_some(),
            GuidedStage::ChooseAttributes => {
                if let Some(exaltation_choice) = self.exaltation_choice {
                    match exaltation_choice {
                        ExaltationChoice::Mortal => {
                            let (primary, secondary, tertiary) = self.attributes_buckets();
                            primary >= 6 && secondary >= 4 && tertiary >= 3
                        }
                        ExaltationChoice::Dawn
                        | ExaltationChoice::Zenith
                        | ExaltationChoice::Twilight
                        | ExaltationChoice::Night
                        | ExaltationChoice::Eclipse => {
                            let (primary, secondary, tertiary) = self.attributes_buckets();
                            primary >= 8 && secondary >= 6 && tertiary >= 4
                        }
                    }
                } else {
                    return Err(GuidedError::StageOrderError);
                }
            }
            GuidedStage::ChooseSolarCasteAbilities => {
                matches!(
                    self.solar_caste_abilities.as_ref().map(|v| v.len()),
                    Some(5)
                )
            }
            GuidedStage::ChooseSolarSupernalAbility => {
                matches!(self.solar_supernal_ability, Some(_))
            }
            GuidedStage::ChooseSolarFavoredAbilities => {
                matches!(
                    self.solar_favored_abilities.as_ref().map(|v| v.len()),
                    Some(5)
                )
            }
            GuidedStage::ChooseMartialArtsStyles => true,
            GuidedStage::ChooseSorcery => {
                // Either no sorcery, or all sorcery is specified
                self.sorcery_archetype.is_some() == self.shaping_ritual.is_some()
                    && self.shaping_ritual.is_some() == self.control_spell.is_some()
            }
            GuidedStage::ChooseAbilities => todo!(),
        } {
            Err(GuidedError::StageIncompleteError)
        } else {
            Ok(())
        }
    }

    pub(in crate::guided) fn next_stage(&self) -> Result<GuidedStage, GuidedError> {
        // Mortal order: ChooseNameAndConcept > ChooseExaltation
        //   > ChooseAttributes > ChooseMartialArtsStyles > ChooseSorcery
        //   > Choose Abilities
        // Solar order: ChooseNameAndConcept > ChooseExaltation
        //   > ChooseAttributes > ChooseSolarCasteAbilities
        //   > ChooseSolarSupernalAbility > ChooseSolarFavoredAbilities
        //   > ChooseMartialArtsStyles > ChooseSorcery > Choose Abilities
        Ok(match (self.stage, self.exaltation_choice) {
            (GuidedStage::ChooseNameAndConcept, _) => GuidedStage::ChooseExaltation,
            (GuidedStage::ChooseExaltation, _) => GuidedStage::ChooseAttributes,
            (GuidedStage::ChooseAttributes, Some(ExaltationChoice::Mortal)) => {
                GuidedStage::ChooseMartialArtsStyles
            }
            (GuidedStage::ChooseAttributes, Some(ExaltationChoice::Dawn))
            | (GuidedStage::ChooseAttributes, Some(ExaltationChoice::Zenith))
            | (GuidedStage::ChooseAttributes, Some(ExaltationChoice::Twilight))
            | (GuidedStage::ChooseAttributes, Some(ExaltationChoice::Night))
            | (GuidedStage::ChooseAttributes, Some(ExaltationChoice::Eclipse)) => {
                GuidedStage::ChooseSolarCasteAbilities
            }
            (GuidedStage::ChooseSolarCasteAbilities, _) => GuidedStage::ChooseSolarSupernalAbility,
            (GuidedStage::ChooseSolarSupernalAbility, _) => {
                GuidedStage::ChooseSolarFavoredAbilities
            }
            (GuidedStage::ChooseSolarFavoredAbilities, _) => GuidedStage::ChooseMartialArtsStyles,
            (GuidedStage::ChooseMartialArtsStyles, _) => GuidedStage::ChooseSorcery,
            (GuidedStage::ChooseSorcery, _) => GuidedStage::ChooseAbilities,
            _ => {
                return Err(GuidedError::StageOrderError);
            }
        })
    }

    pub(in crate::guided) fn finalize_stage(&mut self) -> Result<&mut Self, GuidedError> {
        match self.stage {
            GuidedStage::ChooseSolarFavoredAbilities => {
                self.character_view
                    .set_solar_view(self.solar_traits()?)
                    .map_err(GuidedError::CharacterMutationError)?;
                Ok(self)
            }
            GuidedStage::ChooseSorcery => {
                if let Some(hashmap) = &self.martial_arts_styles {
                    if !hashmap.is_empty() {
                        self.character_view.set_ability_dots(AbilityNameVanilla::Brawl, 1).map_err(GuidedError::CharacterMutationError)?;
                    }
                }

                if self.shaping_ritual.is_some() && self.control_spell.is_some() && self.sorcery_archetype.is_some() {
                    self.character_view.set_ability_dots(AbilityNameVanilla::Occult, 3).map_err(GuidedError::CharacterMutationError)?;
                }

                if let Some(favored) = &self.solar_favored_abilities {
                    let favored_vanillas = favored.iter().filter_map(|not_vanilla| if let Ok(vanilla) = (*not_vanilla).try_into() {
                        Some(vanilla)
                    } else {
                        None
                    }).collect::<Vec<AbilityNameVanilla>>();

                    favored_vanillas.into_iter().fold(Ok(&mut self.character_view), |res_view, vanilla| {
                        res_view.and_then(|view| view.set_ability_dots(vanilla, 1))
                    }).map_err(GuidedError::CharacterMutationError)?;
                }

                Ok(self)
            }
            _ => Ok(self),
        }
    }
}