use std::{collections::{HashSet, HashMap}};

use crate::{
    abilities::AbilityName,
    exalt_state::exalt::exalt_type::solar::{
        validate_solar_caste_ability, Dawn, Eclipse, Night, Solar, Twilight, Zenith,
    },
    AttributeName, CharacterView, martial_arts::{MartialArtsStyleId, MartialArtsStyle, AddMartialArtsStyleError, RemoveMartialArtsStyleError}, CharacterMutationError,
};

use super::{
    error::{GuidedError, SolarAbilityError},
    ExaltationChoice, GuidedMutation, GuidedStage,
};

/// A view into the current state of the guided character builder, including
/// any partial or temporarily incomplete state.
pub struct GuidedView<'source> {
    pub(in crate::guided) character_view: CharacterView<'source>,
    pub(in crate::guided) stage: GuidedStage,
    pub(in crate::guided) bonus_points: i32,
    pub(in crate::guided) exaltation_choice: Option<ExaltationChoice>,
    pub(in crate::guided) solar_caste_abilities: Option<HashSet<AbilityName>>,
    pub(in crate::guided) solar_supernal_ability: Option<AbilityName>,
    pub(in crate::guided) solar_favored_abilities: Option<HashSet<AbilityName>>,
    pub(in crate::guided) martial_arts_styles: Option<HashMap<MartialArtsStyleId, &'source MartialArtsStyle>>,
}

impl<'source> GuidedView<'source> {
    /// Returns a new owned Solar object for the previously specified Caste,
    /// Supernal, and Favored abilities.
    pub fn solar_traits(&self) -> Result<Solar, GuidedError> {
        Ok(match self.exaltation_choice {
            None => return Err(GuidedError::StageOrderError),
            Some(ExaltationChoice::Dawn) => {
                let dawn = {
                    let mut builder = Dawn::builder();
                    self.solar_caste_abilities
                        .as_ref()
                        .ok_or(GuidedError::StageIncompleteError)?
                        .iter()
                        .for_each(|ability| {
                            builder
                                .add_caste_ability(*ability)
                                .expect("GuidedView should have valid caste abilities");
                        });
                    builder
                        .set_supernal_ability(
                            *self
                                .solar_supernal_ability
                                .as_ref()
                                .ok_or(GuidedError::StageIncompleteError)?,
                        )
                        .or(Err(GuidedError::StageIncompleteError))?;
                    builder.build().or(Err(GuidedError::StageIncompleteError))?
                };

                let mut builder = Solar::builder();
                builder.set_dawn(dawn);
                self.solar_favored_abilities
                    .as_ref()
                    .ok_or(GuidedError::StageIncompleteError)?
                    .iter()
                    .for_each(|ability| {
                        builder
                            .add_favored_ability(*ability)
                            .expect("GuidedView should have valid favored abilities");
                    });
                builder.build().or(Err(GuidedError::StageIncompleteError))?
            }
            Some(ExaltationChoice::Zenith) => {
                let zenith = {
                    let mut builder = Zenith::builder();
                    self.solar_caste_abilities
                        .as_ref()
                        .ok_or(GuidedError::StageIncompleteError)?
                        .iter()
                        .for_each(|ability| {
                            builder
                                .add_caste_ability(*ability)
                                .expect("GuidedView should have valid caste abilities");
                        });
                    builder
                        .set_supernal_ability(
                            self.solar_supernal_ability
                                .ok_or(GuidedError::StageIncompleteError)?,
                        )
                        .or(Err(GuidedError::StageIncompleteError))?;
                    builder.build().or(Err(GuidedError::StageIncompleteError))?
                };

                let mut builder = Solar::builder();
                builder.set_zenith(zenith);
                self.solar_favored_abilities
                    .as_ref()
                    .ok_or(GuidedError::StageIncompleteError)?
                    .iter()
                    .for_each(|ability| {
                        builder
                            .add_favored_ability(*ability)
                            .expect("GuidedView should have valid favored abilities");
                    });
                builder.build().or(Err(GuidedError::StageIncompleteError))?
            }
            Some(ExaltationChoice::Twilight) => {
                let twilight = {
                    let mut builder = Twilight::builder();
                    self.solar_caste_abilities
                        .as_ref()
                        .ok_or(GuidedError::StageIncompleteError)?
                        .iter()
                        .for_each(|ability| {
                            builder
                                .add_caste_ability(*ability)
                                .expect("GuidedView should have valid caste abilities");
                        });
                    builder
                        .set_supernal_ability(
                            self.solar_supernal_ability
                                .ok_or(GuidedError::StageIncompleteError)?,
                        )
                        .or(Err(GuidedError::StageIncompleteError))?;
                    builder.build().or(Err(GuidedError::StageIncompleteError))?
                };

                let mut builder = Solar::builder();
                builder.set_twilight(twilight);
                self.solar_favored_abilities
                    .as_ref()
                    .ok_or(GuidedError::StageIncompleteError)?
                    .iter()
                    .for_each(|ability| {
                        builder
                            .add_favored_ability(*ability)
                            .expect("GuidedView should have valid favored abilities");
                    });
                builder.build().or(Err(GuidedError::StageIncompleteError))?
            }
            Some(ExaltationChoice::Night) => {
                let night = {
                    let mut builder = Night::builder();
                    self.solar_caste_abilities
                        .as_ref()
                        .ok_or(GuidedError::StageIncompleteError)?
                        .iter()
                        .for_each(|ability| {
                            builder
                                .add_caste_ability(*ability)
                                .expect("GuidedView should have valid caste abilities");
                        });
                    builder
                        .set_supernal_ability(
                            self.solar_supernal_ability
                                .ok_or(GuidedError::StageIncompleteError)?,
                        )
                        .or(Err(GuidedError::StageIncompleteError))?;
                    builder.build().or(Err(GuidedError::StageIncompleteError))?
                };

                let mut builder = Solar::builder();
                builder.set_night(night);
                self.solar_favored_abilities
                    .as_ref()
                    .ok_or(GuidedError::StageIncompleteError)?
                    .iter()
                    .for_each(|ability| {
                        builder
                            .add_favored_ability(*ability)
                            .expect("GuidedView should have valid favored abilities");
                    });
                builder.build().or(Err(GuidedError::StageIncompleteError))?
            }
            Some(ExaltationChoice::Eclipse) => {
                let eclipse = {
                    let mut builder = Eclipse::builder();
                    self.solar_caste_abilities
                        .as_ref()
                        .ok_or(GuidedError::StageIncompleteError)?
                        .iter()
                        .for_each(|ability| {
                            builder
                                .add_caste_ability(*ability)
                                .expect("GuidedView should have valid caste abilities");
                        });
                    builder
                        .set_supernal_ability(
                            self.solar_supernal_ability
                                .ok_or(GuidedError::StageIncompleteError)?,
                        )
                        .or(Err(GuidedError::StageIncompleteError))?;
                    builder.build().or(Err(GuidedError::StageIncompleteError))?
                };

                let mut builder = Solar::builder();
                builder.set_eclipse(eclipse);
                self.solar_favored_abilities
                    .as_ref()
                    .ok_or(GuidedError::StageIncompleteError)?
                    .iter()
                    .for_each(|ability| {
                        builder
                            .add_favored_ability(*ability)
                            .expect("GuidedView should have valid favored abilities");
                    });
                builder.build().or(Err(GuidedError::StageIncompleteError))?
            }
            Some(_) => {
                return Err(GuidedError::StageOrderError);
            }
        })
    }

