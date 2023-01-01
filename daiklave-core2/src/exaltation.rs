use serde::{Deserialize, Serialize};

/// Traits common to all Exalts
pub mod exalt;

/// Traits which are unique to mortals (or which function differently without
///  Essence)
pub mod mortal;
use mortal::{Mortal, MortalView};

use crate::{
    martial_arts::{
        MartialArtist, MartialArtistSwitch, MartialArtistView, MartialArtistViewSwitch,
        MartialArtsStyle, MartialArtsStyleId,
    },
    sorcery::{
        ShapingRitual, ShapingRitualId, SolarSorcerer, SolarSorcererView, SorceryArchetype,
        SorceryArchetypeId, SpellId, TerrestrialSpell,
    },
    Character, CharacterMutationError, CharacterView,
};

use self::exalt::{
    essence::{
        CommitMotesError, Essence, MoteCommitmentId, MotePoolName, RecoverMotesError,
        SetEssenceRatingError, SpendMotesError, UncommitMotesError,
    },
    exalt_type::{solar::Solar, ExaltType, ExaltTypeView},
    Exalt, ExaltView,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum ExaltState {
    Mortal(Box<Mortal>),
    Exalt(Box<Exalt>),
}

impl Default for ExaltState {
    fn default() -> Self {
        Self::Mortal(Box::new(Mortal::default()))
    }
}

impl ExaltState {
    pub fn is_mortal(&self) -> bool {
        matches!(self, Self::Mortal(_))
    }

    pub fn is_exalted(&self) -> bool {
        !self.is_mortal()
    }

    pub fn check_set_mortal(&self) -> Result<(), CharacterMutationError> {
        Ok(())
    }

    pub fn set_mortal(&mut self) -> Result<&mut Self, CharacterMutationError> {
        if self.is_mortal() {
            return Ok(self);
        }

        let exalt = if let ExaltState::Exalt(exalt) = self {
            exalt
        } else {
            unreachable!()
        };

        // Preserve Terrestrial circle sorcery
        let sorcery = {
            match exalt.exalt_type() {
                ExaltType::Solar(solar) => {
                    if let Some(sorcery) = &solar.sorcery {
                        match sorcery {
                            SolarSorcerer::Terrestrial(terrestrial) => {
                                Some((**terrestrial).clone())
                            }
                            SolarSorcerer::Celestial(celestial) => {
                                Some((**celestial).clone().into())
                            }
                            SolarSorcerer::Solar(solar) => Some((**solar).clone().into()),
                        }
                    } else {
                        None
                    }
                }
            }
        };

        // Preserve martial arts styles
        let martial_arts_styles = std::mem::take(exalt.martial_arts_styles_mut())
            .into_iter()
            .map(|(id, exalt_artist)| (id, exalt_artist.into()))
            .collect();

        *self = ExaltState::Mortal(Box::new(Mortal::new(martial_arts_styles, sorcery)));

        Ok(self)
    }

    pub fn essence(&self) -> Option<&Essence> {
        match self {
            ExaltState::Mortal(_) => None,
            ExaltState::Exalt(exalt) => Some(exalt.essence()),
        }
    }

    pub fn check_spend_motes(
        &self,
        first: MotePoolName,
        amount: u8,
    ) -> Result<(), CharacterMutationError> {
        match self {
            ExaltState::Mortal(_) => Err(CharacterMutationError::SpendMotesError(
                SpendMotesError::MortalError,
            )),
            ExaltState::Exalt(exalt) => exalt.check_spend_motes(first, amount),
        }
    }

    pub fn spend_motes(
        &mut self,
        first: MotePoolName,
        amount: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            ExaltState::Mortal(_) => Err(CharacterMutationError::SpendMotesError(
                SpendMotesError::MortalError,
            )),
            ExaltState::Exalt(exalt) => exalt.spend_motes(first, amount),
        }?;
        Ok(self)
    }

    pub fn check_commit_motes(
        &self,
        id: &MoteCommitmentId,
        name: &str,
        first: MotePoolName,
        amount: u8,
    ) -> Result<(), CharacterMutationError> {
        match self {
            ExaltState::Mortal(_) => Err(CharacterMutationError::CommitMotesError(
                CommitMotesError::MortalError,
            )),
            ExaltState::Exalt(exalt) => exalt.check_commit_motes(id, name, first, amount),
        }
    }

    pub fn commit_motes(
        &mut self,
        id: &MoteCommitmentId,
        name: &str,
        first: MotePoolName,
        amount: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            ExaltState::Mortal(_) => Err(CharacterMutationError::CommitMotesError(
                CommitMotesError::MortalError,
            )),
            ExaltState::Exalt(exalt) => exalt.commit_motes(id, name, first, amount),
        }?;
        Ok(self)
    }

    pub fn check_recover_motes(&self, _amount: u8) -> Result<(), CharacterMutationError> {
        match self {
            ExaltState::Mortal(_) => Err(CharacterMutationError::RecoverMotesError(
                RecoverMotesError::MortalError,
            )),
            ExaltState::Exalt(_) => Ok(()),
        }
    }

    pub fn recover_motes(&mut self, amount: u8) -> Result<&mut Self, CharacterMutationError> {
        match self {
            ExaltState::Mortal(_) => Err(CharacterMutationError::RecoverMotesError(
                RecoverMotesError::MortalError,
            )),
            ExaltState::Exalt(exalt_type) => exalt_type.recover_motes(amount),
        }?;
        Ok(self)
    }

    pub fn check_uncommit_motes(
        &self,
        id: &MoteCommitmentId,
    ) -> Result<(), CharacterMutationError> {
        match self {
            ExaltState::Mortal(_) => Err(CharacterMutationError::UncommitMotesError(
                UncommitMotesError::MortalError,
            )),
            ExaltState::Exalt(exalt_type) => exalt_type.check_uncommit_motes(id),
        }
    }

    pub fn uncommit_motes(
        &mut self,
        id: &MoteCommitmentId,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            ExaltState::Mortal(_) => Err(CharacterMutationError::UncommitMotesError(
                UncommitMotesError::MortalError,
            )),
            ExaltState::Exalt(exalt_type) => exalt_type.uncommit_motes(id),
        }?;
        Ok(self)
    }

    pub fn check_set_essence_rating(&self, rating: u8) -> Result<(), CharacterMutationError> {
        match self {
            ExaltState::Mortal(_) => Err(CharacterMutationError::SetEssenceRatingError(
                SetEssenceRatingError::MortalError,
            )),
            ExaltState::Exalt(_) => {
                if (1..=5).contains(&rating) {
                    Ok(())
                } else {
                    Err(CharacterMutationError::SetEssenceRatingError(
                        SetEssenceRatingError::InvalidRating(rating),
                    ))
                }
            }
        }
    }

    pub fn set_essence_rating(&mut self, rating: u8) -> Result<&mut Self, CharacterMutationError> {
        self.check_set_essence_rating(rating)?;
        match self {
            ExaltState::Exalt(exalt_type) => exalt_type.set_essence_rating(rating),
            ExaltState::Mortal(_) => Err(CharacterMutationError::SetEssenceRatingError(
                SetEssenceRatingError::MortalError,
            )),
        }?;
        Ok(self)
    }

    pub(crate) fn check_add_martial_arts_style(
        &self,
        id: MartialArtsStyleId,
        style: &MartialArtsStyle,
    ) -> Result<(), CharacterMutationError> {
        match self {
            ExaltState::Mortal(mortal) => mortal.check_add_martial_arts_style(id, style),
            ExaltState::Exalt(exalt) => exalt.check_add_martial_arts_style(id, style),
        }
    }

    pub(crate) fn add_martial_arts_style(
        &mut self,
        id: MartialArtsStyleId,
        style: &MartialArtsStyle,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            ExaltState::Mortal(mortal) => {
                mortal.add_martial_arts_style(id, style)?;
            }
            ExaltState::Exalt(exalt) => {
                exalt.add_martial_arts_style(id, style)?;
            }
        }
        Ok(self)
    }

    pub(crate) fn check_remove_martial_arts_style(
        &self,
        id: MartialArtsStyleId,
    ) -> Result<(), CharacterMutationError> {
        match self {
            ExaltState::Mortal(mortal) => mortal.check_remove_martial_arts_style(id),
            ExaltState::Exalt(exalt) => exalt.check_remove_martial_arts_style(id),
        }
    }

    pub(crate) fn remove_martial_arts_style(
        &mut self,
        id: MartialArtsStyleId,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            ExaltState::Mortal(mortal) => {
                mortal.remove_martial_arts_style(id)?;
            }
            ExaltState::Exalt(exalt) => {
                exalt.remove_martial_arts_style(id)?;
            }
        }
        Ok(self)
    }

    pub(crate) fn check_set_martial_arts_dots(
        &self,
        id: MartialArtsStyleId,
        dots: u8,
    ) -> Result<(), CharacterMutationError> {
        match self {
            ExaltState::Mortal(mortal) => mortal.check_set_martial_arts_dots(id, dots),
            ExaltState::Exalt(exalt) => exalt.check_set_martial_arts_dots(id, dots),
        }
    }

    pub(crate) fn set_martial_arts_dots(
        &mut self,
        id: MartialArtsStyleId,
        dots: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            ExaltState::Mortal(mortal) => {
                mortal.set_martial_arts_dots(id, dots)?;
            }
            ExaltState::Exalt(exalt) => {
                exalt.set_martial_arts_dots(id, dots)?;
            }
        }
        Ok(self)
    }

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
                *self = Self::Exalt(Box::new(Exalt::new(
                    Essence::new_solar(1),
                    std::mem::take(mortal.as_mut().martial_arts_styles_mut())
                        .into_iter()
                        .map(|(id, mortal_artist)| (id, mortal_artist.into()))
                        .collect(),
                    ExaltType::Solar(solar.clone()),
                )))
            }
            ExaltState::Exalt(exalt) => {
                // Preserve essence rating
                // Preserve martial arts styles (including charms)
                *self = Self::Exalt(Box::new(Exalt::new(
                    Essence::new_solar(exalt.essence().rating()),
                    std::mem::take(exalt.martial_arts_styles_mut()),
                    ExaltType::Solar(solar.clone()),
                )));
            }
        }

        Ok(self)
    }
}

