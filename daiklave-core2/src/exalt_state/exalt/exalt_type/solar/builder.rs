use std::collections::HashSet;

use thiserror::Error;

use crate::abilities::AbilityName;

use super::{caste::SolarCaste, Dawn, Eclipse, Night, Solar, Twilight, Zenith};

pub struct SolarTraitsBuilder {
    pub(in crate::exalt_state::exalt::exalt_type::solar) caste: Option<SolarCaste>,
    pub(in crate::exalt_state::exalt::exalt_type::solar) favored_abilities: HashSet<AbilityName>,
}

#[derive(Debug, Error)]
pub enum SolarBuilderError {
    #[error("Caste and Favored abilities must be unique")]
    UniqueCasteAndFavored,
    #[error("Required field missing: {0}")]
    MissingField(&'static str),
    #[error("Must have 5 Caste and 5 Favored abilities")]
    CasteAndFavoredCount,
    #[error("Martial Arts cannot be a Caste or Favored ability")]
    MartialArts,
    #[error("Must use correct caste abilities")]
    InvalidCasteAbility,
}

impl SolarTraitsBuilder {
    pub fn set_dawn(&mut self, dawn: Dawn) -> &mut Self {
        if !self.favored_abilities.is_empty() {
            self.favored_abilities.clear();
        }

        self.caste = Some(SolarCaste::Dawn(dawn));
        self
    }

    pub fn set_zenith(&mut self, zenith: Zenith) -> &mut Self {
        if !self.favored_abilities.is_empty() {
            self.favored_abilities.clear();
        }

        self.caste = Some(SolarCaste::Zenith(zenith));
        self
    }

    pub fn set_twilight(&mut self, twilight: Twilight) -> &mut Self {
        if !self.favored_abilities.is_empty() {
            self.favored_abilities.clear();
        }

        self.caste = Some(SolarCaste::Twilight(twilight));
        self
    }

    pub fn set_night(&mut self, night: Night) -> &mut Self {
        if !self.favored_abilities.is_empty() {
            self.favored_abilities.clear();
        }

        self.caste = Some(SolarCaste::Night(night));
        self
    }

    pub fn set_eclipse(&mut self, eclipse: Eclipse) -> &mut Self {
        if !self.favored_abilities.is_empty() {
            self.favored_abilities.clear();
        }

        self.caste = Some(SolarCaste::Eclipse(eclipse));
        self
    }

    pub fn add_favored_ability(
        &mut self,
        ability: AbilityName,
    ) -> Result<&mut Self, SolarBuilderError> {
        if ability == AbilityName::MartialArts {
            Err(SolarBuilderError::MartialArts)
        } else if self
            .caste
            .as_ref()
            .map_or(false, |c| c.has_caste_ability(ability))
            || !self.favored_abilities.insert(ability)
        {
            Err(SolarBuilderError::UniqueCasteAndFavored)
        } else {
            Ok(self)
        }
    }

    /// Consumes the builder to finalize Solar Traits.
    pub fn build(self) -> Result<Solar, SolarBuilderError> {
        if self.caste.is_none() {
            return Err(SolarBuilderError::MissingField("caste"));
        }

        if self.favored_abilities.len() != 5 {
            return Err(SolarBuilderError::CasteAndFavoredCount);
        }

        let mut option_arr = [None; 5];

        for (i, ability) in self.favored_abilities.into_iter().enumerate() {
            option_arr[i] = Some(ability);
        }

        let mut arr = option_arr.map(|el| el.unwrap());
        arr.sort();

        Ok(Solar {
            caste: self.caste.unwrap(),
            favored_abilities: arr,
        })
    }
}
