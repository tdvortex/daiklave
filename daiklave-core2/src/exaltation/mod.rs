/// Traits common to all Exalts
pub mod exalt;

/// Traits which are unique to mortals (or which function differently without
///  Essence)
pub mod mortal;

mod martial_artist;
mod memo;
mod sorcery;

pub(crate) use martial_artist::ExaltationMartialArtist;
pub(crate) use memo::ExaltationMemo;
pub(crate) use sorcery::ExaltationSorcery;

use crate::{
    armor::armor_item::{
        artifact::{ArtifactArmorView, ArtifactArmorId},
        mundane::MundaneArmor,
        ArmorId, ArmorItem, BaseArmorId,
    },
    martial_arts::{MartialArtist, MartialArtsStyle, MartialArtsStyleId},
    sorcery::{
        ShapingRitual, ShapingRitualId, Sorcery, SorceryArchetype, SorceryArchetypeId, SpellId,
        TerrestrialSpell,
    },
    weapons::weapon::{
        artifact::ArtifactWeaponView, mundane::MundaneWeaponMemo, ArtifactWeaponId, BaseWeaponId,
        EquipHand, Equipped, Weapon, WeaponId,
    },
    CharacterMutationError,
};

use self::{
    exalt::{
        essence::{
            CommitMotesError, Essence, MoteCommitmentId, MotePoolName, RecoverMotesError,
            SetEssenceRatingError, SpendMotesError, UncommitMotesError,
        },
        exalt_type::{
            solar::{Solar, SolarMemo, SolarSorcererView},
            ExaltType,
        },
        Exalt,
    },
    mortal::Mortal,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Exaltation<'source> {
    Mortal(Box<Mortal<'source>>),
    Exalt(Box<Exalt<'source>>),
}

impl<'source> Default for Exaltation<'source> {
    fn default() -> Self {
        Self::Mortal(Box::new(Mortal::default()))
    }
}

impl<'source> Exaltation<'source> {
    pub fn as_memo(&self) -> ExaltationMemo {
        match self {
            Exaltation::Mortal(box_view) => {
                ExaltationMemo::Mortal(Box::new(box_view.as_ref().as_memo()))
            }
            Exaltation::Exalt(box_view) => {
                ExaltationMemo::Exalt(Box::new(box_view.as_ref().as_memo()))
            }
        }
    }

    pub fn is_mortal(&self) -> bool {
        matches!(self, Self::Mortal(_))
    }

    pub fn is_exalted(&self) -> bool {
        !self.is_mortal()
    }

    pub fn get_weapon(
        &self,
        weapon_id: WeaponId,
        equipped: Option<Equipped>,
    ) -> Option<Weapon<'source>> {
        if matches!(weapon_id, WeaponId::Unarmed) {
            if matches!(equipped, Some(Equipped::Natural)) {
                Some(crate::weapons::weapon::mundane::unarmed())
            } else {
                None
            }
        } else {
            match self {
                Exaltation::Mortal(box_mortal) => {
                    box_mortal.as_ref().get_weapon(weapon_id, equipped)
                }
                Exaltation::Exalt(box_exalt) => box_exalt.as_ref().get_weapon(weapon_id, equipped),
            }
        }
    }

    pub fn iter_weapons(&self) -> impl Iterator<Item = (WeaponId, Option<Equipped>)> {
        match self {
            Exaltation::Mortal(box_mortal) => box_mortal
                .as_ref()
                .iter_weapons()
                .collect::<Vec<(WeaponId, Option<Equipped>)>>(),
            Exaltation::Exalt(box_exalt) => box_exalt
                .as_ref()
                .iter_weapons()
                .collect::<Vec<(WeaponId, Option<Equipped>)>>(),
        }
        .into_iter()
    }

    pub fn check_set_mortal(&self) -> Result<(), CharacterMutationError> {
        Ok(())
    }

    pub fn set_mortal(&mut self) -> Result<&mut Self, CharacterMutationError> {
        if self.is_mortal() {
            return Ok(self);
        }

        let exalt = if let Exaltation::Exalt(exalt) = self {
            exalt
        } else {
            unreachable!()
        };

        // Preserve Terrestrial circle sorcery
        let sorcery = {
            match exalt.exalt_type() {
                ExaltType::Solar(solar) => {
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

        // Remove all artifact attunements
        let weapons = std::mem::take(exalt.weapons_mut()).into();
        let armor = std::mem::take(exalt.armor_mut()).into();

        *self = Exaltation::Mortal(Box::new(Mortal {
            martial_arts_styles,
            sorcery,
            weapons,
            armor,
        }));
        Ok(self)
    }

    pub(crate) fn check_add_martial_arts_style(
        &self,
        id: MartialArtsStyleId,
        style: &MartialArtsStyle,
    ) -> Result<(), CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => mortal.check_add_martial_arts_style(id, style),
            Exaltation::Exalt(exalt) => exalt.check_add_martial_arts_style(id, style),
        }
    }

    pub(crate) fn add_martial_arts_style(
        &mut self,
        id: MartialArtsStyleId,
        style: &'source MartialArtsStyle,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.add_martial_arts_style(id, style)?;
            }
            Exaltation::Exalt(exalt) => {
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
            Exaltation::Mortal(mortal) => mortal.check_remove_martial_arts_style(id),
            Exaltation::Exalt(exalt) => exalt.check_remove_martial_arts_style(id),
        }
    }

    pub(crate) fn remove_martial_arts_style(
        &mut self,
        id: MartialArtsStyleId,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.remove_martial_arts_style(id)?;
            }
            Exaltation::Exalt(exalt) => {
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
            Exaltation::Mortal(mortal) => mortal.check_set_martial_arts_dots(id, dots),
            Exaltation::Exalt(exalt) => exalt.check_set_martial_arts_dots(id, dots),
        }
    }

    pub(crate) fn set_martial_arts_dots(
        &mut self,
        id: MartialArtsStyleId,
        dots: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.set_martial_arts_dots(id, dots)?;
            }
            Exaltation::Exalt(exalt) => {
                exalt.set_martial_arts_dots(id, dots)?;
            }
        }
        Ok(self)
    }
}

