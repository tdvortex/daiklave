use std::collections::{HashMap, HashSet};

use crate::{
    abilities::{AbilityName, AbilityNameVanilla},
    attributes::AttributeName,
    exaltation::exalt::exalt_type::solar::{
        caste::{dawn::Dawn, eclipse::Eclipse, night::Night, twilight::Twilight, zenith::Zenith},
        Solar,
    },
    martial_arts::{
        AddMartialArtsStyleError, MartialArtsStyle, MartialArtsStyleId, RemoveMartialArtsStyleError,
    },
    sorcery::{
        ShapingRitual, ShapingRitualId, SorceryArchetype, SorceryArchetypeId, SpellId,
        TerrestrialSpell,
    },
    Character, CharacterMutation, CharacterMutationError,
};

use super::{
    error::{GuidedError, SolarAbilityError, SorceryError},
    guided_stage::GuidedStage,
    ExaltationChoice, GuidedMutation,
};

mod validate_solar_caste_ability;
use validate_solar_caste_ability::validate_solar_caste_ability;

/// A view into the current state of the guided character builder, including
/// any partial or temporarily incomplete state.
#[derive(Debug)]
pub struct GuidedState<'source> {
    pub(in crate::guided) character_view: Character<'source>,
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

impl<'source> GuidedState<'source> {
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
                if let CharacterMutation::SetAbilityDots(ability_name_vanilla, dots) =
                    character_mutation
                {
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
    pub fn as_character_view(&self) -> &Character {
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

    fn check_abilities_floor(
        &self,
        ability_name_vanilla: AbilityNameVanilla,
        dots: u8,
    ) -> Result<(), GuidedError> {
        if dots >= 3 {
            // 3 dots in any ability is enough to cover minimum values
            return Ok(());
        }

        if ability_name_vanilla == AbilityNameVanilla::Occult
            && self.character_view.sorcery().is_some()
        {
            return Err(GuidedError::AbilityMin);
        }

        if dots >= 1 {
            return Ok(());
        }

        if ability_name_vanilla == AbilityNameVanilla::Brawl
            && self.character_view.martial_arts().iter().next().is_some()
        {
            return Err(GuidedError::AbilityMin);
        }

        if let Some(solar_traits) = self.character_view.solar_traits() {
            if solar_traits.has_favored_ability(ability_name_vanilla.into()) {
                return Err(GuidedError::AbilityMin);
            }
        }

        Ok(())
    }

    /// Returns a new owned Solar object for the previously specified Caste,
    /// Supernal, and Favored abilities.
    pub(crate) fn solar_traits(&self) -> Result<Solar<'source>, GuidedError> {
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
                builder
                    .build_view()
                    .or(Err(GuidedError::StageIncompleteError))?
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
                builder
                    .build_view()
                    .or(Err(GuidedError::StageIncompleteError))?
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
                builder
                    .build_view()
                    .or(Err(GuidedError::StageIncompleteError))?
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
                builder
                    .build_view()
                    .or(Err(GuidedError::StageIncompleteError))?
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
                builder
                    .build_view()
                    .or(Err(GuidedError::StageIncompleteError))?
            }
            Some(_) => {
                return Err(GuidedError::StageOrderError);
            }
        })
    }

    pub(in crate::guided) fn add_solar_caste_ability(
        mut self,
        ability: &AbilityName,
    ) -> Result<Self, GuidedError> {
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

        Ok(self)
    }

    pub(in crate::guided) fn remove_solar_caste_ability(
        mut self,
        ability: &AbilityName,
    ) -> Result<Self, GuidedError> {
        if self.solar_caste_abilities.is_none() {
            return Err(GuidedError::SolarAbilityError(SolarAbilityError::NotFound));
        }

        if !self.solar_caste_abilities.as_mut().unwrap().remove(ability) {
            return Err(GuidedError::SolarAbilityError(SolarAbilityError::NotFound));
        }

        Ok(self)
    }

    pub(in crate::guided) fn set_solar_supernal_ability(
        mut self,
        ability: &AbilityName,
    ) -> Result<Self, GuidedError> {
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

        self.solar_supernal_ability = Some(*ability);

        Ok(self)
    }

    pub(in crate::guided) fn add_solar_favored_ability(
        mut self,
        ability: &AbilityName,
    ) -> Result<Self, GuidedError> {
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
        Ok(self)
    }

    pub(in crate::guided) fn remove_solar_favored_ability(
        mut self,
        ability: &AbilityName,
    ) -> Result<Self, GuidedError> {
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
        Ok(self)
    }

    pub(in crate::guided) fn attributes_buckets(&self) -> (u8, u8, u8) {
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

    fn mortal_merits_bonus_points_spent(&self) -> i32 {
        // Mortals get 7 free merit dots, the rest are 1 BP per dot
        self.merit_dots - self.merit_dots.min(7)
    }

    fn solar_merits_bonus_points_spent(&self) -> i32 {
        // Solars get 10 free merit dots, the rest are 1 BP per dot
        self.merit_dots - self.merit_dots.min(10)
    }

    fn mortal_ability_bonus_points_spent(&self) -> i32 {
        // Mortals get 28 free ability dots with a limit of 3 per skill
        // Dots at 4 or 5, or 29+ total, are 2 each
        let mut three_or_less = 0;
        let mut more_than_three = 0;

        for ability in self.character_view.abilities().iter() {
            let dots = ability.dots();
            three_or_less += dots.min(3);
            more_than_three += dots - dots.min(3);
        }

        (2 * (three_or_less - 28.min(three_or_less) + more_than_three)) as i32
    }

    fn solar_ability_bonus_points_spent(&self) -> i32 {
        if self.character_view.solar_traits().is_none() {
            // Solar traits are set before abilities
            return 0;
        }
        let solar_traits = self.character_view.solar_traits().unwrap();

        // Solars get 28 free ability dots with a limit of 3 per skill
        // Dots above 3 in a skill need to be purchases, as do dots 29+ at 3
        // or less
        // Caste or Favored skills cost 1 BP each, non-Caste non-Favored
        // abilities cost 2
        // Efficent allocation puts 28 free dots towards non-C/F skills first
        let mut cf_three_or_less = 0;
        let mut cf_more_than_three = 0;
        let mut not_cf_three_or_less = 0;
        let mut not_cf_more_than_three = 0;

        for ability in self.character_view.abilities().iter() {
            let dots = ability.dots();
            if solar_traits.has_caste_ability(ability.name())
                || solar_traits.has_favored_ability(ability.name())
            {
                cf_three_or_less += dots.min(3);
                cf_more_than_three += dots - dots.min(3);
            } else {
                not_cf_three_or_less += dots.min(3);
                not_cf_more_than_three += dots - dots.min(3);
            }
        }

        let three_or_less = cf_three_or_less + not_cf_three_or_less;
        let over_28 = three_or_less - 28.min(three_or_less);
        let discount = over_28.min(cf_three_or_less);

        (2 * (over_28 + not_cf_more_than_three) + cf_more_than_three - discount) as i32
    }

    pub(in crate::guided) fn update_bonus_points(&mut self) {
        if let Some(exaltation_choice) = self.exaltation_choice {
            match exaltation_choice {
                ExaltationChoice::Mortal => {
                    self.bonus_points = 21;
                    self.bonus_points -= self.mortal_attributes_bonus_points_spent();
                    self.bonus_points -= self.mortal_ability_bonus_points_spent();
                    self.bonus_points -= self.mortal_merits_bonus_points_spent();
                }
                ExaltationChoice::Dawn
                | ExaltationChoice::Zenith
                | ExaltationChoice::Twilight
                | ExaltationChoice::Night
                | ExaltationChoice::Eclipse => {
                    self.bonus_points = 15;
                    self.bonus_points -= self.solar_attributes_bonus_points_spent();
                    self.bonus_points -= self.solar_ability_bonus_points_spent();
                    self.bonus_points -= self.solar_merits_bonus_points_spent();
                }
            }
        } else {
            self.bonus_points = 0;
        }
    }

    /// The number of available Bonus Points to spend.
    pub fn bonus_points_remaining(&self) -> i32 {
        self.bonus_points
    }

    pub(in crate::guided) fn validate_correct_stage(
        &self,
        guided_mutation: &GuidedMutation,
    ) -> Result<(), GuidedError> {
        if match guided_mutation {
            GuidedMutation::CharacterMutation(mutation) => match mutation {
                CharacterMutation::SetName(_)
                | CharacterMutation::SetConcept(_)
                | CharacterMutation::RemoveConcept => self.stage == GuidedStage::NameAndConcept,
                CharacterMutation::SetAttribute(_, _) => self.stage == GuidedStage::Attributes,
                CharacterMutation::SetAbilityDots(_, _)
                | CharacterMutation::SetMartialArtsDots(_, _)
                | CharacterMutation::SetCraftDots(_, _) => self.stage == GuidedStage::Abilities,
                _ => false,
            },
            GuidedMutation::AdvanceStage => true,
            GuidedMutation::SetExaltation(_) => self.stage == GuidedStage::Exaltation,
            GuidedMutation::AddSolarCasteAbility(_)
            | GuidedMutation::RemoveSolarCasteAbility(_) => {
                self.stage == GuidedStage::SolarCasteAbilities
            }
            GuidedMutation::SetSolarSupernalAbility(_) => {
                self.stage == GuidedStage::SolarSupernalAbility
            }
            GuidedMutation::AddSolarFavoredAbility(_)
            | GuidedMutation::RemoveSolarFavoredAbility(_) => {
                self.stage == GuidedStage::SolarFavoredAbilities
            }
            GuidedMutation::AddMartialArtsStyle(_, _)
            | GuidedMutation::RemoveMartialArtsStyle(_) => {
                self.stage == GuidedStage::MartialArtsStyles
            }
            GuidedMutation::SetSorceryArchetype(_, _)
            | GuidedMutation::SetShapingRitual(_, _)
            | GuidedMutation::SetControlSpell(_, _) => self.stage == GuidedStage::Sorcery,
        } {
            Ok(())
        } else {
            Err(GuidedError::StageOrderError)
        }
    }

    pub(in crate::guided) fn validate_stage_complete(&self) -> Result<(), GuidedError> {
        if !match self.stage {
            GuidedStage::NameAndConcept => true,
            GuidedStage::Exaltation => self.exaltation_choice.is_some(),
            GuidedStage::Attributes => {
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
            GuidedStage::SolarCasteAbilities => {
                matches!(
                    self.solar_caste_abilities.as_ref().map(|v| v.len()),
                    Some(5)
                )
            }
            GuidedStage::SolarSupernalAbility => {
                matches!(self.solar_supernal_ability, Some(_))
            }
            GuidedStage::SolarFavoredAbilities => {
                matches!(
                    self.solar_favored_abilities.as_ref().map(|v| v.len()),
                    Some(5)
                )
            }
            GuidedStage::MartialArtsStyles => true,
            GuidedStage::Sorcery => {
                // Either no sorcery, or all sorcery is specified
                self.sorcery_archetype.is_some() == self.shaping_ritual.is_some()
                    && self.shaping_ritual.is_some() == self.control_spell.is_some()
            }
            GuidedStage::Abilities => {
                let three_or_less = self
                    .character_view
                    .abilities()
                    .iter()
                    .map(|ability| {
                        let dots = ability.dots();
                        dots.min(3)
                    })
                    .sum::<u8>();

                let craft_favored_met = if self
                    .character_view
                    .solar_traits()
                    .map(|solar_traits| solar_traits.has_favored_ability(AbilityName::Craft))
                    .unwrap_or(false)
                {
                    self.character_view
                        .craft()
                        .iter()
                        .map(|focus| self.character_view.craft().dots(focus).min(3))
                        .max()
                        .map_or(false, |max_craft| max_craft > 0)
                } else {
                    true
                };

                three_or_less == 28 && craft_favored_met
            }
            GuidedStage::Specialties => todo!(),
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
            (GuidedStage::NameAndConcept, _) => GuidedStage::Exaltation,
            (GuidedStage::Exaltation, _) => GuidedStage::Attributes,
            (GuidedStage::Attributes, Some(ExaltationChoice::Mortal)) => {
                GuidedStage::MartialArtsStyles
            }
            (GuidedStage::Attributes, Some(ExaltationChoice::Dawn))
            | (GuidedStage::Attributes, Some(ExaltationChoice::Zenith))
            | (GuidedStage::Attributes, Some(ExaltationChoice::Twilight))
            | (GuidedStage::Attributes, Some(ExaltationChoice::Night))
            | (GuidedStage::Attributes, Some(ExaltationChoice::Eclipse)) => {
                GuidedStage::SolarCasteAbilities
            }
            (GuidedStage::SolarCasteAbilities, _) => GuidedStage::SolarSupernalAbility,
            (GuidedStage::SolarSupernalAbility, _) => GuidedStage::SolarFavoredAbilities,
            (GuidedStage::SolarFavoredAbilities, _) => GuidedStage::MartialArtsStyles,
            (GuidedStage::MartialArtsStyles, _) => GuidedStage::Sorcery,
            (GuidedStage::Sorcery, _) => GuidedStage::Abilities,
            (GuidedStage::Abilities, _) => GuidedStage::Specialties,
            _ => {
                return Err(GuidedError::StageOrderError);
            }
        })
    }

    pub(in crate::guided) fn finalize_stage(&mut self) -> Result<&mut Self, GuidedError> {
        match self.stage {
            GuidedStage::SolarFavoredAbilities => {
                self.character_view
                    .set_solar_view(self.solar_traits()?)
                    .map_err(GuidedError::CharacterMutationError)?;

                if let Some(favored) = &self.solar_favored_abilities {
                    favored
                        .iter()
                        .filter_map(|not_vanilla| {
                            if let Ok(vanilla) = (*not_vanilla).try_into() {
                                Some(vanilla)
                            } else {
                                None
                            }
                        })
                        .fold(Ok(&mut self.character_view), |res_view, vanilla| {
                            res_view.and_then(|view| view.set_ability_dots(vanilla, 1))
                        })
                        .map_err(GuidedError::CharacterMutationError)?;
                }

                self.solar_caste_abilities = None;
                self.solar_supernal_ability = None;
                self.solar_favored_abilities = None;

                Ok(self)
            }
            GuidedStage::MartialArtsStyles => {
                if let Some(hashmap) = &self.martial_arts_styles {
                    if !hashmap.is_empty() {
                        self.character_view
                            .set_ability_dots(AbilityNameVanilla::Brawl, 1)
                            .map_err(GuidedError::CharacterMutationError)?;

                        for (style_id, style) in hashmap.iter() {
                            self.character_view
                                .add_martial_arts_style(*style_id, style)
                                .map_err(GuidedError::CharacterMutationError)?;
                        }
                    }
                }

                self.martial_arts_styles = None;

                Ok(self)
            }
            GuidedStage::Sorcery => {
                match (
                    self.shaping_ritual,
                    self.control_spell,
                    self.sorcery_archetype,
                ) {
                    (
                        Some((shaping_ritual_id, shaping_ritual)),
                        Some((control_spell_id, control_spell)),
                        Some((archetype_id, archetype)),
                    ) => {
                        self.character_view
                            .set_ability_dots(AbilityNameVanilla::Occult, 3)
                            .map_err(GuidedError::CharacterMutationError)?;
                        self.character_view
                            .add_terrestrial_sorcery(
                                archetype_id,
                                archetype,
                                shaping_ritual_id,
                                shaping_ritual,
                                control_spell_id,
                                control_spell,
                            )
                            .map_err(GuidedError::CharacterMutationError)?;
                        Ok(self)
                    }
                    (None, None, None) => Ok(self),
                    _ => Err(GuidedError::StageIncompleteError),
                }
            }
            _ => Ok(self),
        }
    }
}
