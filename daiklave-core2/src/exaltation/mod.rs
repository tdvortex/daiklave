/// Traits common to all Exalts
pub mod exalt;

/// Traits which are unique to mortals (or which function differently without
///  Essence)
pub mod mortal;

mod martial_artist;
mod memo;
mod sorcery;

use std::num::NonZeroU8;

pub(crate) use martial_artist::ExaltationMartialArtist;
pub(crate) use memo::ExaltationMemo;
pub(crate) use sorcery::ExaltationSorcery;

use crate::{
    armor::armor_item::{artifact::ArtifactArmorView, mundane::MundaneArmor, ArmorItem, ArmorName},
    artifact::{
        wonders::{OwnedWonder, Wonder},
        ArtifactName, ArtifactNameMutation,
    },
    charms::{
        charm::{evocation::Evocation, Charm},
        CharmError,
    },
    hearthstones::UnslottedHearthstone,
    martial_arts::{
        charm::{MartialArtsCharm, MartialArtsCharmDetails},
        style::MartialArtsStyleDetails,
        MartialArtsStyle,
    },
    merits::merit::SorceryArchetypeMeritDetails,
    sorcery::{
        circles::{
            celestial::{sorcerer::CelestialCircleSorcerer, AddCelestialSorcery},
            solar::AddSolarSorcery,
        },
        spell::SpellMutation,
        AddTerrestrialSorcery, Sorcery, SorceryError,
    },
    weapons::weapon::{
        artifact::ArtifactWeapon, mundane::MundaneWeapon, EquipHand, Equipped, Weapon, WeaponName,
    },
    CharacterMutationError,
};

use self::{
    exalt::{
        essence::{
            Essence, EssenceError, EssenceState, MoteCommitmentName, MoteCommitmentNameMutation,
            MotePoolName, MotesState, UncommitMotes,
        },
        exalt_type::{
            solar::{charm::SolarCharmDetails, Solar, SolarMemo, SolarSorcererView},
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
    pub fn get_weapon(
        &self,
        weapon_name: WeaponName<'_>,
        equipped: Option<Equipped>,
    ) -> Option<Weapon<'source>> {
        if matches!(weapon_name, WeaponName::Unarmed) {
            if matches!(equipped, Some(Equipped::Natural)) {
                Some(crate::weapons::weapon::mundane::unarmed())
            } else {
                None
            }
        } else {
            match self {
                Exaltation::Mortal(box_mortal) => {
                    box_mortal.as_ref().get_weapon(weapon_name, equipped)
                }
                Exaltation::Exalt(box_exalt) => {
                    box_exalt.as_ref().get_weapon(weapon_name, equipped)
                }
            }
        }
    }

    pub fn iter_weapons(&self) -> impl Iterator<Item = (WeaponName<'source>, Option<Equipped>)> {
        match self {
            Exaltation::Mortal(box_mortal) => box_mortal
                .as_ref()
                .iter_weapons()
                .collect::<Vec<(WeaponName, Option<Equipped>)>>(),
            Exaltation::Exalt(box_exalt) => box_exalt
                .as_ref()
                .iter_weapons()
                .collect::<Vec<(WeaponName, Option<Equipped>)>>(),
        }
        .into_iter()
    }

    pub fn set_mortal(&mut self) -> Result<&mut Self, CharacterMutationError> {
        if matches!(self, Exaltation::Mortal(_)) {
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
                            SolarSorcererView::Celestial(celestial) => Some(
                                celestial
                                    .try_into()
                                    .map_err(CharacterMutationError::SorceryError)?,
                            ),
                            SolarSorcererView::Solar(solar) => Some(
                                (&Into::<CelestialCircleSorcerer>::into(solar))
                                    .try_into()
                                    .map_err(CharacterMutationError::SorceryError)?,
                            ),
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
        name: &'source str,
        style: &'source MartialArtsStyleDetails,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.add_martial_arts_style(name, style)?;
            }
            Exaltation::Exalt(exalt) => {
                exalt.add_martial_arts_style(name, style)?;
            }
        }
        Ok(self)
    }

    pub(crate) fn remove_martial_arts_style(
        &mut self,
        name: &str,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.remove_martial_arts_style(name)?;
            }
            Exaltation::Exalt(exalt) => {
                exalt.remove_martial_arts_style(name)?;
            }
        }
        Ok(self)
    }

    pub(crate) fn set_martial_arts_dots(
        &mut self,
        name: &str,
        dots: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.set_martial_arts_dots(name, dots)?;
            }
            Exaltation::Exalt(exalt) => {
                exalt.set_martial_arts_dots(name, dots)?;
            }
        }
        Ok(self)
    }
}

