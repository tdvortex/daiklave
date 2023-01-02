use std::collections::HashSet;

use crate::abilities::AbilityName;

use super::{
    builder_error::SolarBuilderError,
    caste::{
        dawn::Dawn, eclipse::Eclipse, night::Night, twilight::Twilight, zenith::Zenith, SolarCaste,
    },
    sorcery::SolarSorcererView,
    Solar,
};

#[derive(Debug, Default)]
pub struct SolarBuilder<'source> {
    caste: Option<SolarCaste>,
    favored_abilities: HashSet<AbilityName>,
    sorcery: Option<SolarSorcererView<'source>>,
}

impl<'source> SolarBuilder<'source> {
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

    /// Consumes the builder, without cloning.
    pub fn build_view(self) -> Result<Solar<'source>, SolarBuilderError> {
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

        Ok(Solar::new(self.caste.unwrap(), arr, self.sorcery))
    }
}
