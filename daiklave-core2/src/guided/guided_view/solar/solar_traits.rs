use crate::{
    exaltation::exalt::exalt_type::solar::{
        caste::{dawn::Dawn, eclipse::Eclipse, night::Night, twilight::Twilight, zenith::Zenith},
        Solar, SolarView,
    },
    guided::{error::GuidedError, guided_view::GuidedView, ExaltationChoice},
};

impl<'source> GuidedView<'source> {
    /// Returns a new owned Solar object for the previously specified Caste,
    /// Supernal, and Favored abilities.
    pub fn solar_traits(&self) -> Result<SolarView<'source>, GuidedError> {
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
}