impl<'char> ExaltState {
    pub(crate) fn martial_artist(
        &'char self,
        id: MartialArtsStyleId,
    ) -> Option<MartialArtist<'char>> {
        match self {
            ExaltState::Mortal(mortal) => Some(MartialArtist(MartialArtistSwitch::Mortal(
                mortal.martial_arts_styles().get(&id)?,
            ))),
            ExaltState::Exalt(exalt) => Some(MartialArtist(MartialArtistSwitch::Exalt(
                exalt.martial_arts_styles().get(&id)?,
            ))),
        }
    }

    pub(crate) fn martial_arts_id_iter(&'char self) -> impl Iterator<Item = MartialArtsStyleId> {
        match self {
            ExaltState::Mortal(mortal) => mortal
                .martial_arts_styles()
                .keys()
                .copied()
                .collect::<Vec<MartialArtsStyleId>>()
                .into_iter(),
            ExaltState::Exalt(exalt) => exalt
                .martial_arts_styles()
                .keys()
                .copied()
                .collect::<Vec<MartialArtsStyleId>>()
                .into_iter(),
        }
    }
}

impl Character {
    /// Returns true if character is not Exalted.
    pub fn is_mortal(&self) -> bool {
        self.exalt_state.is_mortal()
    }

    /// Returns true if character is an Exalt.
    pub fn is_exalted(&self) -> bool {
        self.exalt_state.is_exalted()
    }

