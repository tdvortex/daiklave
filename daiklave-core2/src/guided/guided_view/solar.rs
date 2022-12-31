use std::collections::HashSet;

use crate::{
    abilities::AbilityName,
    exalt_state::exalt::exalt_type::solar::validate_solar_caste_ability,
    guided::{
        error::{GuidedError, SolarAbilityError},
        GuidedStage,
    },
};

use super::GuidedView;

mod solar_traits;

impl<'source> GuidedView<'source> {
    pub(in crate::guided) fn add_solar_caste_ability(
        mut self,
        ability: &AbilityName,
    ) -> Result<Self, GuidedError> {
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

        Ok(self)
    }

    pub(in crate::guided) fn remove_solar_caste_ability(
        mut self,
        ability: &AbilityName,
    ) -> Result<Self, GuidedError> {
        if self.stage != GuidedStage::ChooseSolarCasteAbilities {
            return Err(GuidedError::StageOrderError);
        }

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

        self.solar_supernal_ability = Some(*ability);

        Ok(self)
    }

    pub(in crate::guided) fn add_solar_favored_ability(
        mut self,
        ability: &AbilityName,
    ) -> Result<Self, GuidedError> {
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
        Ok(self)
    }

    pub(in crate::guided) fn remove_solar_favored_ability(
        mut self,
        ability: &AbilityName,
    ) -> Result<Self, GuidedError> {
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
        Ok(self)
    }
}
