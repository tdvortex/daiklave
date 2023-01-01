use std::collections::{HashMap, HashSet};

use crate::{
    abilities::{AbilityName, AbilityNameVanilla},
    martial_arts::{
        AddMartialArtsStyleError, MartialArtsStyle, MartialArtsStyleId, RemoveMartialArtsStyleError,
    },
    sorcery::{
        ShapingRitual, ShapingRitualId, SorceryArchetype, SorceryArchetypeId, SpellId,
        TerrestrialSpell,
    },
    CharacterMutationError, CharacterView, CharacterMutation,
};

use super::{
    error::{GuidedError, SorceryError},
    ExaltationChoice, GuidedMutation, GuidedStage,
};

mod bonus_points;
mod solar;
mod stages;

/// A view into the current state of the guided character builder, including
/// any partial or temporarily incomplete state.
#[derive(Debug)]
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
    pub(in crate::guided) sorcery_archetype:
        Option<(SorceryArchetypeId, &'source SorceryArchetype)>,
    pub(in crate::guided) shaping_ritual: Option<(ShapingRitualId, &'source ShapingRitual)>,
    pub(in crate::guided) control_spell: Option<(SpellId, &'source TerrestrialSpell)>,
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
        self.validate_correct_stage(guided_mutation)?;

        match guided_mutation {
            GuidedMutation::CharacterMutation(character_mutation) => {
                if let CharacterMutation::SetAbilityDots(ability_name_vanilla, dots) = character_mutation {
                    self.check_abilities_floor(*ability_name_vanilla, *dots)?;
                }

                self.character_view
                    .apply_mutation(character_mutation)
                    .map_err(GuidedError::CharacterMutationError)?;
                self.update_bonus_points();
            }
            GuidedMutation::AdvanceStage => {
                self.validate_stage_complete()?;
                self.finalize_stage()?;

                self.stage = self.next_stage()?;
            }
            GuidedMutation::SetExaltation(exaltation_choice) => {
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
                self.update_bonus_points();
            }
            GuidedMutation::RemoveMartialArtsStyle(id) => {
                if let Some(true) = self
                    .martial_arts_styles
                    .as_ref()
                    .map(|hashmap| hashmap.contains_key(id))
                {
                    self.martial_arts_styles.as_mut().unwrap().remove(id);
                    self.merit_dots -= 4;
                    self.update_bonus_points();
                } else {
                    return Err(GuidedError::CharacterMutationError(
                        CharacterMutationError::RemoveMartialArtsStyleError(
                            RemoveMartialArtsStyleError::NotFound,
                        ),
                    ));
                }
            }
            GuidedMutation::SetSorceryArchetype(id, archetype) => {
                if let Some(ExaltationChoice::Mortal) = self.exaltation_choice {
                    if self.sorcery_archetype.is_none() {
                        self.merit_dots += 5;
                    }
                }
                self.shaping_ritual = None;
                self.sorcery_archetype = Some((*id, archetype));
                self.update_bonus_points();
            }
            GuidedMutation::SetShapingRitual(id, shaping_ritual) => {
                if self.sorcery_archetype.is_none()
                    || shaping_ritual.archetype_id() != self.sorcery_archetype.unwrap().0
                {
                    return Err(GuidedError::SorceryError(SorceryError::MissingArchetype));
                }

                self.shaping_ritual = Some((*id, shaping_ritual));
            }
            GuidedMutation::SetControlSpell(id, spell) => {
                self.control_spell = Some((*id, spell));
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

    pub fn merit_dots(&self) -> i32 {
        self.merit_dots
    }

    pub fn charms(&self) -> i32 {
        i32::from(
            self.sorcery_archetype.is_some()
                || self.shaping_ritual.is_some()
                || self.control_spell.is_some(),
        )
    }

    fn check_abilities_floor(&self, ability_name_vanilla: AbilityNameVanilla, dots: u8) -> Result<(), GuidedError> {
        if dots >= 3 {
            // 3 dots in any ability is enough to cover minimum values
            return Ok(());
        }
        
        if ability_name_vanilla == AbilityNameVanilla::Occult && self.character_view.sorcery().is_some() {
            return Err(GuidedError::AbilityMin);
        } 
        
        if dots >= 1 {
            return Ok(());
        }

        if ability_name_vanilla == AbilityNameVanilla::Brawl && self.character_view.martial_arts().iter().next().is_some() {
            return Err(GuidedError::AbilityMin);
        }
        
        if let Some(favored) = &self.solar_favored_abilities {
            if favored.contains(&ability_name_vanilla.into()) {
                return Err(GuidedError::AbilityMin);
            }
        }

        Ok(())
    }
}