    /// Checks if character can be de-Exalted and set to be mortal.
    pub fn check_set_mortal(&self) -> Result<(), CharacterMutationError> {
        self.exalt_state.check_set_mortal()
    }

    /// De-Exalts character, setting them to be mortal. This also reduces their
    /// permanent willpower rating by 2 (reflecting the difference between
    /// mortal default and Exalt default).
    pub fn set_mortal(&mut self) -> Result<&mut Self, CharacterMutationError> {
        if self.is_mortal() {
            return Ok(self);
        }
        self.exalt_state.set_mortal()?;
        let new_willpower_rating = self.willpower().rating().max(2) - 2;
        self.set_willpower_rating(new_willpower_rating)?;
        Ok(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ExaltStateView<'source> {
    Mortal(Box<MortalView<'source>>),
    Exalt(Box<ExaltView<'source>>),
}

impl<'source> Default for ExaltStateView<'source> {
    fn default() -> Self {
        Self::Mortal(Box::new(MortalView::default()))
    }
}

impl<'source> ExaltStateView<'source> {
    pub fn is_mortal(&self) -> bool {
        matches!(self, Self::Mortal(_))
    }

    pub fn is_exalted(&self) -> bool {
        !self.is_mortal()
    }

    pub fn check_set_mortal(&self) -> Result<(), CharacterMutationError> {
        Ok(())
    }

    pub fn set_mortal(&mut self) -> Result<&mut Self, CharacterMutationError> {
        if self.is_mortal() {
            return Ok(self);
        }

        let exalt = if let ExaltStateView::Exalt(exalt) = self {
            exalt
        } else {
            unreachable!()
        };

        // Preserve Terrestrial circle sorcery
        let sorcery = {
            match exalt.exalt_type() {
                ExaltTypeView::Solar(solar) => {
                    if let Some(sorcery) = solar.sorcery() {
                        match sorcery {
                            SolarSorcererView::Terrestrial(terrestrial) => {
                                Some(terrestrial.clone())
                            }
                            SolarSorcererView::Celestial(celestial) => Some(celestial.into()),
                            SolarSorcererView::Solar(solar) => Some(solar.into()),
                        }
                    } else {
                        None
                    }
                }
            }
        };

        // Preserve martial arts styles
        let martial_arts_styles = std::mem::take(exalt.as_mut().martial_arts_styles_mut())
            .into_iter()
            .map(|(id, exalt_artist)| (id, exalt_artist.into()))
            .collect();

        *self = ExaltStateView::Mortal(Box::new(MortalView {
            martial_arts_styles,
            sorcery,
        }));
        Ok(self)
    }

    pub(crate) fn check_add_martial_arts_style(
        &self,
        id: MartialArtsStyleId,
        style: &MartialArtsStyle,
    ) -> Result<(), CharacterMutationError> {
        match self {
            ExaltStateView::Mortal(mortal) => mortal.check_add_martial_arts_style(id, style),
            ExaltStateView::Exalt(exalt) => exalt.check_add_martial_arts_style(id, style),
        }
    }

    pub(crate) fn add_martial_arts_style(
        &mut self,
        id: MartialArtsStyleId,
        style: &'source MartialArtsStyle,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            ExaltStateView::Mortal(mortal) => {
                mortal.add_martial_arts_style(id, style)?;
            }
            ExaltStateView::Exalt(exalt) => {
                exalt.add_martial_arts_style(id, style)?;
            }
        }
        Ok(self)
    }

    pub(crate) fn check_remove_martial_arts_style(
        &self,
        id: MartialArtsStyleId,
    ) -> Result<(), CharacterMutationError> {
        match self {
            ExaltStateView::Mortal(mortal) => mortal.check_remove_martial_arts_style(id),
            ExaltStateView::Exalt(exalt) => exalt.check_remove_martial_arts_style(id),
        }
    }

    pub(crate) fn remove_martial_arts_style(
        &mut self,
        id: MartialArtsStyleId,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            ExaltStateView::Mortal(mortal) => {
                mortal.remove_martial_arts_style(id)?;
            }
            ExaltStateView::Exalt(exalt) => {
                exalt.remove_martial_arts_style(id)?;
            }
        }
        Ok(self)
    }

    pub(crate) fn check_set_martial_arts_dots(
        &self,
        id: MartialArtsStyleId,
        dots: u8,
    ) -> Result<(), CharacterMutationError> {
        match self {
            ExaltStateView::Mortal(mortal) => mortal.check_set_martial_arts_dots(id, dots),
            ExaltStateView::Exalt(exalt) => exalt.check_set_martial_arts_dots(id, dots),
        }
    }

    pub(crate) fn set_martial_arts_dots(
        &mut self,
        id: MartialArtsStyleId,
        dots: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            ExaltStateView::Mortal(mortal) => {
                mortal.set_martial_arts_dots(id, dots)?;
            }
            ExaltStateView::Exalt(exalt) => {
                exalt.set_martial_arts_dots(id, dots)?;
            }
        }
        Ok(self)
    }
}

