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
        artifact::{ArtifactArmorId, ArtifactArmorView},
        mundane::MundaneArmor,
        ArmorId, ArmorItem, BaseArmorId,
    },
    artifact::{
        wonders::{OwnedWonder, Wonder, WonderId},
        ArtifactId,
    },
    charms::{
        charm::{
            evocation::{Evocation, EvocationId},
            Charm, SpiritCharmId,
        },
        CharmError,
    },
    hearthstones::{HearthstoneId, UnslottedHearthstone},
    martial_arts::{
        charm::{MartialArtsCharm, MartialArtsCharmId},
        MartialArtist, MartialArtsStyle, MartialArtsStyleId,
    },
    sorcery::{
        circles::{
            celestial::AddCelestialSorcery, solar::AddSolarSorcery,
            terrestrial::AddTerrestrialSorceryView,
        },
        spell::{SpellId, SpellMutation},
        Sorcery, SorceryArchetypeId, SorceryArchetypeMerit, SorceryArchetypeMeritId, SorceryError,
    },
    weapons::weapon::{
        artifact::ArtifactWeaponView, mundane::MundaneWeapon, ArtifactWeaponId, BaseWeaponId,
        EquipHand, Equipped, Weapon, WeaponId,
    },
    CharacterMutationError,
};

use self::{
    exalt::{
        essence::{
            Essence, EssenceError, EssenceState, MoteCommitmentId, MotePool, MotePoolName,
            MotesState, OtherMoteCommitmentId,
        },
        exalt_type::{
            solar::{
                charm::{SolarCharm, SolarCharmId},
                Solar, SolarMemo, SolarSorcererView,
            },
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
        let wonders = std::mem::take(exalt.wonders_mut()).into();

        // Assume no Exalted Healing

        *self = Exaltation::Mortal(Box::new(Mortal {
            martial_arts_styles,
            sorcery,
            weapons,
            armor,
            wonders,
            exalted_healing: false,
        }));
        Ok(self)
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
        add_terrestrial: AddTerrestrialSorceryView<'source>,
        occult_dots: u8,
        intelligence_dots: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                if occult_dots < 3 {
                    return Err(CharacterMutationError::SorceryError(
                        SorceryError::PrerequisitesNotMet,
                    ));
                }

                mortal.add_terrestrial_sorcery(add_terrestrial)?;
            }
            Exaltation::Exalt(exalt) => {
                exalt.add_terrestrial_sorcery(add_terrestrial, occult_dots, intelligence_dots)?;
            }
        }
        Ok(self)
    }

    pub fn remove_terrestrial_sorcery(&mut self) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.remove_terrestrial_sorcery()?;
            }
            Exaltation::Exalt(exalt) => {
                exalt.remove_terrestrial_sorcery()?;
            }
        }
        Ok(self)
    }

    pub fn add_celestial_sorcery(
        &mut self,
        add_sorcery: &'source AddCelestialSorcery,
        occult_dots: u8,
        intelligence_dots: u8,
        essence_rating: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(_) => {
                return Err(CharacterMutationError::SorceryError(
                    SorceryError::WrongExaltType,
                ));
            }
            Exaltation::Exalt(exalt) => {
                if essence_rating < 3 {
                    return Err(CharacterMutationError::SorceryError(
                        SorceryError::PrerequisitesNotMet,
                    ));
                }

                exalt.add_celestial_sorcery(add_sorcery, occult_dots, intelligence_dots)?;
            }
        }
        Ok(self)
    }

    pub(crate) fn remove_celestial_sorcery(&mut self) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(_) => {
                return Err(CharacterMutationError::SorceryError(
                    SorceryError::WrongExaltType,
                ));
            }
            Exaltation::Exalt(exalt) => {
                exalt.remove_celestial_sorcery()?;
            }
        }
        Ok(self)
    }

    pub fn add_solar_sorcery(
        &mut self,
        add_solar: &'source AddSolarSorcery,
        occult_dots: u8,
        essence_rating: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(_) => {
                return Err(CharacterMutationError::SorceryError(
                    SorceryError::WrongExaltType,
                ));
            }
            Exaltation::Exalt(exalt) => {
                exalt.add_solar_sorcery(add_solar, occult_dots, essence_rating)?;
            }
        }
        Ok(self)
    }

    pub fn remove_solar_sorcery(&mut self) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(_) => {
                return Err(CharacterMutationError::SorceryError(
                    SorceryError::WrongExaltType,
                ));
            }
            Exaltation::Exalt(exalt) => {
                exalt.remove_solar_sorcery()?;
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

    pub fn set_solar(
        &mut self,
        solar: &'source SolarMemo,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.set_solar_view(solar.as_ref())
    }

    pub fn set_solar_view(
        &mut self,
        mut solar: Solar<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                // Default to essence 1
                // Preserve martial arts styles, with empty Charms set
                // Preserve terrestrial circle sorcery if it exists
                if let Some(terrestrial) = mortal.sorcery.take() {
                    if solar.sorcery.is_none() {
                        solar.sorcery = Some(SolarSorcererView::Terrestrial(terrestrial));
                    }
                }

                *self = Self::Exalt(Box::new(Exalt {
                    armor: std::mem::take(&mut mortal.armor).into(),
                    essence: EssenceState {
                        rating: 1,
                        motes: MotesState {
                            peripheral: MotePool::new(33, 0),
                            personal: MotePool::new(13, 0),
                            commitments: Default::default(),
                        },
                    },
                    evocations: Vec::new(),
                    martial_arts_styles: std::mem::take(&mut mortal.martial_arts_styles)
                        .into_iter()
                        .map(|(id, mortal_artist)| (id, mortal_artist.into()))
                        .collect(),
                    exalt_type: ExaltType::Solar(solar),
                    weapons: std::mem::take(&mut mortal.weapons).into(),
                    wonders: std::mem::take(&mut mortal.wonders).into(),
                }))
            }
            Exaltation::Exalt(exalt) => {
                // Preserve essence rating, but uncommit all motes and unattune all artifacts
                let to_uncommit = exalt
                    .essence()
                    .motes()
                    .committed()
                    .map(|(id, _)| id)
                    .collect::<Vec<MoteCommitmentId>>();
                for commit_id in to_uncommit.into_iter() {
                    exalt.uncommit_motes(&commit_id)?;
                }

                // If switching solar->solar, try to preserve solar charms
                let ExaltType::Solar(old_solar) = &mut exalt.exalt_type;
                solar.solar_charms = std::mem::take(&mut old_solar.solar_charms);

                // Try to preserve martial arts styles (including charms)

                // Preserve sorcery
                if let Some(solar_sorcerer) = old_solar.sorcery.take() {
                    solar.sorcery = Some(solar_sorcerer);
                }

                *self = Self::Exalt(Box::new(Exalt {
                    armor: std::mem::take(&mut exalt.armor),
                    essence: EssenceState {
                        rating: exalt.essence().rating(),
                        motes: MotesState {
                            peripheral: MotePool::new(26 * exalt.essence().rating() * 7, 0),
                            personal: MotePool::new(10 + exalt.essence().rating() * 3, 0),
                            commitments: Default::default(),
                        },
                    },
                    // Preserve Evocations
                    evocations: std::mem::take(&mut exalt.evocations),
                    martial_arts_styles: std::mem::take(&mut exalt.martial_arts_styles),
                    exalt_type: ExaltType::Solar(solar),
                    weapons: std::mem::take(&mut exalt.weapons),
                    wonders: std::mem::take(&mut exalt.wonders),
                }));
            }
        }
        Ok(self)
    }

    pub fn essence(&'view self) -> Option<Essence<'view, 'source>> {
        match self {
            Exaltation::Mortal(_) => None,
            Exaltation::Exalt(exalt) => Some(Essence(exalt.as_ref())),
        }
    }

    pub fn spend_motes(
        &mut self,
        first: MotePoolName,
        amount: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(_) => {
                Err(CharacterMutationError::EssenceError(EssenceError::Mortal))
            }
            Exaltation::Exalt(exalt) => exalt.spend_motes(first, amount),
        }?;
        Ok(self)
    }

    pub fn commit_motes(
        &mut self,
        id: &OtherMoteCommitmentId,
        name: &'source str,
        first: MotePoolName,
        amount: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(_) => {
                Err(CharacterMutationError::EssenceError(EssenceError::Mortal))
            }
            Exaltation::Exalt(exalt) => exalt.commit_motes(id, name, first, amount),
        }?;
        Ok(self)
    }

    pub fn recover_motes(&mut self, amount: u8) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(_) => {
                Err(CharacterMutationError::EssenceError(EssenceError::Mortal))
            }
            Exaltation::Exalt(exalt) => exalt.recover_motes(amount),
        }?;
        Ok(self)
    }

    pub fn uncommit_motes(
        &mut self,
        id: &MoteCommitmentId,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(_) => {
                Err(CharacterMutationError::EssenceError(EssenceError::Mortal))
            }
            Exaltation::Exalt(exalt) => exalt.uncommit_motes(id),
        }?;
        Ok(self)
    }

    pub fn set_essence_rating(&mut self, rating: u8) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Exalt(exalt) => {
                if !(1..=5).contains(&rating) {
                    return Err(CharacterMutationError::EssenceError(
                        EssenceError::InvalidRating,
                    ));
                }

                let old_rating = exalt.essence().rating();
                exalt.set_essence_rating(rating)?;
                if old_rating > rating {
                    if rating < 5 {
                        exalt.remove_solar_sorcery().ok();
                    }

                    if rating < 3 {
                        exalt.remove_celestial_sorcery().ok();
                    }
                }
                Ok(self)
            }
            Exaltation::Mortal(_) => {
                Err(CharacterMutationError::EssenceError(EssenceError::Mortal))
            }
        }
    }

    pub fn add_mundane_weapon(
        &mut self,
        weapon_id: BaseWeaponId,
        weapon: &'source MundaneWeapon,
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

    pub fn wonders_iter(&self) -> impl Iterator<Item = WonderId> + '_ {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.wonders_iter().collect::<Vec<WonderId>>().into_iter()
            }
            Exaltation::Exalt(exalt) => exalt.wonders_iter().collect::<Vec<WonderId>>().into_iter(),
        }
    }

    pub fn get_wonder(&self, wonder_id: WonderId) -> Option<OwnedWonder<'source>> {
        match self {
            Exaltation::Mortal(mortal) => mortal.get_wonder(wonder_id),
            Exaltation::Exalt(exalt) => exalt.get_wonder(wonder_id),
        }
    }

    pub fn add_wonder(
        &mut self,
        wonder_id: WonderId,
        wonder: &'source Wonder,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.add_wonder(wonder_id, wonder)?;
            }
            Exaltation::Exalt(exalt) => {
                exalt.add_wonder(wonder_id, wonder)?;
            }
        }
        Ok(self)
    }

    pub fn remove_wonder(
        &mut self,
        wonder_id: WonderId,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.remove_wonder(wonder_id)?;
            }
            Exaltation::Exalt(exalt) => {
                exalt.remove_wonder(wonder_id)?;
            }
        }
        Ok(self)
    }

    pub fn slot_hearthstone_into_weapon(
        &mut self,
        artifact_weapon_id: ArtifactWeaponId,
        hearthstone_id: HearthstoneId,
        unslotted: UnslottedHearthstone<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.slot_hearthstone_into_weapon(
                    artifact_weapon_id,
                    hearthstone_id,
                    unslotted,
                )?;
            }
            Exaltation::Exalt(exalt) => {
                exalt.slot_hearthstone_into_weapon(
                    artifact_weapon_id,
                    hearthstone_id,
                    unslotted,
                )?;
            }
        }
        Ok(self)
    }

    pub fn slot_hearthstone_into_armor(
        &mut self,
        artifact_armor_id: ArtifactArmorId,
        hearthstone_id: HearthstoneId,
        unslotted: UnslottedHearthstone<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.slot_hearthstone_into_armor(artifact_armor_id, hearthstone_id, unslotted)?;
            }
            Exaltation::Exalt(exalt) => {
                exalt.slot_hearthstone_into_armor(artifact_armor_id, hearthstone_id, unslotted)?;
            }
        }
        Ok(self)
    }

    pub fn slot_hearthstone_into_wonder(
        &mut self,
        wonder_id: WonderId,
        hearthstone_id: HearthstoneId,
        unslotted: UnslottedHearthstone<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.slot_hearthstone_into_wonder(wonder_id, hearthstone_id, unslotted)?;
            }
            Exaltation::Exalt(exalt) => {
                exalt.slot_hearthstone_into_wonder(wonder_id, hearthstone_id, unslotted)?;
            }
        }
        Ok(self)
    }

    pub fn unslot_hearthstone_from_weapon(
        &mut self,
        artifact_weapon_id: ArtifactWeaponId,
        hearthstone_id: HearthstoneId,
    ) -> Result<UnslottedHearthstone<'source>, CharacterMutationError> {
        Ok(match self {
            Exaltation::Mortal(mortal) => {
                mortal.unslot_hearthstone_from_weapon(artifact_weapon_id, hearthstone_id)?
            }
            Exaltation::Exalt(exalt) => {
                exalt.unslot_hearthstone_from_weapon(artifact_weapon_id, hearthstone_id)?
            }
        })
    }

    pub fn unslot_hearthstone_from_armor(
        &mut self,
        artifact_armor_id: ArtifactArmorId,
        hearthstone_id: HearthstoneId,
    ) -> Result<UnslottedHearthstone<'source>, CharacterMutationError> {
        Ok(match self {
            Exaltation::Mortal(mortal) => {
                mortal.unslot_hearthstone_from_armor(artifact_armor_id, hearthstone_id)?
            }
            Exaltation::Exalt(exalt) => {
                exalt.unslot_hearthstone_from_armor(artifact_armor_id, hearthstone_id)?
            }
        })
    }

    pub fn unslot_hearthstone_from_wonder(
        &mut self,
        wonder_id: WonderId,
        hearthstone_id: HearthstoneId,
    ) -> Result<UnslottedHearthstone<'source>, CharacterMutationError> {
        Ok(match self {
            Exaltation::Mortal(mortal) => {
                mortal.unslot_hearthstone_from_wonder(wonder_id, hearthstone_id)?
            }
            Exaltation::Exalt(exalt) => {
                exalt.unslot_hearthstone_from_wonder(wonder_id, hearthstone_id)?
            }
        })
    }

    pub fn attune_artifact(
        &mut self,
        artifact_id: ArtifactId,
        first: MotePoolName,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(_) => {
                Err(CharacterMutationError::EssenceError(EssenceError::Mortal))
            }
            Exaltation::Exalt(exalt) => exalt.attune_artifact(artifact_id, first),
        }?;
        Ok(self)
    }

    pub fn add_sorcery_archetype_merit(
        &mut self,
        sorcery_archetype_id: SorceryArchetypeId,
        sorcery_archetype_merit_id: SorceryArchetypeMeritId,
        sorcery_archetype_merit: &'source SorceryArchetypeMerit,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.add_sorcery_archetype_merit(
                    sorcery_archetype_id,
                    sorcery_archetype_merit_id,
                    sorcery_archetype_merit,
                )?;
            }
            Exaltation::Exalt(exalt) => {
                exalt.add_sorcery_archetype_merit(
                    sorcery_archetype_id,
                    sorcery_archetype_merit_id,
                    sorcery_archetype_merit,
                )?;
            }
        }
        Ok(self)
    }

    pub fn remove_sorcery_archetype_merit(
        &mut self,
        sorcery_archetype_merit_id: SorceryArchetypeMeritId,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.remove_sorcery_archetype_merit(sorcery_archetype_merit_id)?;
            }
            Exaltation::Exalt(exalt) => {
                exalt.remove_sorcery_archetype_merit(sorcery_archetype_merit_id)?;
            }
        }
        Ok(self)
    }

    pub fn correct_sorcery_level(
        &mut self,
        occult_dots: u8,
        intelligence_dots: u8,
        essence_rating: u8,
    ) -> bool {
        match self {
            Exaltation::Mortal(mortal) => {
                if mortal.sorcery.is_some() && occult_dots < 3 {
                    mortal.sorcery = None;
                    true
                } else {
                    false
                }
            }
            Exaltation::Exalt(exalt) => {
                exalt.correct_sorcery_level(occult_dots, intelligence_dots, essence_rating)
            }
        }
    }

    pub fn add_solar_charm(
        &mut self,
        solar_charm_id: SolarCharmId,
        charm: &'source SolarCharm,
        ability_dots: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(_) => {
                return Err(CharacterMutationError::CharmError(CharmError::Mortal));
            }
            Exaltation::Exalt(exalt) => {
                exalt.add_solar_charm(solar_charm_id, charm, ability_dots)?;
            }
        }
        Ok(self)
    }

    pub fn get_solar_charm(&self, solar_charm_id: SolarCharmId) -> Option<Charm<'source>> {
        match self {
            Exaltation::Mortal(_) => None,
            Exaltation::Exalt(exalt) => exalt.get_solar_charm(solar_charm_id),
        }
    }

    pub fn solar_charms_iter(&self) -> impl Iterator<Item = SolarCharmId> + '_ {
        match self {
            Exaltation::Mortal(_) => vec![].into_iter(),
            Exaltation::Exalt(exalt) => exalt
                .solar_charms_iter()
                .collect::<Vec<SolarCharmId>>()
                .into_iter(),
        }
    }

    pub fn add_evocation(
        &mut self,
        evocation_id: EvocationId,
        evocation: &'source Evocation,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(_) => {
                return Err(CharacterMutationError::CharmError(CharmError::Mortal));
            }
            Exaltation::Exalt(exalt) => {
                exalt.add_evocation(evocation_id, evocation)?;
            }
        }
        Ok(self)
    }

    pub fn add_spell(
        &mut self,
        spell_id: SpellId,
        spell: &'source SpellMutation,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.add_spell(spell_id, spell)?;
            }
            Exaltation::Exalt(exalt) => {
                exalt.add_spell(spell_id, spell)?;
            }
        }
        Ok(self)
    }

    pub fn remove_spell(&mut self, spell_id: SpellId) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.remove_spell(spell_id)?;
            }
            Exaltation::Exalt(exalt) => {
                exalt.remove_spell(spell_id)?;
            }
        }
        Ok(self)
    }

    pub fn add_martial_arts_charm(
        &mut self,
        martial_arts_charm_id: MartialArtsCharmId,
        martial_arts_charm: &'source MartialArtsCharm,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(_) => Err(CharacterMutationError::CharmError(CharmError::Mortal)),
            Exaltation::Exalt(exalt) => {
                exalt.add_martial_arts_charm(martial_arts_charm_id, martial_arts_charm)?;
                Ok(self)
            }
        }
    }

    pub fn get_martial_arts_charm(
        &self,
        martial_arts_charm_id: MartialArtsCharmId,
    ) -> Option<Charm<'source>> {
        match self {
            Exaltation::Mortal(_) => None,
            Exaltation::Exalt(exalt) => exalt
                .martial_arts_styles
                .iter()
                .flat_map(|(_, martial_artist)| martial_artist.charms.iter())
                .find_map(|(known_charm_id, charm)| {
                    if known_charm_id == &martial_arts_charm_id {
                        Some(Charm::MartialArts(charm))
                    } else {
                        None
                    }
                }),
        }
    }

    pub fn martial_arts_charms_iter(&self) -> impl Iterator<Item = MartialArtsCharmId> + '_ {
        match self {
            Exaltation::Mortal(_) => vec![],
            Exaltation::Exalt(exalt) => exalt
                .martial_arts_styles()
                .iter()
                .flat_map(|(_, martial_artist)| {
                    martial_artist.charms.iter().map(|(charm_id, _)| *charm_id)
                })
                .collect::<Vec<MartialArtsCharmId>>(),
        }
        .into_iter()
    }

    pub(crate) fn correct_martial_arts_charms(
        &mut self,
        force_remove: &[MartialArtsCharmId],
    ) -> bool {
        match self {
            Exaltation::Mortal(_) => false,
            Exaltation::Exalt(exalt) => exalt.correct_martial_arts_charms(force_remove),
        }
    }

    pub fn remove_martial_arts_charm(
        &mut self,
        martial_arts_charm_id: MartialArtsCharmId,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(_) => Err(CharacterMutationError::CharmError(CharmError::Mortal)),
            Exaltation::Exalt(exalt) => {
                if exalt.correct_martial_arts_charms(&[martial_arts_charm_id]) {
                    Ok(self)
                } else {
                    Err(CharacterMutationError::CharmError(CharmError::NotFound))
                }
            }
        }
    }

    pub fn get_eclipse_charm(&self, spirit_charm_id: SpiritCharmId) -> Option<Charm<'source>> {
        match self {
            Exaltation::Mortal(_) => None,
            Exaltation::Exalt(exalt) => exalt.get_eclipse_charm(spirit_charm_id),
        }
    }

    pub fn eclipse_charms_iter(&self) -> impl Iterator<Item = SpiritCharmId> + '_ {
        match self {
            Exaltation::Mortal(_) => vec![],
            Exaltation::Exalt(exalt) => exalt.eclipse_charms_iter().collect::<Vec<SpiritCharmId>>(),
        }
        .into_iter()
    }
}
