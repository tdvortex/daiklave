use std::collections::HashSet;

use crate::{abilities::AbilityName, sorcery::SolarSorcererView};

use super::{
    builder_error::SolarBuilderError,
    caste::{
        dawn::DawnView, eclipse::EclipseView, night::NightView, twilight::TwilightView,
        zenith::ZenithView, SolarCasteView,
    },
    solar::Solar,
    solar_view::SolarView,
};

#[derive(Debug, Default)]
pub struct SolarBuilder<'source> {
    caste: Option<SolarCasteView>,
    favored_abilities: HashSet<AbilityName>,
    sorcery: Option<SolarSorcererView<'source>>,
}

impl<'source> SolarBuilder<'source> {
    pub fn set_dawn(&mut self, dawn: DawnView) -> &mut Self {
        if !self.favored_abilities.is_empty() {
            self.favored_abilities.clear();
        }

        self.caste = Some(SolarCasteView::Dawn(dawn));
        self
    }

    pub fn set_zenith(&mut self, zenith: ZenithView) -> &mut Self {
        if !self.favored_abilities.is_empty() {
            self.favored_abilities.clear();
        }

        self.caste = Some(SolarCasteView::Zenith(zenith));
        self
    }

    pub fn set_twilight(&mut self, twilight: TwilightView) -> &mut Self {
        if !self.favored_abilities.is_empty() {
            self.favored_abilities.clear();
        }

        self.caste = Some(SolarCasteView::Twilight(twilight));
        self
    }

    pub fn set_night(&mut self, night: NightView) -> &mut Self {
        if !self.favored_abilities.is_empty() {
            self.favored_abilities.clear();
        }

        self.caste = Some(SolarCasteView::Night(night));
        self
    }

    pub fn set_eclipse(&mut self, eclipse: EclipseView) -> &mut Self {
        if !self.favored_abilities.is_empty() {
            self.favored_abilities.clear();
        }

        self.caste = Some(SolarCasteView::Eclipse(eclipse));
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

    /// Consumes the builder to finalize Solar, cloning into an owned object.
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

        Ok(Solar::new(
            self.caste.unwrap().into_owned(),
            arr,
            self.sorcery.map(|sorcery| sorcery.into()),
        ))
    }

    /// Consumes the builder, without cloning.
    pub fn build_view(self) -> Result<SolarView<'source>, SolarBuilderError> {
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

        Ok(SolarView::new(self.caste.unwrap(), arr, self.sorcery))
    }
}