impl<'view, 'source> Exaltation<'source> {
    pub(crate) fn martial_artist(
        &'view self,
        name: &str,
    ) -> Option<MartialArtsStyle<'view, 'source>> {
        match self {
            Exaltation::Mortal(mortal) => {
                let (name, mortal_martial_artist) =
                    mortal.martial_arts_styles.get_key_value(name)?;
                Some(MartialArtsStyle {
                    name,
                    maybe_exalt: ExaltationMartialArtist::Mortal(mortal_martial_artist),
                })
            }
            Exaltation::Exalt(exalt) => {
                let (name, exalt_martial_artist) = exalt.martial_arts_styles.get_key_value(name)?;
                Some(MartialArtsStyle {
                    name,
                    maybe_exalt: ExaltationMartialArtist::Exalt(exalt_martial_artist),
                })
            }
        }
    }

    pub(crate) fn martial_arts_id_iter(&'view self) -> impl Iterator<Item = &'source str> {
        let mut ids = match self {
            Exaltation::Mortal(mortal) => mortal
                .martial_arts_styles
                .keys()
                .copied()
                .collect::<Vec<&str>>(),
            Exaltation::Exalt(exalt) => exalt
                .martial_arts_styles()
                .keys()
                .copied()
                .collect::<Vec<&str>>(),
        };

        ids.sort_by(|a, b| {
            self.martial_artist(a)
                .unwrap()
                .name()
                .cmp(self.martial_artist(b).unwrap().name())
        });
        ids.into_iter()
    }

    pub fn add_terrestrial_sorcery(
        &mut self,
        add_terrestrial: &'source AddTerrestrialSorcery,
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

    pub fn set_solar(
        &mut self,
        solar: &'source SolarMemo,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.set_solar_view(solar.into())
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
                        rating: NonZeroU8::new(1).unwrap(),
                        motes: MotesState {
                            peripheral_available: 33,
                            peripheral_spent: 0,
                            personal_available: 13,
                            personal_spent: 0,
                            other_commitments: Default::default(),
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
                    .essence
                    .motes
                    .other_commitments
                    .iter()
                    .map(|(name, _)| MoteCommitmentName::Other(name))
                    .collect::<Vec<MoteCommitmentName>>();
                for commit_id in to_uncommit.into_iter() {
                    exalt.uncommit_motes(commit_id)?;
                }

                // If switching solar->solar, try to preserve solar charms
                let ExaltType::Solar(old_solar) = &mut exalt.exalt_type;
                solar.solar_charms = std::mem::take(&mut old_solar.solar_charms);

                // If switching solar->solar, preserve Solar experience
                solar.experience = std::mem::take(&mut old_solar.experience);

                // If switching solar->solar, preserve Limit track
                solar.limit.track = old_solar.limit.track;

                // Preserve sorcery
                if let Some(solar_sorcerer) = old_solar.sorcery.take() {
                    solar.sorcery = Some(solar_sorcerer);
                }

                *self = Self::Exalt(Box::new(Exalt {
                    armor: std::mem::take(&mut exalt.armor),
                    essence: EssenceState {
                        rating: exalt.essence.rating,
                        motes: MotesState {
                            peripheral_available: 26 * exalt.essence().rating() * 7,
                            peripheral_spent: 0,
                            personal_available: 10 + exalt.essence().rating() * 3,
                            personal_spent: 0,
                            other_commitments: Default::default(),
                        },
                    },
                    // Preserve Evocations
                    evocations: std::mem::take(&mut exalt.evocations),
                    // Try to preserve martial arts styles (including charms)
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
        amount: NonZeroU8,
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
        name: &'source str,
        first: MotePoolName,
        amount: NonZeroU8,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(_) => {
                Err(CharacterMutationError::EssenceError(EssenceError::Mortal))
            }
            Exaltation::Exalt(exalt) => exalt.commit_motes(name, first, amount),
        }?;
        Ok(self)
    }

    pub fn recover_motes(
        &mut self,
        amount: NonZeroU8,
    ) -> Result<&mut Self, CharacterMutationError> {
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
        to_uncommit: &'source UncommitMotes,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(_) => {
                Err(CharacterMutationError::EssenceError(EssenceError::Mortal))
            }
            Exaltation::Exalt(exalt) => exalt.uncommit_motes(match &to_uncommit.0 {
                MoteCommitmentNameMutation::AttunedArtifact(ArtifactNameMutation::Armor(name)) => {
                    MoteCommitmentName::AttunedArtifact(ArtifactName::Armor(name.as_str()))
                }
                MoteCommitmentNameMutation::AttunedArtifact(ArtifactNameMutation::Wonder(
                    wonder_name,
                )) => {
                    MoteCommitmentName::AttunedArtifact(ArtifactName::Wonder(wonder_name.as_str()))
                }
                MoteCommitmentNameMutation::AttunedArtifact(ArtifactNameMutation::Weapon(name)) => {
                    MoteCommitmentName::AttunedArtifact(ArtifactName::Weapon(name.as_str()))
                }
                MoteCommitmentNameMutation::Other(name) => MoteCommitmentName::Other(name.as_str()),
            }),
        }?;
        Ok(self)
    }

    pub fn set_essence_rating(
        &mut self,
        rating: NonZeroU8,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Exalt(exalt) => {
                if rating > NonZeroU8::new(5).unwrap() {
                    return Err(CharacterMutationError::EssenceError(
                        EssenceError::InvalidRating,
                    ));
                }

                let old_rating = exalt.essence().rating();
                exalt.set_essence_rating(rating)?;
                if old_rating > rating.get() {
                    if rating < NonZeroU8::new(5).unwrap() {
                        exalt.remove_solar_sorcery().ok();
                    }

                    if rating < NonZeroU8::new(3).unwrap() {
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
        name: &'source str,
        weapon: &'source MundaneWeapon,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.as_mut().add_mundane_weapon(name, weapon)?;
            }
            Exaltation::Exalt(exalt) => {
                exalt.as_mut().add_mundane_weapon(name, weapon)?;
            }
        }
        Ok(self)
    }

    pub fn equip_weapon(
        &mut self,
        name: WeaponName<'_>,
        hand: Option<EquipHand>,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.as_mut().equip_weapon(name, hand)?;
            }
            Exaltation::Exalt(exalt) => {
                exalt.as_mut().equip_weapon(name, hand)?;
            }
        }
        Ok(self)
    }

    pub fn unequip_weapon(
        &mut self,
        weapon_name: WeaponName<'_>,
        equipped: Equipped,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.as_mut().unequip_weapon(weapon_name, equipped)?;
            }
            Exaltation::Exalt(exalt) => {
                exalt.as_mut().unequip_weapon(weapon_name, equipped)?;
            }
        }
        Ok(self)
    }

    pub fn add_artifact_weapon(
        &mut self,
        name: &'source str,
        weapon: ArtifactWeapon<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.add_artifact_weapon(name, weapon)?;
            }
            Exaltation::Exalt(exalt) => {
                exalt.add_artifact_weapon(name, weapon)?;
            }
        }
        Ok(self)
    }

    pub fn remove_artifact_weapon(
        &mut self,
        name: &str,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.remove_artifact_weapon(name)?;
            }
            Exaltation::Exalt(exalt) => {
                exalt.remove_artifact_weapon(name)?;
            }
        }
        Ok(self)
    }

    pub fn remove_mundane_weapon(
        &mut self,
        name: &str,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.remove_mundane_weapon(name)?;
            }
            Exaltation::Exalt(exalt) => {
                exalt.remove_mundane_weapon(name)?;
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

    pub fn armor_iter(&self) -> impl Iterator<Item = ArmorName<'source>> + '_ {
        match self {
            Exaltation::Mortal(mortal) => mortal.armor_iter(),
            Exaltation::Exalt(exalt) => exalt.armor_iter(),
        }
    }

    pub fn get_armor(&self, name: ArmorName<'_>) -> Option<ArmorItem<'source>> {
        match self {
            Exaltation::Mortal(mortal) => mortal.get_armor(name),
            Exaltation::Exalt(exalt) => exalt.get_armor(name),
        }
    }

    pub fn add_mundane_armor(
        &mut self,
        name: &'source str,
        armor: &'source MundaneArmor,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.add_mundane_armor(name, armor)?;
            }
            Exaltation::Exalt(exalt) => {
                exalt.add_mundane_armor(name, armor)?;
            }
        }
        Ok(self)
    }

    pub fn remove_mundane_armor(
        &mut self,
        name: &str,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.remove_mundane_armor(name)?;
            }
            Exaltation::Exalt(exalt) => {
                exalt.remove_mundane_armor(name)?;
            }
        }
        Ok(self)
    }

    pub fn equip_armor(
        &mut self,
        name: ArmorName<'_>,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.equip_armor(name)?;
            }
            Exaltation::Exalt(exalt) => {
                exalt.equip_armor(name)?;
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
        name: &'source str,
        armor: ArtifactArmorView<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.add_artifact_armor(name, armor)?;
            }
            Exaltation::Exalt(exalt) => {
                exalt.add_artifact_armor(name, armor)?;
            }
        }
        Ok(self)
    }

    pub fn remove_artifact_armor(
        &mut self,
        name: &str,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.remove_artifact_armor(name)?;
            }
            Exaltation::Exalt(exalt) => {
                exalt.remove_artifact_armor(name)?;
            }
        }
        Ok(self)
    }

    pub fn wonders_iter(&self) -> impl Iterator<Item = &'source str> + '_ {
        match self {
            Exaltation::Mortal(mortal) => mortal.wonders_iter().collect::<Vec<&str>>().into_iter(),
            Exaltation::Exalt(exalt) => exalt.wonders_iter().collect::<Vec<&str>>().into_iter(),
        }
    }

    pub fn get_wonder(&self, name: &str) -> Option<OwnedWonder<'source>> {
        match self {
            Exaltation::Mortal(mortal) => mortal.get_wonder(name),
            Exaltation::Exalt(exalt) => exalt.get_wonder(name),
        }
    }

    pub fn add_wonder(
        &mut self,
        name: &'source str,
        wonder: &'source Wonder,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.add_wonder(name, wonder)?;
            }
            Exaltation::Exalt(exalt) => {
                exalt.add_wonder(name, wonder)?;
            }
        }
        Ok(self)
    }

    pub fn remove_wonder(&mut self, name: &str) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.remove_wonder(name)?;
            }
            Exaltation::Exalt(exalt) => {
                exalt.remove_wonder(name)?;
            }
        }
        Ok(self)
    }

    pub fn slot_hearthstone_into_weapon(
        &mut self,
        artifact_weapon_name: &str,
        hearthstone_name: &'source str,
        unslotted: UnslottedHearthstone<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.slot_hearthstone_into_weapon(
                    artifact_weapon_name,
                    hearthstone_name,
                    unslotted,
                )?;
            }
            Exaltation::Exalt(exalt) => {
                exalt.slot_hearthstone_into_weapon(
                    artifact_weapon_name,
                    hearthstone_name,
                    unslotted,
                )?;
            }
        }
        Ok(self)
    }

    pub fn slot_hearthstone_into_armor(
        &mut self,
        artifact_armor_name: &str,
        hearthstone_name: &'source str,
        unslotted: UnslottedHearthstone<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.slot_hearthstone_into_armor(
                    artifact_armor_name,
                    hearthstone_name,
                    unslotted,
                )?;
            }
            Exaltation::Exalt(exalt) => {
                exalt.slot_hearthstone_into_armor(
                    artifact_armor_name,
                    hearthstone_name,
                    unslotted,
                )?;
            }
        }
        Ok(self)
    }

    pub fn slot_hearthstone_into_wonder(
        &mut self,
        wonder_name: &str,
        hearthstone_name: &'source str,
        unslotted: UnslottedHearthstone<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.slot_hearthstone_into_wonder(wonder_name, hearthstone_name, unslotted)?;
            }
            Exaltation::Exalt(exalt) => {
                exalt.slot_hearthstone_into_wonder(wonder_name, hearthstone_name, unslotted)?;
            }
        }
        Ok(self)
    }

    pub fn unslot_hearthstone_from_weapon(
        &mut self,
        artifact_weapon_name: &str,
        hearthstone_name: &str,
    ) -> Result<(&'source str, UnslottedHearthstone<'source>), CharacterMutationError> {
        Ok(match self {
            Exaltation::Mortal(mortal) => {
                mortal.unslot_hearthstone_from_weapon(artifact_weapon_name, hearthstone_name)?
            }
            Exaltation::Exalt(exalt) => {
                exalt.unslot_hearthstone_from_weapon(artifact_weapon_name, hearthstone_name)?
            }
        })
    }

    pub fn unslot_hearthstone_from_armor(
        &mut self,
        artifact_armor_name: &str,
        hearthstone_name: &str,
    ) -> Result<(&'source str, UnslottedHearthstone<'source>), CharacterMutationError> {
        Ok(match self {
            Exaltation::Mortal(mortal) => {
                mortal.unslot_hearthstone_from_armor(artifact_armor_name, hearthstone_name)?
            }
            Exaltation::Exalt(exalt) => {
                exalt.unslot_hearthstone_from_armor(artifact_armor_name, hearthstone_name)?
            }
        })
    }

    pub fn unslot_hearthstone_from_wonder(
        &mut self,
        wonder_name: &str,
        hearthstone_name: &str,
    ) -> Result<(&'source str, UnslottedHearthstone<'source>), CharacterMutationError> {
        Ok(match self {
            Exaltation::Mortal(mortal) => {
                mortal.unslot_hearthstone_from_wonder(wonder_name, hearthstone_name)?
            }
            Exaltation::Exalt(exalt) => {
                exalt.unslot_hearthstone_from_wonder(wonder_name, hearthstone_name)?
            }
        })
    }

    pub fn attune_artifact(
        &mut self,
        artifact_name: ArtifactName<'_>,
        first: MotePoolName,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(_) => {
                Err(CharacterMutationError::EssenceError(EssenceError::Mortal))
            }
            Exaltation::Exalt(exalt) => exalt.attune_artifact(artifact_name, first),
        }?;
        Ok(self)
    }

    pub fn add_sorcery_archetype_merit(
        &mut self,
        sorcery_archetype_name: &str,
        sorcery_archetype_merit_name: &'source str,
        sorcery_archetype_merit: &'source SorceryArchetypeMeritDetails,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.add_sorcery_archetype_merit(
                    sorcery_archetype_name,
                    sorcery_archetype_merit_name,
                    sorcery_archetype_merit,
                )?;
            }
            Exaltation::Exalt(exalt) => {
                exalt.add_sorcery_archetype_merit(
                    sorcery_archetype_name,
                    sorcery_archetype_merit_name,
                    sorcery_archetype_merit,
                )?;
            }
        }
        Ok(self)
    }

    pub fn remove_sorcery_archetype_merit(
        &mut self,
        sorcery_archetype_name: &str,
        merit_name: &str,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.remove_sorcery_archetype_merit(sorcery_archetype_name, merit_name)?;
            }
            Exaltation::Exalt(exalt) => {
                exalt.remove_sorcery_archetype_merit(sorcery_archetype_name, merit_name)?;
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
        name: &'source str,
        details: &'source SolarCharmDetails,
        ability_dots: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(_) => {
                return Err(CharacterMutationError::CharmError(CharmError::Mortal));
            }
            Exaltation::Exalt(exalt) => {
                exalt.add_solar_charm(name, details, ability_dots)?;
            }
        }
        Ok(self)
    }

    pub fn get_solar_charm(&self, name: &str) -> Option<Charm<'source>> {
        match self {
            Exaltation::Mortal(_) => None,
            Exaltation::Exalt(exalt) => exalt.get_solar_charm(name),
        }
    }

    pub fn solar_charms_iter(&self) -> impl Iterator<Item = &'source str> + '_ {
        match self {
            Exaltation::Mortal(_) => vec![].into_iter(),
            Exaltation::Exalt(exalt) => {
                exalt.solar_charms_iter().collect::<Vec<&str>>().into_iter()
            }
        }
    }

    pub fn add_evocation(
        &mut self,
        name: &'source str,
        evocation: &'source Evocation,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(_) => {
                return Err(CharacterMutationError::CharmError(CharmError::Mortal));
            }
            Exaltation::Exalt(exalt) => {
                exalt.add_evocation(name, evocation)?;
            }
        }
        Ok(self)
    }

    pub fn add_spell(
        &mut self,
        name: &'source str,
        spell: &'source SpellMutation,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.add_spell(name, spell)?;
            }
            Exaltation::Exalt(exalt) => {
                exalt.add_spell(name, spell)?;
            }
        }
        Ok(self)
    }

    pub fn remove_spell(&mut self, name: &str) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.remove_spell(name)?;
            }
            Exaltation::Exalt(exalt) => {
                exalt.remove_spell(name)?;
            }
        }
        Ok(self)
    }

    pub fn add_martial_arts_charm(
        &mut self,
        style_name: &str,
        name: &'source str,
        martial_arts_charm: &'source MartialArtsCharmDetails,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(_) => Err(CharacterMutationError::CharmError(CharmError::Mortal)),
            Exaltation::Exalt(exalt) => {
                exalt.add_martial_arts_charm(style_name, name, martial_arts_charm)?;
                Ok(self)
            }
        }
    }

    pub fn get_martial_arts_charm(&self, name: &str) -> Option<Charm<'source>> {
        match self {
            Exaltation::Mortal(_) => None,
            Exaltation::Exalt(exalt) => exalt
                .martial_arts_styles
                .iter()
                .flat_map(|(style_name, martial_artist)| {
                    martial_artist
                        .charms
                        .iter()
                        .map(|(charm_name, charm_details)| {
                            (*style_name, *charm_name, *charm_details)
                        })
                })
                .find_map(|(style_name, known_charm_name, details)| {
                    if known_charm_name == name {
                        Some(Charm::MartialArts(MartialArtsCharm {
                            name: known_charm_name,
                            style_name,
                            details,
                        }))
                    } else {
                        None
                    }
                }),
        }
    }

    pub fn martial_arts_charms_iter(&self) -> impl Iterator<Item = &'source str> + '_ {
        match self {
            Exaltation::Mortal(_) => vec![],
            Exaltation::Exalt(exalt) => exalt
                .martial_arts_styles()
                .iter()
                .flat_map(|(_, martial_artist)| {
                    martial_artist
                        .charms
                        .iter()
                        .map(|(charm_name, _)| *charm_name)
                })
                .collect::<Vec<&str>>(),
        }
        .into_iter()
    }

    pub(crate) fn correct_martial_arts_charms(&mut self, force_remove: &[&str]) -> bool {
        match self {
            Exaltation::Mortal(_) => false,
            Exaltation::Exalt(exalt) => exalt.correct_martial_arts_charms(force_remove),
        }
    }

    pub fn remove_martial_arts_charm(
        &mut self,
        name: &str,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(_) => Err(CharacterMutationError::CharmError(CharmError::Mortal)),
            Exaltation::Exalt(exalt) => {
                if exalt.correct_martial_arts_charms(&[name]) {
                    Ok(self)
                } else {
                    Err(CharacterMutationError::CharmError(CharmError::NotFound))
                }
            }
        }
    }

    pub fn get_eclipse_charm(&self, name: &str) -> Option<Charm<'source>> {
        match self {
            Exaltation::Mortal(_) => None,
            Exaltation::Exalt(exalt) => exalt.get_eclipse_charm(name),
        }
    }

    pub fn eclipse_charms_iter(&self) -> impl Iterator<Item = &'source str> + '_ {
        match self {
            Exaltation::Mortal(_) => vec![],
            Exaltation::Exalt(exalt) => exalt.eclipse_charms_iter().collect::<Vec<&str>>(),
        }
        .into_iter()
    }

    pub fn add_martial_arts_specialty(&mut self, style_name: &str, specialty: &'source str) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {mortal.add_martial_arts_specialty(style_name, specialty)?;}
            Exaltation::Exalt(exalt) => {exalt.add_martial_arts_specialty(style_name, specialty)?;}
        }
        Ok(self)
    }

    pub fn remove_martial_arts_specialty(&mut self, style_name: &str, specialty: &str) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {mortal.remove_martial_arts_specialty(style_name, specialty)?;}
            Exaltation::Exalt(exalt) => {exalt.remove_martial_arts_specialty(style_name, specialty)?;}
        }
        Ok(self)
    }
}
