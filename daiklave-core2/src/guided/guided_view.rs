use std::collections::{HashMap, HashSet};

use crate::{
    abilities::AbilityName,
    martial_arts::{
        AddMartialArtsStyleError, MartialArtsStyle, MartialArtsStyleId, RemoveMartialArtsStyleError,
    },
    CharacterMutationError, CharacterView,
};

use super::{error::GuidedError, ExaltationChoice, GuidedMutation, GuidedStage};

mod bonus_points;
mod solar;
mod stages;

/// A view into the current state of the guided character builder, including
/// any partial or temporarily incomplete state.
pub struct GuidedView<'source> {
    pub(in crate::guided) character_view: CharacterView<'source>,
    pub(in crate::guided) stage: GuidedStage,
    pub(in crate::guided) bonus_points: i32,
    pub(in crate::guided) merit_dots: i32,
    pub(in crate::guided) exaltation_choice: Option<ExaltationChoice>,
    pub(in crate::guided) solar_caste_abilities: Option<HashSet<AbilityName>>,
    pub(in crate::guided) solar_supernal_ability: Option<AbilityName>,
    pub(in crate::guided) solar_favored_abilities: Option<HashSet<AbilityName>>,
    pub(in crate::guided) martial_arts_styles:
        Option<HashMap<MartialArtsStyleId, &'source MartialArtsStyle>>,
}

impl<'source> GuidedView<'source> {
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
                self.validate_stage_order(next_stage)?;
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
                self = self.add_solar_caste_ability(ability)?;
            }
            GuidedMutation::RemoveSolarCasteAbility(ability) => {
                self = self.remove_solar_caste_ability(ability)?;
            }
            GuidedMutation::SetSolarSupernalAbility(ability) => {
                self = self.set_solar_supernal_ability(ability)?;
            }
            GuidedMutation::AddSolarFavoredAbility(ability) => {
                self = self.add_solar_favored_ability(ability)?;
            }
            GuidedMutation::RemoveSolarFavoredAbility(ability) => {
                self = self.remove_solar_favored_ability(ability)?;
            }
            GuidedMutation::AddMartialArtsStyle(id, style) => {
                if let Some(true) = self
                    .martial_arts_styles
                    .as_ref()
                    .map(|hashmap| hashmap.contains_key(id))
                {
                    return Err(GuidedError::CharacterMutationError(
                        CharacterMutationError::AddMartialArtsStyleError(
                            AddMartialArtsStyleError::DuplicateStyle,
                        ),
                    ));
                }

                if self.martial_arts_styles.is_none() {
                    self.martial_arts_styles = Some(HashMap::new());
                }

                self.martial_arts_styles
                    .as_mut()
                    .unwrap()
                    .insert(*id, style);
                self.merit_dots += 4;
            }
            GuidedMutation::RemoveMartialArtsStyle(id) => {
                if let Some(true) = self
                    .martial_arts_styles
                    .as_ref()
                    .map(|hashmap| hashmap.contains_key(id))
                {
                    self.martial_arts_styles.as_mut().unwrap().remove(id);
                    self.merit_dots -= 4;
                } else {
                    return Err(GuidedError::CharacterMutationError(
                        CharacterMutationError::RemoveMartialArtsStyleError(
                            RemoveMartialArtsStyleError::NotFound,
                        ),
                    ));
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
