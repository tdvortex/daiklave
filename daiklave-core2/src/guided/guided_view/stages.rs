use crate::guided::{error::GuidedError, ExaltationChoice, GuidedStage};

use super::GuidedView;

impl<'source> GuidedView<'source> {
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
            GuidedStage::ChooseSorcery => todo!(),
            GuidedStage::ChooseAbilities => todo!(),
        } {
            Err(GuidedError::StageIncompleteError)
        } else {
            Ok(())
        }
    }

    pub(in crate::guided) fn validate_stage_order(
        &self,
        next_stage: &GuidedStage,
    ) -> Result<(), GuidedError> {
        match (self.stage, next_stage) {
            (GuidedStage::ChooseNameAndConcept, GuidedStage::ChooseExaltation)
            | (GuidedStage::ChooseExaltation, GuidedStage::ChooseAttributes)
            | (GuidedStage::ChooseMartialArtsStyles, GuidedStage::ChooseSorcery) => Ok(()),
            (GuidedStage::ChooseAttributes, GuidedStage::ChooseMartialArtsStyles) => {
                if matches!(self.exaltation_choice, Some(ExaltationChoice::Mortal)) {
                    Ok(())
                } else {
                    Err(GuidedError::StageOrderError)
                }
            }
            (GuidedStage::ChooseAttributes, GuidedStage::ChooseSolarCasteAbilities)
            | (GuidedStage::ChooseSolarCasteAbilities, GuidedStage::ChooseSolarSupernalAbility)
            | (GuidedStage::ChooseSolarSupernalAbility, GuidedStage::ChooseSolarFavoredAbilities)
            | (GuidedStage::ChooseSolarFavoredAbilities, GuidedStage::ChooseMartialArtsStyles) => {
                if matches!(
                    self.exaltation_choice,
                    Some(ExaltationChoice::Dawn)
                        | Some(ExaltationChoice::Zenith)
                        | Some(ExaltationChoice::Twilight)
                        | Some(ExaltationChoice::Night)
                        | Some(ExaltationChoice::Eclipse)
                ) {
                    Ok(())
                } else {
                    Err(GuidedError::StageOrderError)
                }
            }
            _ => Err(GuidedError::StageOrderError),
        }
    }
}