    fn attributes_buckets(&self) -> (u8, u8, u8) {
        let physical_attributes = self
            .character_view
            .attributes()
            .dots(AttributeName::Strength)
            + self
                .character_view
                .attributes()
                .dots(AttributeName::Dexterity)
            + self
                .character_view
                .attributes()
                .dots(AttributeName::Stamina);
        let mental_attributes = self
            .character_view
            .attributes()
            .dots(AttributeName::Perception)
            + self
                .character_view
                .attributes()
                .dots(AttributeName::Intelligence)
            + self.character_view.attributes().dots(AttributeName::Wits);
        let social_attributes = self
            .character_view
            .attributes()
            .dots(AttributeName::Charisma)
            + self
                .character_view
                .attributes()
                .dots(AttributeName::Manipulation)
            + self
                .character_view
                .attributes()
                .dots(AttributeName::Appearance);

        let primary = physical_attributes
            .max(mental_attributes)
            .max(social_attributes)
            - 3;
        let tertiary = physical_attributes
            .min(mental_attributes)
            .min(social_attributes)
            - 3;
        let secondary =
            physical_attributes + mental_attributes + social_attributes - primary - tertiary - 9;

        (primary, secondary, tertiary)
    }

    fn mortal_attributes_bonus_points_spent(&self) -> i32 {
        let (primary, secondary, tertiary) = self.attributes_buckets();
        ((primary - primary.min(6) + secondary - secondary.min(4)) * 4
            + (tertiary - tertiary.min(3)) * 3)
            .into()
    }

    fn solar_attributes_bonus_points_spent(&self) -> i32 {
        let (primary, secondary, tertiary) = self.attributes_buckets();
        ((primary - primary.min(8) + secondary - secondary.min(6)) * 4
            + (tertiary - tertiary.min(4)) * 3)
            .into()
    }

