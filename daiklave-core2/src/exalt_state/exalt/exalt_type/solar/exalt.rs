use crate::{
    exalt_state::{
        exalt::{
            essence::{Essence, EssenceView},
            exalt_type::{ExaltType, ExaltTypeView},
            Exalt, ExaltView,
        },
        ExaltState, ExaltStateView,
    },
    CharacterMutationError,
};

use super::{Solar, SolarView};

impl Exalt {
    pub fn is_solar(&self) -> bool {
        self.exalt_type.is_solar()
    }

    pub fn solar_traits(&self) -> Option<&Solar> {
        self.exalt_type.solar_traits()
    }
}

impl<'source> ExaltView<'source> {
    pub fn is_solar(&self) -> bool {
        self.exalt_type.is_solar()
    }

    pub fn solar_traits(&self) -> Option<&SolarView> {
        self.exalt_type.solar_traits()
    }
}

impl ExaltType {
    pub fn is_solar(&self) -> bool {
        true
    }

    pub fn solar_traits(&self) -> Option<&Solar> {
        match self {
            ExaltType::Solar(solar_traits) => Some(solar_traits),
        }
    }
}

impl<'source> ExaltTypeView<'source> {
    pub fn is_solar(&self) -> bool {
        true
    }

    pub fn solar_traits(&self) -> Option<&SolarView> {
        match self {
            ExaltTypeView::Solar(solar_traits) => Some(solar_traits),
        }
    }
}

impl ExaltState {
    pub fn is_solar(&self) -> bool {
        if let Self::Exalt(exalt) = self {
            exalt.is_solar()
        } else {
            false
        }
    }

    pub fn solar_traits(&self) -> Option<&Solar> {
        if let Self::Exalt(exalt) = self {
            exalt.solar_traits()
        } else {
            None
        }
    }

    pub fn check_set_solar(&self, _solar: &Solar) -> Result<(), CharacterMutationError> {
        Ok(())
    }

    pub fn set_solar(&mut self, solar: &Solar) -> Result<&mut Self, CharacterMutationError> {
        if self.is_solar() {
            return Ok(self);
        }

        match self {
            ExaltState::Mortal(mortal) => {
                // Default to essence 1
                // Preserve martial arts styles, with empty Charms set
                *self = Self::Exalt(Exalt {
                    essence: Essence::new_solar(1),
                    martial_arts_styles: std::mem::take(&mut mortal.martial_arts_styles)
                        .into_iter()
                        .map(|(id, mortal_artist)| (id, mortal_artist.into()))
                        .collect(),
                    exalt_type: ExaltType::Solar(solar.clone()),
                })
            }
            ExaltState::Exalt(exalt) => {
                // Preserve essence rating
                // Preserve martial arts styles (including charms)
                *self = Self::Exalt(Exalt {
                    essence: Essence::new_solar(exalt.essence().rating()),
                    martial_arts_styles: std::mem::take(&mut exalt.martial_arts_styles),
                    exalt_type: ExaltType::Solar(solar.clone()),
                });
            }
        }

        Ok(self)
    }
}

impl<'source> ExaltStateView<'source> {
    pub fn is_solar(&self) -> bool {
        if let Self::Exalt(exalt_type) = self {
            exalt_type.is_solar()
        } else {
            false
        }
    }

    pub fn solar_traits(&self) -> Option<&SolarView> {
        if let Self::Exalt(exalt_type) = self {
            exalt_type.solar_traits()
        } else {
            None
        }
    }

    pub fn check_set_solar(&self, _solar: &'source Solar) -> Result<(), CharacterMutationError> {
        Ok(())
    }

    pub fn check_set_solar_view(&self, _solar: &SolarView) -> Result<(), CharacterMutationError> {
        Ok(())
    }

    pub fn set_solar(
        &mut self,
        solar: &'source Solar,
    ) -> Result<&mut Self, CharacterMutationError> {
        if self.is_solar() {
            return Ok(self);
        }

        self.set_solar_view(solar.as_view())
    }

    pub fn set_solar_view(
        &mut self,
        solar: SolarView<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        if self.is_solar() {
            return Ok(self);
        }

        match self {
            ExaltStateView::Mortal(mortal) => {
                // Default to essence 1
                // Preserve martial arts styles, with empty Charms set
                *self = Self::Exalt(ExaltView {
                    essence: EssenceView::new_solar(1),
                    martial_arts_styles: std::mem::take(&mut mortal.martial_arts_styles)
                        .into_iter()
                        .map(|(id, mortal_artist)| (id, mortal_artist.into()))
                        .collect(),
                    exalt_type: ExaltTypeView::Solar(solar),
                })
            }
            ExaltStateView::Exalt(exalt) => {
                // Preserve essence rating
                // Preserve martial arts styles (including charms)
                *self = Self::Exalt(ExaltView {
                    essence: EssenceView::new_solar(exalt.essence().rating()),
                    martial_arts_styles: std::mem::take(&mut exalt.martial_arts_styles),
                    exalt_type: ExaltTypeView::Solar(solar),
                });
            }
        }

        Ok(self)
    }
}
