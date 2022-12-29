use std::collections::HashSet;

use thiserror::Error;

use crate::{
    abilities::AbilityName, id::CharacterId, solar::validate_solar_caste_ability, AttributeName,
    CharacterMutation, CharacterMutationError, CharacterView, Dawn, Eclipse, Night, Solar,
    Twilight, Zenith,
};

/// Initiates a new guided character builder.
pub fn begin_guided_builder(id: CharacterId) -> GuidedEventSource {
    GuidedEventSource {
        history: vec![GuidedMutation::CharacterMutation(CharacterMutation::SetId(
            id,
        ))],
        future: Vec::new(),
    }
}

/// The operations you can do during a guided character building process.
pub enum GuidedMutation {
    /// Apply a standard character mutation (with additional validation).
    CharacterMutation(CharacterMutation),
    /// Move on to the next stage of the builder. Note that because different
    /// Exalt types have different stages, some stages may be skipped or done
    /// in a different order.
    SetStage(GuidedStage),
    /// Choose a specific Exalt type (or Mortal), without necessarily setting
    /// all exaltations up-front.
    SetExaltation(ExaltationChoice),
    /// Add a Solar Caste ability to the guided builder.
    AddSolarCasteAbility(AbilityName),
    /// Removes a Solar Caste ability from the guided builder.
    RemoveSolarCasteAbility(AbilityName),
    /// Sets the Solar's Supernal ability.
    SetSolarSupernalAbility(AbilityName),
    /// Add a Solar Favored ability to the guided builder.
    AddSolarFavoredAbility(AbilityName),
    /// Remove a Solar Favored ability from the guided builder.
    RemoveSolarFavoredAbility(AbilityName),
}

/// An event-sourced guided character builder, supporting undo/redo.
pub struct GuidedEventSource {
    history: Vec<GuidedMutation>,
    future: Vec<GuidedMutation>,
}

/// A view into the current state of the guided character builder, including
/// any partial or temporarily incomplete state.
pub struct GuidedView<'source> {
    character_view: CharacterView<'source>,
    stage: GuidedStage,
    bonus_points: i32,
    exaltation_choice: Option<ExaltationChoice>,
    solar_caste_abilities: Option<HashSet<AbilityName>>,
    solar_supernal_ability: Option<AbilityName>,
    solar_favored_abilities: Option<HashSet<AbilityName>>,
}

impl GuidedEventSource {
    /// Derives the current state of the partially-complete character,
    /// including all state which is character-creation-only (like bonus points)
    pub fn as_guided_view(&self) -> Result<GuidedView, GuidedError> {
        let mut guided_view = GuidedView {
            character_view: CharacterView::default(),
            stage: GuidedStage::ChooseNameAndConcept,
            bonus_points: 0,
            exaltation_choice: None,
            solar_caste_abilities: None,
            solar_supernal_ability: None,
            solar_favored_abilities: None,
        };

        // Don't use GuidedView::apply_mutation() to avoid redundant bonus
        // point recalculations and unnecessary validity checks
        for guided_mutation in self.history.iter() {
            match guided_mutation {
                GuidedMutation::CharacterMutation(character_mutation) => {
                    guided_view
                        .character_view
                        .apply_mutation(character_mutation)?;
                }
                GuidedMutation::SetStage(stage) => {
                    guided_view.stage = *stage;
                }
                GuidedMutation::SetExaltation(exaltation_choice) => {
                    guided_view.exaltation_choice = Some(*exaltation_choice);
                }
                GuidedMutation::AddSolarCasteAbility(ability) => {
                    if guided_view.solar_caste_abilities.is_none() {
                        guided_view.solar_caste_abilities = Some(HashSet::new());
                    }

                    guided_view
                        .solar_caste_abilities
                        .as_mut()
                        .unwrap()
                        .insert(*ability);
                }
                GuidedMutation::RemoveSolarCasteAbility(ability) => {
                    if let Some(abilities) = guided_view.solar_caste_abilities.as_mut() {
                        if !abilities.remove(&ability) {
                            return Err(GuidedError::SolarAbilityError(
                                SolarAbilityError::NotFound,
                            ));
                        }
                    } else {
                        return Err(GuidedError::SolarAbilityError(SolarAbilityError::NotFound));
                    }
                }
                GuidedMutation::SetSolarSupernalAbility(ability) => {
                    guided_view.solar_supernal_ability = Some(*ability);
                }
                GuidedMutation::AddSolarFavoredAbility(ability) => {
                    if guided_view.solar_favored_abilities.is_none() {
                        guided_view.solar_favored_abilities = Some(HashSet::new());
                    }

                    guided_view
                        .solar_favored_abilities
                        .as_mut()
                        .unwrap()
                        .insert(*ability);
                }
                GuidedMutation::RemoveSolarFavoredAbility(ability) => {
                    if let Some(abilities) = guided_view.solar_favored_abilities.as_mut() {
                        if !abilities.remove(&ability) {
                            return Err(GuidedError::SolarAbilityError(
                                SolarAbilityError::NotFound,
                            ));
                        }
                    } else {
                        return Err(GuidedError::SolarAbilityError(SolarAbilityError::NotFound));
                    }
                }
            }
        }
        guided_view.update_bonus_points();

        Ok(guided_view)
    }
}

impl<'source> GuidedView<'source> {
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

