use std::collections::{HashMap};

use serde::{Deserialize, Serialize};

use crate::{
    abilities::{Ability, AbilityView, SetAbilityError},
    exalt_state::exalt::{Exalt, ExaltView},
    CharacterMutationError,
};

use super::{
    AddMartialArtsStyleError, MartialArtsCharm, MartialArtsCharmId, MartialArtsStyle,
    MartialArtsStyleId, RemoveMartialArtsStyleError, SetMartialArtsError,
};

impl Exalt {
    pub(crate) fn check_add_martial_arts_style(
        &self,
        id: MartialArtsStyleId,
        _style: &MartialArtsStyle,
    ) -> Result<(), CharacterMutationError> {
        if self.martial_arts_styles.contains_key(&id) {
            Err(CharacterMutationError::AddMartialArtsStyleError(
                AddMartialArtsStyleError::DuplicateStyle,
            ))
        } else {
            Ok(())
        }
    }

    pub(crate) fn add_martial_arts_style(
        &mut self,
        id: MartialArtsStyleId,
        style: &MartialArtsStyle,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.check_add_martial_arts_style(id, style)?;
        self.martial_arts_styles.insert(
            id,
            ExaltMartialArtist {
                style: style.to_owned(),
                ability: Ability::Zero,
                charms: HashMap::new(),
            },
        );
        Ok(self)
    }

    pub(crate) fn check_remove_martial_arts_style(
        &self,
        id: MartialArtsStyleId,
    ) -> Result<(), CharacterMutationError> {
        if !self.martial_arts_styles.contains_key(&id) {
            Err(CharacterMutationError::RemoveMartialArtsStyleError(
                RemoveMartialArtsStyleError::NotFound,
            ))
        } else {
            Ok(())
        }
    }

    pub(crate) fn remove_martial_arts_style(
        &mut self,
        id: MartialArtsStyleId,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.check_remove_martial_arts_style(id)?;
        self.martial_arts_styles.remove(&id);
        Ok(self)
    }

    pub(crate) fn check_set_martial_arts_dots(
        &self,
        id: MartialArtsStyleId,
        dots: u8,
    ) -> Result<(), CharacterMutationError> {
        if dots > 5 {
            Err(CharacterMutationError::SetAbilityError(SetAbilityError::InvalidRating(dots)))
        } else if let Some(style) = self.martial_arts_styles.get(&id) {
            Ok(())
        } else {
            Err(CharacterMutationError::SetMartialArtsError(SetMartialArtsError::NotFound))
        }
    }

    pub(crate) fn set_martial_arts_dots(
        &mut self,
        id: MartialArtsStyleId,
        dots: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        if dots > 5 {
            Err(CharacterMutationError::SetAbilityError(SetAbilityError::InvalidRating(dots)))
        } else if let Some(style) = self.martial_arts_styles.get_mut(&id) {
            if dots >= style.ability.dots() {
                style.ability.set_dots(dots)?;
                Ok(self)
            } else {
                // May have to remove charms
                let mut prereq_charms_map = HashMap::<MartialArtsCharmId, Vec<MartialArtsCharmId>>::new();
                let mut removal_stack = Vec::<MartialArtsCharmId>::new();
    
                for (charm_id, charm) in style.charms.iter() {
                    for prereq_charm_id in charm.charms_required.iter() {
                        prereq_charms_map.entry(*prereq_charm_id).or_insert(Vec::new()).push(*charm_id);
                    }

                    if charm.ability_required > dots {
                        removal_stack.push(*charm_id);
                    }
                }

                while let Some(id_to_remove) = removal_stack.pop() {
                    style.charms.remove(&id_to_remove);
                    if let Some(dependents) = prereq_charms_map.remove(&id_to_remove) {
                        for dependent_id in dependents.iter() {
                            removal_stack.push(*dependent_id);
                        }
                    }
                }
    
                Ok(self)
            }       
        } else {
            Err(CharacterMutationError::SetMartialArtsError(SetMartialArtsError::NotFound))
        }
    }
}

