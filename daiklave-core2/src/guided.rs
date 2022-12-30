use std::collections::HashSet;

use crate::{abilities::AbilityName, CharacterMutation, CharacterView};

use self::{
    error::{GuidedError, SolarAbilityError},
    guided_view::GuidedView,
};

mod error;
mod guided_view;

/// Initiates a new guided character builder.
pub fn begin_guided_builder() -> GuidedEventSource {
    GuidedEventSource {
        history: Vec::new(),
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
    /// A stage for selecting whether to be a sorcerer or not, and if so, what
    /// Terrestrial shaping ritual they use, and what their Control Spell is. 
    /// This purchases either the Mortal Sorcerer merit if mortal, or the 
    /// Terrestrial Circle Sorcery Charm if Exalted, and forces Occult 3, but 
    /// does not purchase any non-Control Spells or associated Shaping Ritual 
    /// merits.
    ChooseSorcery,
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

/// An event-sourced guided character builder, supporting undo/redo.
pub struct GuidedEventSource {
    history: Vec<GuidedMutation>,
    future: Vec<GuidedMutation>,
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
                        if !abilities.remove(ability) {
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
                        if !abilities.remove(ability) {
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