    fn update_bonus_points(&mut self) {
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
            GuidedStage::ChooseMartialArtsStyles => todo!(),
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
                    .contains(&ability)
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

                if ability == &AbilityName::MartialArts {
                    if !self
                        .solar_caste_abilities
                        .as_ref()
                        .unwrap()
                        .contains(&AbilityName::Brawl)
                    {
                        return Err(GuidedError::SolarAbilityError(
                            SolarAbilityError::SupernalIsCaste,
                        ));
                    }
                }

                if !self
                    .solar_caste_abilities
                    .as_ref()
                    .unwrap()
                    .contains(&ability)
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
                    .contains(&ability)
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
                    .contains(&ability)
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

/// The different phases of a guided character builder.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuidedStage {
    /// The first stage, choosing a character name and (optional) concept.
    ChooseNameAndConcept,
    /// The second stage, choosing the Exaltation for the character (or Mortal).
    ChooseExaltation,
    /// The attribute selection stage. Comes after ChooseExaltation for
    /// Mortals and Solars.
    ChooseAttributes,
    /// The stage where Solars pick five Caste abilities from the 7 available
    /// for their Caste.
    ChooseSolarCasteAbilities,
    /// The stage where Solars pick their Supernal ability from the 5 Caste
    /// abilities they previously selected, except that Dawn castes may
    /// instead pick Martial Arts if Brawl is a selected caste ability.
    ChooseSolarSupernalAbility,
    /// The stage where Solars pick their Favored abilities.
    ChooseSolarFavoredAbilities,
    /// A stage for selecting which Martial Arts styles (if any) the character
    /// practices. This purchases the MartialArtist merit and forces Brawl 1
    /// but does not purchase any MartialArts dots, specialties, or charms.
    ChooseMartialArtsStyles,
}

/// The supported options for Exaltations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExaltationChoice {
    /// No exaltation, just a heroic mortal.
    Mortal,
    /// Dawn caste Solar.
    Dawn,
    /// Zenith caste Solar.
    Zenith,
    /// Twilight caste Solar.
    Twilight,
    /// Night caste Solar.
    Night,
    /// Eclipse caste Solar.
    Eclipse,
}

/// An error trying to set or remove a Solar caste, supernal, or favored
/// ability
#[derive(Debug, Error)]
pub enum SolarAbilityError {
    /// Solar caste and favored abilities must be unique.
    #[error("Cannot have duplicate Caste or Favored abilities")]
    UniqueCasteAndFavored,
    /// Referencing an absent ability
    #[error("Could not find ability")]
    NotFound,
    /// Supernal abilities must first be selected as Caste abilities, unless
    /// MartialArts is Supernal, in which case Brawl must be a Caste ability.
    #[error("Supernal ability must be a selected Caste ability")]
    SupernalIsCaste,
    /// Must use correct abilities for the chosen Caste
    #[error("Not a caste ability")]
    InvalidCasteAbility,
    /// Must have exactly 5 Caste abilities and 5 Favored abilities.
    #[error("Incorrect number of Caste and Favored abilities")]
    CasteAndFavoredCount,
    /// Martial Arts cannot be either a Caste or Favored ability (implied by
    /// having Brawl as Caste/Favored).
    #[error("MartialArts cannot be Caster or Favored")]
    MartialArts,
}

/// The possible errors occurring in the guided character builder.
#[derive(Debug, Error)]
pub enum GuidedError {
    /// An error in applying the mutation to the base character
    #[error("Could not apply mutation to base character")]
    CharacterMutationError(#[from] CharacterMutationError),
    /// An error in trying to move stages in the wrong order
    #[error("Cannot move stages out of order")]
    StageOrderError,
    /// An error trying to move because previous stage is not complete
    #[error("Cannot move to the next stage while previous is incomplete")]
    StageIncompleteError,
    /// An error in trying to spend more bonus points than are available
    #[error("Cannot spend more bonus points than are available")]
    InsufficientBonusPoints,
    /// An error trying to set or remove a Solar caste, supernal, or favored
    /// ability
    #[error("Could not add a Solar caste ability")]
    SolarAbilityError(#[from] SolarAbilityError),
}

impl GuidedEventSource {
    /// Checks if a GuidedCharacterMutation can be successfully applied.
    pub fn check_mutation(&self, mutation: &GuidedMutation) -> Result<(), GuidedError> {
        self.as_guided_view()?.apply_mutation(mutation)?;
        Ok(())
    }

    /// Apply a mutation, inserting it into the event history. This will erase
    /// all previously undone operations.
    pub fn apply_mutation(&mut self, mutation: GuidedMutation) -> Result<&mut Self, GuidedError> {
        self.check_mutation(&mutation)?;
        self.future = Vec::new();
        self.history.push(mutation);

        Ok(self)
    }

    /// Returns true if there is an operation which can be undone.
    pub fn can_undo(&self) -> bool {
        self.history.len() > 1 // Don't undo SetId
    }

    /// Attempts to undo the previous operation, returns true if successful.
    pub fn undo(&mut self) -> bool {
        if self.can_undo() {
            self.future.push(self.history.pop().unwrap());
            true
        } else {
            false
        }
    }

    /// Returns true if there is an operation which can be redone.
    pub fn can_redo(&self) -> bool {
        !self.future.is_empty()
    }

    /// Attempts to redo the last undone operation, returns true if successful.
    pub fn redo(&mut self) -> bool {
        if self.can_redo() {
            self.history.push(self.future.pop().unwrap());
            true
        } else {
            false
        }
    }
}