impl<'view, 'source> Exaltation<'source> {
    pub(crate) fn martial_artist(
        &'view self,
        id: MartialArtsStyleId,
    ) -> Option<MartialArtist<'view, 'source>> {
        match self {
            Exaltation::Mortal(mortal) => Some(MartialArtist::new(
                id,
                ExaltationMartialArtist::Mortal(mortal.martial_arts_styles.get(&id)?),
            )),
            Exaltation::Exalt(exalt) => Some(MartialArtist::new(
                id,
                ExaltationMartialArtist::Exalt(exalt.martial_arts_styles().get(&id)?),
            )),
        }
    }

    pub(crate) fn martial_arts_id_iter(&'view self) -> impl Iterator<Item = MartialArtsStyleId> {
        let mut ids = match self {
            Exaltation::Mortal(mortal) => mortal
                .martial_arts_styles
                .keys()
                .copied()
                .collect::<Vec<MartialArtsStyleId>>(),
            Exaltation::Exalt(exalt) => exalt
                .martial_arts_styles()
                .keys()
                .copied()
                .collect::<Vec<MartialArtsStyleId>>(),
        };

        ids.sort_by(|a, b| {
            self.martial_artist(*a)
                .unwrap()
                .name()
                .cmp(self.martial_artist(*b).unwrap().name())
        });
        ids.into_iter()
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
            Exaltation::Mortal(mortal) => {
                mortal.add_terrestrial_sorcery(
                    archetype_id,
                    archetype,
                    shaping_ritual_id,
                    shaping_ritual,
                    control_spell_id,
                    control_spell,
                )?;
            }
            Exaltation::Exalt(exalt) => {
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

    pub(crate) fn sorcery(&'view self) -> Option<Sorcery<'view, 'source>> {
        match self {
            Exaltation::Mortal(mortal) => mortal
                .sorcery
                .as_ref()
                .map(|terrestrial| Sorcery(ExaltationSorcery::Mortal(terrestrial))),
            Exaltation::Exalt(exalt) => exalt.sorcery(),
        }
    }

    pub fn is_solar(&self) -> bool {
        if let Self::Exalt(exalt_type) = self {
            exalt_type.is_solar()
        } else {
            false
        }
    }

    pub fn solar_traits(&self) -> Option<&Solar> {
        if let Self::Exalt(exalt_type) = self {
            exalt_type.solar_traits()
        } else {
            None
        }
    }

    pub fn check_set_solar(
        &self,
        _solar: &'source SolarMemo,
    ) -> Result<(), CharacterMutationError> {
        Ok(())
    }

    pub fn check_set_solar_view(&self, _solar: &Solar) -> Result<(), CharacterMutationError> {
        Ok(())
    }

    pub fn set_solar(
        &mut self,
        solar: &'source SolarMemo,
    ) -> Result<&mut Self, CharacterMutationError> {
        if self.is_solar() {
            return Ok(self);
        }

        self.set_solar_view(solar.as_ref())
    }

    pub fn set_solar_view(
        &mut self,
        solar: Solar<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        if self.is_solar() {
            return Ok(self);
        }

        match self {
            Exaltation::Mortal(mortal) => {
                // Default to essence 1
                // Preserve martial arts styles, with empty Charms set
                *self = Self::Exalt(Box::new(Exalt::new(
                    std::mem::take(&mut mortal.armor).into(),
                    Essence::new_solar(1),
                    std::mem::take(&mut mortal.martial_arts_styles)
                        .into_iter()
                        .map(|(id, mortal_artist)| (id, mortal_artist.into()))
                        .collect(),
                    ExaltType::Solar(solar),
                    std::mem::take(&mut mortal.weapons).into(),
                )))
            }
            Exaltation::Exalt(exalt) => {
                // Preserve essence rating
                // Preserve martial arts styles (including charms)
                *self = Self::Exalt(Box::new(Exalt::new(
                    std::mem::take(exalt.armor_mut()),
                    Essence::new_solar(exalt.essence().rating()),
                    std::mem::take(exalt.martial_arts_styles_mut()),
                    ExaltType::Solar(solar),
                    std::mem::take(exalt.weapons_mut()),
                )));
            }
        }

        Ok(self)
    }

    pub fn essence(&self) -> Option<&Essence> {
        match self {
            Exaltation::Mortal(_) => None,
            Exaltation::Exalt(exalt) => Some(exalt.essence()),
        }
    }

    pub fn check_spend_motes(
        &self,
        first: MotePoolName,
        amount: u8,
    ) -> Result<(), CharacterMutationError> {
        match self {
            Exaltation::Mortal(_) => Err(CharacterMutationError::SpendMotesError(
                SpendMotesError::MortalError,
            )),
            Exaltation::Exalt(exalt) => exalt.check_spend_motes(first, amount),
        }
    }

    pub fn spend_motes(
        &mut self,
        first: MotePoolName,
        amount: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(_) => Err(CharacterMutationError::SpendMotesError(
                SpendMotesError::MortalError,
            )),
            Exaltation::Exalt(exalt) => exalt.spend_motes(first, amount),
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
            Exaltation::Mortal(_) => Err(CharacterMutationError::CommitMotesError(
                CommitMotesError::MortalError,
            )),
            Exaltation::Exalt(exalt) => exalt.check_commit_motes(id, name, first, amount),
        }
    }

    pub fn commit_motes(
        &mut self,
        id: &MoteCommitmentId,
        name: &'source str,
        first: MotePoolName,
        amount: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(_) => Err(CharacterMutationError::CommitMotesError(
                CommitMotesError::MortalError,
            )),
            Exaltation::Exalt(exalt) => exalt.commit_motes(id, name, first, amount),
        }?;
        Ok(self)
    }

    pub fn check_recover_motes(&self, _amount: u8) -> Result<(), CharacterMutationError> {
        match self {
            Exaltation::Mortal(_) => Err(CharacterMutationError::RecoverMotesError(
                RecoverMotesError::MortalError,
            )),
            Exaltation::Exalt(_) => Ok(()),
        }
    }

    pub fn recover_motes(&mut self, amount: u8) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(_) => Err(CharacterMutationError::RecoverMotesError(
                RecoverMotesError::MortalError,
            )),
            Exaltation::Exalt(exalt) => exalt.recover_motes(amount),
        }?;
        Ok(self)
    }

    pub fn check_uncommit_motes(
        &self,
        id: &MoteCommitmentId,
    ) -> Result<(), CharacterMutationError> {
        match self {
            Exaltation::Mortal(_) => Err(CharacterMutationError::UncommitMotesError(
                UncommitMotesError::MortalError,
            )),
            Exaltation::Exalt(exalt) => exalt.check_uncommit_motes(id),
        }
    }

    pub fn uncommit_motes(
        &mut self,
        id: &MoteCommitmentId,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(_) => Err(CharacterMutationError::UncommitMotesError(
                UncommitMotesError::MortalError,
            )),
            Exaltation::Exalt(exalt) => exalt.uncommit_motes(id),
        }?;
        Ok(self)
    }

    pub fn check_set_essence_rating(&self, rating: u8) -> Result<(), CharacterMutationError> {
        match self {
            Exaltation::Mortal(_) => Err(CharacterMutationError::SetEssenceRatingError(
                SetEssenceRatingError::MortalError,
            )),
            Exaltation::Exalt(_) => {
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
            Exaltation::Exalt(exalt) => exalt.set_essence_rating(rating),
            Exaltation::Mortal(_) => Err(CharacterMutationError::SetEssenceRatingError(
                SetEssenceRatingError::MortalError,
            )),
        }?;
        Ok(self)
    }

    pub fn add_mundane_weapon(
        &mut self,
        weapon_id: BaseWeaponId,
        weapon: &'source MundaneWeaponMemo,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.as_mut().add_mundane_weapon(weapon_id, weapon)?;
            }
            Exaltation::Exalt(exalt) => {
                exalt.as_mut().add_mundane_weapon(weapon_id, weapon)?;
            }
        }
        Ok(self)
    }

    pub fn equip_weapon(
        &mut self,
        weapon_id: WeaponId,
        hand: Option<EquipHand>,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.as_mut().equip_weapon(weapon_id, hand)?;
            }
            Exaltation::Exalt(exalt) => {
                exalt.as_mut().equip_weapon(weapon_id, hand)?;
            }
        }
        Ok(self)
    }

    pub fn unequip_weapon(
        &mut self,
        weapon_id: WeaponId,
        equipped: Equipped,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.as_mut().unequip_weapon(weapon_id, equipped)?;
            }
            Exaltation::Exalt(exalt) => {
                exalt.as_mut().unequip_weapon(weapon_id, equipped)?;
            }
        }
        Ok(self)
    }

    pub fn add_artifact_weapon(
        &mut self,
        artifact_weapon_id: ArtifactWeaponId,
        weapon: ArtifactWeaponView<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.add_artifact_weapon(artifact_weapon_id, weapon)?;
            }
            Exaltation::Exalt(exalt) => {
                exalt.add_artifact_weapon(artifact_weapon_id, weapon)?;
            }
        }
        Ok(self)
    }

    pub fn remove_artifact_weapon(
        &mut self,
        artifact_weapon_id: ArtifactWeaponId,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.remove_artifact_weapon(artifact_weapon_id)?;
            }
            Exaltation::Exalt(exalt) => {
                exalt.remove_artifact_weapon(artifact_weapon_id)?;
            }
        }
        Ok(self)
    }

    pub fn remove_mundane_weapon(
        &mut self,
        weapon_id: BaseWeaponId,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.remove_mundane_weapon(weapon_id)?;
            }
            Exaltation::Exalt(exalt) => {
                exalt.remove_mundane_weapon(weapon_id)?;
            }
        }
        Ok(self)
    }

    pub fn worn_armor(&self) -> Option<ArmorItem<'source>> {
        match self {
            Exaltation::Mortal(mortal) => mortal.worn_armor(),
            Exaltation::Exalt(exalt) => exalt.worn_armor(),
        }
    }

    pub fn armor_iter(&self) -> impl Iterator<Item = ArmorId> + '_ {
        match self {
            Exaltation::Mortal(mortal) => mortal.armor_iter(),
            Exaltation::Exalt(exalt) => exalt.armor_iter(),
        }
    }

    pub fn get_armor(&self, armor_id: ArmorId) -> Option<ArmorItem<'source>> {
        match self {
            Exaltation::Mortal(mortal) => mortal.get_armor(armor_id),
            Exaltation::Exalt(exalt) => exalt.get_armor(armor_id),
        }
    }

    pub fn add_mundane_armor(
        &mut self,
        armor_id: BaseArmorId,
        armor: &'source MundaneArmor,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.add_mundane_armor(armor_id, armor)?;
            }
            Exaltation::Exalt(exalt) => {
                exalt.add_mundane_armor(armor_id, armor)?;
            }
        }
        Ok(self)
    }

    pub fn remove_mundane_armor(
        &mut self,
        armor_id: BaseArmorId,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.remove_mundane_armor(armor_id)?;
            }
            Exaltation::Exalt(exalt) => {
                exalt.remove_mundane_armor(armor_id)?;
            }
        }
        Ok(self)
    }

    pub fn equip_armor(&mut self, armor_id: ArmorId) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.equip_armor(armor_id)?;
            }
            Exaltation::Exalt(exalt) => {
                exalt.equip_armor(armor_id)?;
            }
        }
        Ok(self)
    }

    pub fn unequip_armor(&mut self) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.unequip_armor()?;
            }
            Exaltation::Exalt(exalt) => {
                exalt.unequip_armor()?;
            }
        }
        Ok(self)
    }

    pub fn add_artifact_armor(
        &mut self,
        armor_id: ArtifactArmorId,
        armor: ArtifactArmorView<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.add_artifact_armor(armor_id, armor)?;
            }
            Exaltation::Exalt(exalt) => {
                exalt.add_artifact_armor(armor_id, armor)?;
            }
        }
        Ok(self)
    }

    pub fn remove_artifact_armor(
        &mut self,
        armor_id: ArtifactArmorId,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.remove_artifact_armor(armor_id)?;
            }
            Exaltation::Exalt(exalt) => {
                exalt.remove_artifact_armor(armor_id)?;
            }
        }
        Ok(self)
    }
}