impl<'source> ExaltView<'source> {
    pub(crate) fn check_add_martial_arts_style(
        &self,
        id: MartialArtsStyleId,
        _style: &MartialArtsStyle,
    ) -> Result<(), CharacterMutationError> {
        if self.martial_arts_styles.contains_key(&id) {
            Err(CharacterMutationError::AddMartialArtsStyleError(
                AddMartialArtsStyleError::DuplicateStyle,
            ))
        } else {
            Ok(())
        }
    }

    pub(crate) fn add_martial_arts_style(
        &mut self,
        id: MartialArtsStyleId,
        style: &'source MartialArtsStyle,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.check_add_martial_arts_style(id, style)?;
        self.martial_arts_styles.insert(
            id,
            ExaltMartialArtistView {
                style,
                ability: AbilityView::Zero,
                charms: HashMap::new(),
            },
        );
        Ok(self)
    }

    pub(crate) fn check_remove_martial_arts_style(
        &self,
        id: MartialArtsStyleId,
    ) -> Result<(), CharacterMutationError> {
        if self.martial_arts_styles.contains_key(&id) {
            Err(CharacterMutationError::RemoveMartialArtsStyleError(
                RemoveMartialArtsStyleError::NotFound,
            ))
        } else {
            Ok(())
        }
    }

    pub(crate) fn remove_martial_arts_style(
        &mut self,
        id: MartialArtsStyleId,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.check_remove_martial_arts_style(id)?;
        self.martial_arts_styles.remove(&id);
        Ok(self)
    }

    pub(crate) fn check_set_martial_arts_dots(
        &self,
        id: MartialArtsStyleId,
        dots: u8,
    ) -> Result<(), CharacterMutationError> {
        if dots > 5 {
            Err(CharacterMutationError::SetAbilityError(SetAbilityError::InvalidRating(dots)))
        } else if let Some(style) = self.martial_arts_styles.get(&id) {
            Ok(())
        } else {
            Err(CharacterMutationError::SetMartialArtsError(SetMartialArtsError::NotFound))
        }
    }

    pub(crate) fn set_martial_arts_dots(
        &mut self,
        id: MartialArtsStyleId,
        dots: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        if dots > 5 {
            Err(CharacterMutationError::SetAbilityError(SetAbilityError::InvalidRating(dots)))
        } else if let Some(style) = self.martial_arts_styles.get_mut(&id) {
            if dots >= style.ability.dots() {
                style.ability.set_dots(dots)?;
                Ok(self)
            } else {
                // May have to remove charms
                let mut prereq_charms_map = HashMap::<MartialArtsCharmId, Vec<MartialArtsCharmId>>::new();
                let mut removal_stack = Vec::<MartialArtsCharmId>::new();
    
                for (charm_id, charm) in style.charms.iter() {
                    for prereq_charm_id in charm.charms_required.iter() {
                        prereq_charms_map.entry(*prereq_charm_id).or_insert(Vec::new()).push(*charm_id);
                    }

                    if charm.ability_required > dots {
                        removal_stack.push(*charm_id);
                    }
                }

                while let Some(id_to_remove) = removal_stack.pop() {
                    style.charms.remove(&id_to_remove);
                    if let Some(dependents) = prereq_charms_map.remove(&id_to_remove) {
                        for dependent_id in dependents.iter() {
                            removal_stack.push(*dependent_id);
                        }
                    }
                }
    
                Ok(self)
            }       
        } else {
            Err(CharacterMutationError::SetMartialArtsError(SetMartialArtsError::NotFound))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ExaltMartialArtist {
    pub(in crate::martial_arts) style: MartialArtsStyle,
    pub(in crate::martial_arts) ability: Ability,
    pub(in crate::martial_arts) charms: HashMap<MartialArtsCharmId, MartialArtsCharm>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExaltMartialArtistView<'source> {
    pub(in crate::martial_arts) style: &'source MartialArtsStyle,
    pub(in crate::martial_arts) ability: AbilityView<'source>,
    pub(in crate::martial_arts) charms: HashMap<MartialArtsCharmId, &'source MartialArtsCharm>,
}