impl<'view, 'source> ExaltStateView<'source> {
    pub(crate) fn martial_artist(
        &'view self,
        id: MartialArtsStyleId,
    ) -> Option<MartialArtistView<'view, 'source>> {
        match self {
            ExaltStateView::Mortal(mortal) => Some(MartialArtistView(
                MartialArtistViewSwitch::Mortal(mortal.martial_arts_styles.get(&id)?),
            )),
            ExaltStateView::Exalt(exalt) => Some(MartialArtistView(
                MartialArtistViewSwitch::Exalt(exalt.martial_arts_styles().get(&id)?),
            )),
        }
    }

    pub(crate) fn martial_arts_id_iter(&'view self) -> impl Iterator<Item = MartialArtsStyleId> {
        match self {
            ExaltStateView::Mortal(mortal) => mortal
                .martial_arts_styles
                .keys()
                .copied()
                .collect::<Vec<MartialArtsStyleId>>()
                .into_iter(),
            ExaltStateView::Exalt(exalt) => exalt
                .martial_arts_styles()
                .keys()
                .copied()
                .collect::<Vec<MartialArtsStyleId>>()
                .into_iter(),
        }
    }

    pub fn add_terrestrial_sorcery(
        &mut self,
        archetype_id: SorceryArchetypeId,
        archetype: &'source SorceryArchetype,
        shaping_ritual_id: ShapingRitualId,
        shaping_ritual: &'source ShapingRitual,
        control_spell_id: SpellId,
        control_spell: &'source TerrestrialSpell,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            ExaltStateView::Mortal(mortal) => {
                mortal.add_terrestrial_sorcery(
                    archetype_id,
                    archetype,
                    shaping_ritual_id,
                    shaping_ritual,
                    control_spell_id,
                    control_spell,
                )?;
            }
            ExaltStateView::Exalt(exalt) => {
                exalt.add_terrestrial_sorcery(
                    archetype_id,
                    archetype,
                    shaping_ritual_id,
                    shaping_ritual,
                    control_spell_id,
                    control_spell,
                )?;
            }
        }
        Ok(self)
    }
}

impl<'source> CharacterView<'source> {
    /// Returns true if character is not Exalted.
    pub fn is_mortal(&self) -> bool {
        self.exalt_state.is_mortal()
    }

    /// Returns true if character is an Exalt.
    pub fn is_exalted(&self) -> bool {
        self.exalt_state.is_exalted()
    }

    /// Checks if character can be de-Exalted and set to be mortal.
    pub fn check_set_mortal(&self) -> Result<(), CharacterMutationError> {
        self.exalt_state.check_set_mortal()
    }

    /// De-Exalts character, setting them to be mortal. This also reduces their
    /// permanent willpower rating by 2 (reflecting the difference between
    /// mortal default and Exalt default).
    pub fn set_mortal(&mut self) -> Result<&mut Self, CharacterMutationError> {
        if self.is_mortal() {
            return Ok(self);
        }
        self.exalt_state.set_mortal()?;
        let new_willpower_rating = self.willpower().rating().max(2) - 2;
        self.set_willpower_rating(new_willpower_rating)?;
        Ok(self)
    }
}