    pub(in crate::guided) fn update_bonus_points(&mut self) {
        if let Some(exaltation_choice) = self.exaltation_choice {
            match exaltation_choice {
                ExaltationChoice::Mortal => {
                    self.bonus_points = 21;
                    self.bonus_points -= self.mortal_attributes_bonus_points_spent();
                }
                ExaltationChoice::Dawn
                | ExaltationChoice::Zenith
                | ExaltationChoice::Twilight
                | ExaltationChoice::Night
                | ExaltationChoice::Eclipse => {
                    self.bonus_points = 15;
                    self.bonus_points -= self.solar_attributes_bonus_points_spent();
                }
            }
        } else {
            self.bonus_points = 0;
        }
    }

    fn validate_stage_complete(&self) -> Result<(), GuidedError> {
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
        } {
            Err(GuidedError::StageIncompleteError)
        } else {
            Ok(())
        }
    }

    /// The number of available Bonus Points to spend.
    pub fn bonus_points_remaining(&self) -> i32 {
        self.bonus_points
    }

    /// Applies a mutation to the character view. \n Note that unlike
    /// CharacterView::apply_mutation, this operation takes self and not &mut
    /// self. This is because a CharacterMutation may be valid for a
    /// CharacterView but invalid for a GuidedView; applying the mutation will
    /// leave the GuidedView in an invalid state that must be discarded.
    pub fn apply_mutation(
        mut self,
        guided_mutation: &'source GuidedMutation,
    ) -> Result<Self, GuidedError> {
        match guided_mutation {
            GuidedMutation::CharacterMutation(character_mutation) => {
                self.character_view
                    .apply_mutation(character_mutation)
                    .map_err(GuidedError::CharacterMutationError)?;
                self.update_bonus_points();
            }
            GuidedMutation::SetStage(next_stage) => {
                self.validate_stage_complete()?;

                match (self.stage, next_stage) {
                    (GuidedStage::ChooseNameAndConcept, GuidedStage::ChooseExaltation)
                    | (GuidedStage::ChooseExaltation, GuidedStage::ChooseAttributes) => Ok(()),
                    (GuidedStage::ChooseAttributes, GuidedStage::ChooseMartialArtsStyles) => {
                        if matches!(self.exaltation_choice, Some(ExaltationChoice::Mortal)) {
                            Ok(())
                        } else {
                            Err(GuidedError::StageOrderError)
                        }
                    }
                    (GuidedStage::ChooseAttributes, GuidedStage::ChooseSolarCasteAbilities)
                    | (
                        GuidedStage::ChooseSolarCasteAbilities,
                        GuidedStage::ChooseSolarSupernalAbility,
                    )
                    | (
                        GuidedStage::ChooseSolarSupernalAbility,
                        GuidedStage::ChooseSolarFavoredAbilities,
                    )
                    | (
                        GuidedStage::ChooseSolarFavoredAbilities,
                        GuidedStage::ChooseMartialArtsStyles,
                    ) => {
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
                }?;

                self.stage = *next_stage;
            }
            GuidedMutation::SetExaltation(exaltation_choice) => {
                if self.stage != GuidedStage::ChooseExaltation {
                    return Err(GuidedError::StageOrderError);
                }

                self.exaltation_choice = Some(*exaltation_choice);
                self.update_bonus_points();
            }
            GuidedMutation::AddSolarCasteAbility(ability) => {
                if self.stage != GuidedStage::ChooseSolarCasteAbilities {
                    return Err(GuidedError::StageOrderError);
                }

                if self.exaltation_choice.is_none() {
                    return Err(GuidedError::StageOrderError);
                }

                if !validate_solar_caste_ability(self.exaltation_choice.unwrap(), *ability) {
                    return Err(GuidedError::SolarAbilityError(
                        SolarAbilityError::InvalidCasteAbility,
                    ));
                }

                if self.solar_caste_abilities.is_none() {
                    self.solar_caste_abilities = Some(HashSet::new());
                }

                if self
                    .solar_caste_abilities
                    .as_ref()
                    .unwrap()
                    .contains(ability)
                {
                    return Err(GuidedError::SolarAbilityError(
                        SolarAbilityError::UniqueCasteAndFavored,
                    ));
                }

                if self.solar_caste_abilities.as_ref().unwrap().len() >= 5 {
                    return Err(GuidedError::SolarAbilityError(
                        SolarAbilityError::CasteAndFavoredCount,
                    ));
                }

                self.solar_caste_abilities
                    .as_mut()
                    .unwrap()
                    .insert(*ability);
            }
            GuidedMutation::RemoveSolarCasteAbility(ability) => {
                if self.stage != GuidedStage::ChooseSolarCasteAbilities {
                    return Err(GuidedError::StageOrderError);
                }

                if self.solar_caste_abilities.is_none() {
                    return Err(GuidedError::SolarAbilityError(SolarAbilityError::NotFound));
                }

                if !self.solar_caste_abilities.as_mut().unwrap().remove(ability) {
                    return Err(GuidedError::SolarAbilityError(SolarAbilityError::NotFound));
                }
            }
            GuidedMutation::SetSolarSupernalAbility(ability) => {
                if self.stage != GuidedStage::ChooseSolarSupernalAbility
                    || self.solar_caste_abilities.is_none()
                {
                    return Err(GuidedError::StageOrderError);
                }

                if ability == &AbilityName::MartialArts
                    && !self
                        .solar_caste_abilities
                        .as_ref()
                        .unwrap()
                        .contains(&AbilityName::Brawl)
                {
                    return Err(GuidedError::SolarAbilityError(
                        SolarAbilityError::SupernalIsCaste,
                    ));
                }

                if !self
                    .solar_caste_abilities
                    .as_ref()
                    .unwrap()
                    .contains(ability)
                {
                    return Err(GuidedError::SolarAbilityError(
                        SolarAbilityError::SupernalIsCaste,
                    ));
                }
            }
            GuidedMutation::AddSolarFavoredAbility(ability) => {
                if self.stage != GuidedStage::ChooseSolarFavoredAbilities {
                    return Err(GuidedError::StageOrderError);
                }

                if ability == &AbilityName::MartialArts {
                    return Err(GuidedError::SolarAbilityError(
                        SolarAbilityError::MartialArts,
                    ));
                }

                if self.solar_caste_abilities.is_none() {
                    return Err(GuidedError::StageOrderError);
                }

                if self
                    .solar_caste_abilities
                    .as_ref()
                    .unwrap()
                    .contains(ability)
                {
                    return Err(GuidedError::SolarAbilityError(
                        SolarAbilityError::UniqueCasteAndFavored,
                    ));
                }

                if self.solar_favored_abilities.is_none() {
                    self.solar_favored_abilities = Some(HashSet::new());
                }

                if self
                    .solar_favored_abilities
                    .as_ref()
                    .unwrap()
                    .contains(ability)
                {
                    return Err(GuidedError::SolarAbilityError(
                        SolarAbilityError::UniqueCasteAndFavored,
                    ));
                }

                if self.solar_favored_abilities.as_ref().unwrap().len() >= 5 {
                    return Err(GuidedError::SolarAbilityError(
                        SolarAbilityError::CasteAndFavoredCount,
                    ));
                }

                self.solar_favored_abilities
                    .as_mut()
                    .unwrap()
                    .insert(*ability);
            }
            GuidedMutation::RemoveSolarFavoredAbility(ability) => {
                if self.stage != GuidedStage::ChooseSolarFavoredAbilities {
                    return Err(GuidedError::StageOrderError);
                }

                if self.solar_favored_abilities.is_none() {
                    return Err(GuidedError::SolarAbilityError(SolarAbilityError::NotFound));
                }

                if !self
                    .solar_favored_abilities
                    .as_mut()
                    .unwrap()
                    .remove(ability)
                {
                    return Err(GuidedError::SolarAbilityError(SolarAbilityError::NotFound));
                }
            }
            GuidedMutation::AddMartialArtsStyle(id, style) => {
                if let Some(true) = self.martial_arts_styles.as_ref().map(|hashmap| hashmap.contains_key(id)) {
                    return Err(GuidedError::CharacterMutationError(CharacterMutationError::AddMartialArtsStyleError(AddMartialArtsStyleError::DuplicateStyle)));
                }

                if self.martial_arts_styles.is_none() {
                    self.martial_arts_styles = Some(HashMap::new());
                }

                self.martial_arts_styles.as_mut().unwrap().insert(*id, style);
            }
            GuidedMutation::RemoveMartialArtsStyle(id) => {
                if let Some(true) = self.martial_arts_styles.as_ref().map(|hashmap| hashmap.contains_key(id)) {
                    self.martial_arts_styles.as_mut().unwrap().remove(id);
                } else {
                    return Err(GuidedError::CharacterMutationError(CharacterMutationError::RemoveMartialArtsStyleError(RemoveMartialArtsStyleError::NotFound)));
                }
            }
        }

        if self.bonus_points < 0 {
            return Err(GuidedError::InsufficientBonusPoints);
        }

        Ok(self)
    }

    /// Gets a read-only view at the partially constructed character.
    pub fn as_character_view(&self) -> &CharacterView {
        &self.character_view
    }
}
