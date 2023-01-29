/// Structs and methods related to the Essence rating and mote pools for a
/// character.
pub mod essence;

/// Structs and methods related to various Exalt subtypes (Solar, Lunar, etc).
pub mod exalt_type;

/// Structs and methods related to the Limit track of the Celestial Exalted.
pub mod limit;

mod anima_effect;
mod armor;
pub(crate) mod martial_arts;
mod memo;
mod sorcery;
mod weapons;
mod wonders;

pub use anima_effect::AnimaEffect;
pub(crate) use armor::ExaltArmor;
pub(crate) use limit::{Limit, LimitMemo};
pub(crate) use memo::ExaltMemo;
pub(crate) use sorcery::ExaltSorcery;
pub(crate) use weapons::{ExaltEquippedWeapons, ExaltHands, ExaltUnequippedWeapons, ExaltWeapons};
pub(crate) use wonders::ExaltWonders;

use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    num::NonZeroU8,
};

use crate::{
    abilities::{AbilityError, AbilityName, AbilityRating},
    armor::{
        armor_item::{
            artifact::{ArtifactArmorView, ArtifactError},
            mundane::MundaneArmor,
            ArmorItem, ArmorName,
        },
        ArmorError,
    },
    artifact::{
        wonders::{OwnedWonder, Wonder},
        ArtifactName,
    },
    charms::{
        charm::{evocation::Evocation, Charm, CharmName},
        CharmError,
    },
    exaltation::sorcery::ExaltationSorcery,
    hearthstones::UnslottedHearthstone,
    martial_arts::{charm::MartialArtsCharmDetails, style::MartialArtsStyle, MartialArtsError},
    sorcery::{
        circles::{
            celestial::AddCelestialSorcery, solar::AddSolarSorcery,
        },
        spell::SpellMutation,
        Sorcery, SorceryArchetypeMeritDetails, SorceryError, AddTerrestrialSorcery,
    },
    weapons::{
        weapon::{
            artifact::{
                ArtifactWeapon, HandlessArtifactWeapon, HandlessArtifactWeaponNoAttunement,
                NonnaturalArtifactWeapon,
            },
            mundane::{HandlessMundaneWeapon, MundaneWeapon},
            EquipHand, Equipped, Weapon, WeaponName,
        },
        WeaponError,
    },
    CharacterMutationError,
};

use self::{
    essence::{
        Essence, EssenceError, EssenceState, MoteCommitment, MotePoolName, MoteCommitmentName,
    },
    exalt_type::{solar::charm::SolarCharm, ExaltType},
    martial_arts::ExaltMartialArtist,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Exalt<'source> {
    pub(crate) armor: ExaltArmor<'source>,
    pub(crate) essence: EssenceState<'source>,
    pub(crate) evocations: Vec<(&'source str, &'source Evocation)>,
    pub(crate) martial_arts_styles: HashMap<&'source str, ExaltMartialArtist<'source>>,
    pub(crate) exalt_type: ExaltType<'source>,
    pub(crate) weapons: ExaltWeapons<'source>,
    pub(crate) wonders: ExaltWonders<'source>,
}

impl<'view, 'source> Exalt<'source> {
    pub fn exalt_type(&self) -> &ExaltType<'source> {
        &self.exalt_type
    }

    pub fn essence(&'view self) -> Essence<'view, 'source> {
        Essence(self)
    }

    pub fn anima_effects(&self) -> impl Iterator<Item = AnimaEffect> {
        match &self.exalt_type {
            ExaltType::Solar(solar) => solar.anima_effects(),
        }
    }

    pub fn martial_arts_styles(&self) -> &HashMap<&'source str, ExaltMartialArtist<'source>> {
        &self.martial_arts_styles
    }

    pub fn martial_arts_styles_mut(
        &mut self,
    ) -> &mut HashMap<&'source str, ExaltMartialArtist<'source>> {
        &mut self.martial_arts_styles
    }

    pub fn weapons_mut(&mut self) -> &mut ExaltWeapons<'source> {
        &mut self.weapons
    }

    pub fn get_weapon(
        &self,
        name: WeaponName<'_>,
        equipped: Option<Equipped>,
    ) -> Option<Weapon<'source>> {
        if matches!(name, WeaponName::Unarmed) {
            if matches!(equipped, Some(Equipped::Natural)) {
                Some(crate::weapons::weapon::mundane::unarmed())
            } else {
                None
            }
        } else {
            self.weapons.get_weapon(name, equipped)
        }
    }

    pub fn iter_weapons(
        &self,
    ) -> impl Iterator<Item = (WeaponName<'source>, Option<Equipped>)> + '_ {
        self.weapons.iter()
    }

    pub fn check_spend_motes(
        &self,
        _first: MotePoolName,
        amount: u8,
    ) -> Result<(), CharacterMutationError> {
        let total_available = self.essence().motes().peripheral().available()
            + self.essence().motes().personal().available();

        if total_available < amount {
            Err(CharacterMutationError::EssenceError(
                EssenceError::InsufficientMotes,
            ))
        } else {
            Ok(())
        }
    }

    pub fn spend_motes(
        &mut self,
        first: MotePoolName,
        amount: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.check_spend_motes(first, amount)?;

        let (peripheral_spent, personal_spent) = if let MotePoolName::Peripheral = first {
            let peripheral_spent = self.essence().motes().peripheral().available().min(amount);
            let personal_spent = amount - peripheral_spent;
            (peripheral_spent, personal_spent)
        } else {
            let personal_spent = self.essence().motes().personal().available().min(amount);
            let peripheral_spent = amount - personal_spent;
            (peripheral_spent, personal_spent)
        };

        self.essence
            .motes
            .peripheral_mut()
            .spend(peripheral_spent)?;
        self.essence.motes.personal_mut().spend(personal_spent)?;
        Ok(self)
    }

    pub fn commit_motes(
        &mut self,
        name: &'source str,
        first: MotePoolName,
        amount: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        let (peripheral_committed, personal_committed) = if let MotePoolName::Peripheral = first {
            let peripheral_committed = self.essence().motes().peripheral().available().min(amount);
            (
                peripheral_committed,
                self.essence()
                    .motes()
                    .personal()
                    .available()
                    .min(amount - peripheral_committed),
            )
        } else {
            let personal_committed = self.essence().motes().personal().available().min(amount);
            (
                self.essence()
                    .motes()
                    .peripheral()
                    .available()
                    .min(amount - personal_committed),
                personal_committed,
            )
        };

        if peripheral_committed + personal_committed != amount {
            return Err(CharacterMutationError::EssenceError(
                EssenceError::InsufficientMotes,
            ));
        }

        self.essence
            .motes
            .peripheral_mut()
            .commit(peripheral_committed)?;

        if let Err(e) = self.essence.motes.personal_mut().commit(personal_committed) {
            self.essence
                .motes
                .peripheral_mut()
                .uncommit(peripheral_committed)?;
            return Err(e);
        }

        if let Entry::Vacant(e) = self.essence.motes.commitments.entry(name) {
            e.insert(MoteCommitment {
                peripheral: peripheral_committed,
                personal: personal_committed,
            });
            Ok(self)
        } else {
            self.essence
                .motes
                .peripheral_mut()
                .uncommit(peripheral_committed)?;

            self.essence
                .motes
                .personal_mut()
                .uncommit(personal_committed)?;

            Err(CharacterMutationError::EssenceError(
                EssenceError::DuplicateCommitment,
            ))
        }
    }

    pub fn recover_motes(&mut self, amount: u8) -> Result<&mut Self, CharacterMutationError> {
        let peripheral_recovered = self.essence().motes().peripheral().spent().min(amount);
        let personal_recovered = self
            .essence()
            .motes()
            .personal()
            .spent()
            .min(amount - peripheral_recovered);

        self.essence
            .motes
            .peripheral_mut()
            .recover(peripheral_recovered)?;
        self.essence
            .motes
            .personal_mut()
            .recover(personal_recovered)?;
        Ok(self)
    }

    pub fn uncommit_motes(
        &mut self,
        name: MoteCommitmentName<'_>,
    ) -> Result<&mut Self, CharacterMutationError> {
        let (peripheral, personal) = match name {
            MoteCommitmentName::AttunedArtifact(artifact_name) => match artifact_name {
                ArtifactName::Weapon(artifact_weapon_name) => self
                    .weapons
                    .unattune_artifact_weapon(artifact_weapon_name)?,
                ArtifactName::Armor(artifact_armor_name) => {
                    self.armor.unattune_artifact_armor(artifact_armor_name)?
                }
                ArtifactName::Wonder(wonder_name) => self.wonders.unattune_wonder(wonder_name)?,
            },
            MoteCommitmentName::Other(other_name) => {
                let commitment = self
                    .essence
                    .motes
                    .commitments
                    .remove(other_name)
                    .ok_or(CharacterMutationError::EssenceError(EssenceError::NotFound))?;
                (commitment.peripheral, commitment.personal)
            }
        };
        self.essence.motes.peripheral.uncommit(peripheral)?;
        self.essence.motes.personal.uncommit(personal)?;
        Ok(self)
    }

    pub fn set_essence_rating(&mut self, rating: u8) -> Result<&mut Self, CharacterMutationError> {
        if self.essence().rating() == rating {
            return Ok(self);
        }

        if rating > 5 {
            return Err(CharacterMutationError::EssenceError(
                EssenceError::InvalidRating,
            ));
        }

        let rating = NonZeroU8::new(rating).ok_or(CharacterMutationError::EssenceError(
            EssenceError::InvalidRating,
        ))?;

        let (new_peripheral, new_personal) = match self.exalt_type {
            ExaltType::Solar(_) => (rating.get() * 7 + 26, rating.get() * 3 + 10),
        };

        let committed_ids = self
            .essence
            .motes
            .commitments
            .iter()
            .map(|x| MoteCommitmentName::Other(*x.0))
            .collect::<Vec<MoteCommitmentName>>();
        for id in committed_ids {
            self.uncommit_motes(id).unwrap();
        }

        let spent_peripheral = self.essence().motes().peripheral().spent();
        self.essence
            .motes
            .peripheral_mut()
            .recover(spent_peripheral)
            .unwrap();
        let available_peripheral = self.essence().motes().peripheral().available();
        if available_peripheral < new_peripheral {
            self.essence
                .motes
                .peripheral_mut()
                .uncommit(new_peripheral - available_peripheral)
                .unwrap()
                .recover(new_peripheral - available_peripheral)
                .unwrap();
        } else {
            self.essence
                .motes
                .peripheral_mut()
                .commit(available_peripheral - new_peripheral)
                .unwrap();
        }

        let spent_personal = self.essence().motes().personal().spent();
        self.essence
            .motes
            .personal_mut()
            .recover(spent_personal)
            .unwrap();
        let available_personal = self.essence().motes().personal().available();
        if available_personal < new_personal {
            self.essence
                .motes
                .personal_mut()
                .uncommit(new_personal - available_personal)
                .unwrap()
                .recover(new_personal - available_personal)
                .unwrap();
        } else {
            self.essence
                .motes
                .peripheral_mut()
                .commit(available_personal - new_personal)
                .unwrap();
        }

        self.essence.rating = rating;

        Ok(self)
    }

    pub(crate) fn add_martial_arts_style(
        &mut self,
        name: &'source str,
        style: &'source MartialArtsStyle,
    ) -> Result<&mut Self, CharacterMutationError> {
        if let Entry::Vacant(e) = self.martial_arts_styles.entry(name) {
            e.insert(ExaltMartialArtist {
                style,
                ability: AbilityRating::Zero,
                charms: Vec::new(),
            });
            Ok(self)
        } else {
            Err(CharacterMutationError::MartialArtsError(
                MartialArtsError::DuplicateStyle,
            ))
        }
    }

    pub(crate) fn remove_martial_arts_style(
        &mut self,
        name: &'source str,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.martial_arts_styles
            .remove(&name)
            .ok_or(CharacterMutationError::MartialArtsError(
                MartialArtsError::StyleNotFound,
            ))?;
        Ok(self)
    }

    pub(crate) fn set_martial_arts_dots(
        &mut self,
        name: &'source str,
        dots: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        if dots > 5 {
            Err(CharacterMutationError::AbilityError(
                AbilityError::InvalidRating,
            ))
        } else if let Some(style) = self.martial_arts_styles.get_mut(&name) {
            let old_dots = style.ability().dots();
            style.ability_mut().set_dots(dots)?;
            if old_dots > dots {
                self.correct_martial_arts_charms(&[]);
            }
            Ok(self)
        } else {
            Err(CharacterMutationError::MartialArtsError(
                MartialArtsError::StyleNotFound,
            ))
        }
    }

    pub fn add_terrestrial_sorcery(
        &mut self,
        add_terrestrial: &'source AddTerrestrialSorcery,
        occult_dots: u8,
        _intelligence_dots: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        match &mut self.exalt_type {
            ExaltType::Solar(solar) => {
                if occult_dots < 3 {
                    return Err(CharacterMutationError::SorceryError(
                        SorceryError::PrerequisitesNotMet,
                    ));
                }

                solar.add_terrestrial_sorcery(add_terrestrial)?;
            }
        }
        Ok(self)
    }

    pub fn remove_terrestrial_sorcery(&mut self) -> Result<&mut Self, CharacterMutationError> {
        match &mut self.exalt_type {
            ExaltType::Solar(solar) => {
                solar.remove_terrestrial_sorcery()?;
            }
        }
        Ok(self)
    }

    pub fn add_celestial_sorcery(
        &mut self,
        add_celestial: &'source AddCelestialSorcery,
        occult_dots: u8,
        _intelligence_dots: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        match &mut self.exalt_type {
            ExaltType::Solar(solar) => {
                if occult_dots < 4 {
                    return Err(CharacterMutationError::SorceryError(
                        SorceryError::PrerequisitesNotMet,
                    ));
                }

                solar.add_celestial_sorcery(add_celestial)?;
            }
        }
        Ok(self)
    }

    pub fn remove_celestial_sorcery(&mut self) -> Result<&mut Self, CharacterMutationError> {
        match &mut self.exalt_type {
            ExaltType::Solar(solar) => {
                solar.remove_celestial_sorcery()?;
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
        match &mut self.exalt_type {
            ExaltType::Solar(solar) => {
                if occult_dots < 5 || essence_rating < 5 {
                    return Err(CharacterMutationError::SorceryError(
                        SorceryError::PrerequisitesNotMet,
                    ));
                }

                solar.add_solar_sorcery(add_solar)?;
            }
        }
        Ok(self)
    }

    pub fn remove_solar_sorcery(&mut self) -> Result<&mut Self, CharacterMutationError> {
        match &mut self.exalt_type {
            ExaltType::Solar(solar) => {
                solar.remove_solar_sorcery()?;
            }
        }
        Ok(self)
    }

    pub(crate) fn sorcery(&'view self) -> Option<Sorcery<'view, 'source>> {
        match self.exalt_type() {
            ExaltType::Solar(solar) => solar
                .sorcery()
                .map(|sorcerer| Sorcery(ExaltationSorcery::Exalt(ExaltSorcery::Solar(sorcerer)))),
        }
    }

    pub(crate) fn add_mundane_weapon(
        &mut self,
        name: &'source str,
        weapon: &'source MundaneWeapon,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.weapons.add_mundane_weapon(name, weapon)?;
        Ok(self)
    }

    pub fn equip_weapon(
        &mut self,
        name: WeaponName<'_>,
        hand: Option<EquipHand>,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.weapons.equip_weapon(name, hand)?;
        Ok(self)
    }

    pub fn unequip_weapon(
        &mut self,
        name: WeaponName<'_>,
        equipped: Equipped,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.weapons.unequip_weapon(name, equipped)?;
        Ok(self)
    }

    pub fn add_artifact_weapon(
        &mut self,
        name: &'source str,
        weapon: ArtifactWeapon<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.weapons.add_artifact_weapon(name, weapon)?;
        Ok(self)
    }

    pub fn remove_artifact_weapon(
        &mut self,
        name: &str,
    ) -> Result<&mut Self, CharacterMutationError> {
        if let Some(NonnaturalArtifactWeapon(_, attunement)) =
            self.weapons.unequipped.artifact.get(name)
        {
            if let Some(personal) = attunement {
                let peripheral = 5 - (*personal).min(5);
                self.essence.motes.peripheral_mut().uncommit(peripheral)?;
                self.essence.motes.personal_mut().uncommit(*personal)?;
            }

            self.weapons.unequipped.artifact.remove(name);
            Ok(self)
        } else if let Some(HandlessArtifactWeapon(
            HandlessArtifactWeaponNoAttunement::Natural(_),
            attunement,
        )) = self.weapons.equipped.handless_artifact.get(name)
        {
            if let Some(personal) = attunement {
                let peripheral = 5 - (*personal).min(5);
                self.essence.motes.peripheral_mut().uncommit(peripheral)?;
                self.essence.motes.personal_mut().uncommit(*personal)?;
            }

            self.weapons.equipped.handless_artifact.remove(name);
            Ok(self)
        } else {
            Err(CharacterMutationError::WeaponError(WeaponError::NotFound))
        }
    }

    pub fn remove_mundane_weapon(
        &mut self,
        name: &str,
    ) -> Result<&mut Self, CharacterMutationError> {
        if let Some((_, count)) = self.weapons.unequipped.mundane.get(name) {
            let new_count = count.get() - 1;
            if let Some(new_nonzero) = NonZeroU8::new(new_count) {
                self.weapons.unequipped.mundane.get_mut(name).unwrap().1 = new_nonzero
            } else {
                self.weapons.unequipped.mundane.remove(name);
            }
            Ok(self)
        } else if let Some(weapon) = self.weapons.equipped.handless_mundane.get(name) {
            if matches!(weapon, HandlessMundaneWeapon::Natural(_)) {
                self.weapons.equipped.handless_mundane.remove(name);
            }
            Ok(self)
        } else {
            Err(CharacterMutationError::WeaponError(WeaponError::NotFound))
        }
    }

    pub fn worn_armor(&self) -> Option<ArmorItem<'source>> {
        self.armor.worn_armor()
    }

    pub fn armor_iter(&self) -> std::vec::IntoIter<ArmorName<'source>> {
        self.armor.iter()
    }

    pub fn get_armor(&self, name: ArmorName<'_>) -> Option<ArmorItem<'source>> {
        self.armor.get(name)
    }

    pub fn add_mundane_armor(
        &mut self,
        name: &'source str,
        armor: &'source MundaneArmor,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.armor.add_mundane(name, armor)?;
        Ok(self)
    }

    pub fn remove_mundane_armor(
        &mut self,
        name: &str,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.armor.remove_mundane(name)?;
        Ok(self)
    }

    pub fn equip_armor(
        &mut self,
        name: ArmorName<'_>,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.armor.equip(name)?;
        Ok(self)
    }

    pub fn unequip_armor(&mut self) -> Result<&mut Self, CharacterMutationError> {
        self.armor.unequip()?;
        Ok(self)
    }

    pub fn add_artifact_armor(
        &mut self,
        name: &'source str,
        armor: ArtifactArmorView<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.armor.add_artifact(name, armor)?;
        Ok(self)
    }

    pub fn remove_artifact_armor(
        &mut self,
        name: &str,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.armor.remove_artifact(name)?;
        Ok(self)
    }

    pub fn armor_mut(&mut self) -> &mut ExaltArmor<'source> {
        &mut self.armor
    }

    pub fn wonders_iter(&self) -> impl Iterator<Item = &'source str> + '_ {
        self.wonders.iter()
    }

    pub fn get_wonder(&self, name: &str) -> Option<OwnedWonder<'source>> {
        self.wonders.get(name)
    }

    pub fn wonders_mut(&mut self) -> &mut ExaltWonders<'source> {
        &mut self.wonders
    }

    pub fn add_wonder(
        &mut self,
        name: &'source str,
        wonder: &'source Wonder,
    ) -> Result<&mut Self, CharacterMutationError> {
        if let Entry::Vacant(e) = self.wonders.0.entry(name) {
            e.insert(((&wonder.0).into(), None));
            Ok(self)
        } else {
            Err(CharacterMutationError::ArtifactError(
                ArtifactError::NamedArtifactsUnique,
            ))
        }
    }

    pub fn remove_wonder(&mut self, name: &str) -> Result<&mut Self, CharacterMutationError> {
        self.wonders
            .0
            .remove(name)
            .ok_or(CharacterMutationError::ArtifactError(
                ArtifactError::NotFound,
            ))?;
        Ok(self)
    }

    pub fn slot_hearthstone_into_weapon(
        &mut self,
        artifact_weapon_name: &str,
        hearthstone_name: &'source str,
        unslotted: UnslottedHearthstone<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.weapons
            .slot_hearthstone(artifact_weapon_name, hearthstone_name, unslotted)?;
        Ok(self)
    }

    pub fn slot_hearthstone_into_armor(
        &mut self,
        artifact_armor_name: &str,
        hearthstone_name: &'source str,
        unslotted: UnslottedHearthstone<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.armor
            .slot_hearthstone(artifact_armor_name, hearthstone_name, unslotted)?;
        Ok(self)
    }

    pub fn slot_hearthstone_into_wonder(
        &mut self,
        wonder_name: &str,
        hearthstone_name: &'source str,
        unslotted: UnslottedHearthstone<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.wonders
            .slot_hearthstone(wonder_name, hearthstone_name, unslotted)?;
        Ok(self)
    }

    pub fn unslot_hearthstone_from_weapon(
        &mut self,
        artifact_weapon_name: &str,
        hearthstone_name: &str,
    ) -> Result<(&'source str, UnslottedHearthstone<'source>), CharacterMutationError> {
        self.weapons
            .unslot_hearthstone(artifact_weapon_name, hearthstone_name)
    }

    pub fn unslot_hearthstone_from_armor(
        &mut self,
        artifact_armor_name: &str,
        hearthstone_name: &str,
    ) -> Result<(&'source str, UnslottedHearthstone<'source>), CharacterMutationError> {
        self.armor
            .unslot_hearthstone(artifact_armor_name, hearthstone_name)
    }

    pub fn unslot_hearthstone_from_wonder(
        &mut self,
        wonder_name: &str,
        hearthstone_name: &str,
    ) -> Result<(&'source str, UnslottedHearthstone<'source>), CharacterMutationError> {
        self.wonders
            .unslot_hearthstone(wonder_name, hearthstone_name)
    }

    pub fn attune_artifact(
        &mut self,
        artifact_name: ArtifactName<'_>,
        first: MotePoolName,
    ) -> Result<&mut Self, CharacterMutationError> {
        let amount = self.attunement_cost(artifact_name)?;
        let (peripheral_committed, personal_committed) = if let MotePoolName::Peripheral = first {
            let peripheral_committed = self.essence().motes().peripheral().available().min(amount);
            (
                peripheral_committed,
                self.essence()
                    .motes()
                    .personal()
                    .available()
                    .min(amount - peripheral_committed),
            )
        } else {
            let personal_committed = self.essence().motes().personal().available().min(amount);
            (
                self.essence()
                    .motes()
                    .peripheral()
                    .available()
                    .min(amount - personal_committed),
                personal_committed,
            )
        };

        if peripheral_committed + personal_committed != amount {
            return Err(CharacterMutationError::EssenceError(
                EssenceError::InsufficientMotes,
            ));
        }

        self.essence
            .motes
            .peripheral_mut()
            .commit(peripheral_committed)?;

        if let Err(e) = self.essence.motes.personal_mut().commit(personal_committed) {
            self.essence
                .motes
                .peripheral_mut()
                .uncommit(peripheral_committed)?;
            return Err(e);
        }

        let outcome = match artifact_name {
            ArtifactName::Weapon(artifact_weapon_name) => self
                .weapons_mut()
                .attune_artifact_weapon(artifact_weapon_name, personal_committed)
                .err(),
            ArtifactName::Armor(artifact_armor_id) => self
                .armor_mut()
                .attune_artifact_armor(artifact_armor_id, personal_committed)
                .err(),
            ArtifactName::Wonder(wonder_id) => self
                .wonders_mut()
                .attune_wonder(wonder_id, personal_committed)
                .err(),
        };

        if let Some(e) = outcome {
            self.essence
                .motes
                .peripheral_mut()
                .uncommit(peripheral_committed)?;
            self.essence
                .motes
                .personal_mut()
                .uncommit(personal_committed)?;
            Err(e)
        } else {
            Ok(self)
        }
    }

    pub fn attunement_cost(
        &self,
        artifact_name: ArtifactName<'_>,
    ) -> Result<u8, CharacterMutationError> {
        match artifact_name {
            ArtifactName::Weapon(artifact_weapon_name) => {
                if self.weapons.iter().any(|(weapon_name, _)| {
                    weapon_name == WeaponName::Artifact(artifact_weapon_name)
                }) {
                    Ok(5)
                } else {
                    Err(CharacterMutationError::WeaponError(WeaponError::NotFound))
                }
            }
            ArtifactName::Armor(artifact_armor_name) => self
                .armor
                .get(ArmorName::Artifact(artifact_armor_name))
                .ok_or(CharacterMutationError::ArmorError(ArmorError::NotFound))?
                .attunement_cost()
                .ok_or(CharacterMutationError::EssenceError(
                    EssenceError::NoAttunementCost,
                )),
            ArtifactName::Wonder(wonder_name) => self
                .wonders
                .get(wonder_name)
                .ok_or(CharacterMutationError::ArtifactError(
                    ArtifactError::NotFound,
                ))?
                .1
                .attunement_cost
                .ok_or(CharacterMutationError::EssenceError(
                    EssenceError::NoAttunementCost,
                )),
        }
    }

    pub fn add_sorcery_archetype_merit(
        &mut self,
        sorcery_archetype_name: &str,
        sorcery_archetype_merit_name: &'source str,
        sorcery_archetype_merit: &'source SorceryArchetypeMeritDetails,
    ) -> Result<&mut Self, CharacterMutationError> {
        match &mut self.exalt_type {
            ExaltType::Solar(solar) => {
                solar.add_sorcery_archetype_merit(
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
        name: &str,
    ) -> Result<&mut Self, CharacterMutationError> {
        match &mut self.exalt_type {
            ExaltType::Solar(solar) => {
                solar.remove_sorcery_archetype_merit(name)?;
            }
        }
        Ok(self)
    }

    pub fn correct_sorcery_level(
        &mut self,
        occult_dots: u8,
        _intelligence_dots: u8,
        essence_rating: u8,
    ) -> bool {
        match &mut self.exalt_type {
            ExaltType::Solar(solar) => solar.correct_sorcery_level(occult_dots, essence_rating),
        }
    }

    pub fn add_solar_charm(
        &mut self,
        name: &'source str,
        charm: &'source SolarCharm,
        ability_dots: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        let essence_rating = self.essence.rating;
        match &mut self.exalt_type {
            ExaltType::Solar(solar) => {
                solar.add_solar_charm(name, charm, ability_dots, essence_rating.get())?;
            }
        }
        Ok(self)
    }

    pub fn get_solar_charm(&self, name: &str) -> Option<Charm<'source>> {
        match &self.exalt_type {
            ExaltType::Solar(solar) => solar.get_solar_charm(name),
        }
    }

    pub fn solar_charms_iter(&self) -> impl Iterator<Item = &'source str> + '_ {
        match &self.exalt_type {
            ExaltType::Solar(solar) => solar
                .solar_charms
                .iter()
                .map(|(id, _)| *id)
                .collect::<Vec<&str>>()
                .into_iter(),
        }
    }

    pub fn add_evocation(
        &mut self,
        name: &'source str,
        evocation: &'source Evocation,
    ) -> Result<&mut Self, CharacterMutationError> {
        let actual_essence = self.essence.rating;
        if evocation.essence_required() > actual_essence.get() {
            return Err(CharacterMutationError::CharmError(
                CharmError::PrerequisitesNotMet,
            ));
        }

        if let Some(charm_id) = evocation.upgrade() {
            let charm_exists = match charm_id {
                CharmName::Spirit(spirit_charm_id) => {
                    self.get_eclipse_charm(spirit_charm_id).is_some()
                }
                CharmName::Evocation(evocation_id) => self
                    .evocations
                    .iter()
                    .any(|(known_evocation_id, _)| known_evocation_id == &evocation_id),
                CharmName::MartialArts(martial_arts_charm_id) => self
                    .martial_arts_styles
                    .iter()
                    .flat_map(|(_, martial_artist)| martial_artist.charms.iter())
                    .any(|(charm_id, _)| charm_id == &martial_arts_charm_id),
                CharmName::Solar(solar_charm_id) => self.get_solar_charm(solar_charm_id).is_some(),
                CharmName::Spell(spell_id) => self
                    .sorcery()
                    .and_then(|sorcery| sorcery.spells().get(spell_id))
                    .is_some(),
            };

            if !charm_exists {
                return Err(CharacterMutationError::CharmError(
                    CharmError::PrerequisitesNotMet,
                ));
            }
        }

        let mut unmet_evocation_prereqs = evocation
            .evocation_prerequisites()
            .collect::<HashSet<&str>>();

        for known_evocation_id in self
            .evocations
            .iter()
            .map(|(known_evocation_id, _)| *known_evocation_id)
        {
            if name == known_evocation_id {
                return Err(CharacterMutationError::CharmError(
                    CharmError::DuplicateCharm,
                ));
            }

            unmet_evocation_prereqs.remove(&known_evocation_id);
        }

        if !unmet_evocation_prereqs.is_empty() {
            return Err(CharacterMutationError::CharmError(
                CharmError::PrerequisitesNotMet,
            ));
        }

        self.evocations.push((name, evocation));
        Ok(self)
    }

    pub fn add_spell(
        &mut self,
        name: &'source str,
        spell: &'source SpellMutation,
    ) -> Result<&mut Self, CharacterMutationError> {
        match &mut self.exalt_type {
            ExaltType::Solar(solar) => {
                solar.add_spell(name, spell)?;
            }
        }
        Ok(self)
    }

    pub fn remove_spell(&mut self, name: &str) -> Result<&mut Self, CharacterMutationError> {
        match &mut self.exalt_type {
            ExaltType::Solar(solar) => {
                solar.remove_spell(name)?;
            }
        }
        Ok(self)
    }

    pub fn add_martial_arts_charm(
        &mut self,
        name: &'source str,
        martial_arts_charm: &'source MartialArtsCharmDetails,
    ) -> Result<&mut Self, CharacterMutationError> {
        let style = self
            .martial_arts_styles
            .get(&martial_arts_charm.style())
            .ok_or(CharacterMutationError::MartialArtsError(
                MartialArtsError::StyleNotFound,
            ))?;
        let required_ability = martial_arts_charm.ability_required();
        let actual_ability = style.ability().dots();

        if required_ability > actual_ability {
            return Err(CharacterMutationError::CharmError(
                CharmError::PrerequisitesNotMet,
            ));
        }

        let required_essence = martial_arts_charm.essence_required();
        let actual_essence = self.essence.rating;
        if actual_essence.get() < required_essence {
            let mut martial_arts_supernal = false;
            // May still be okay for a Dawn caste, Martial Arts supernal solar
            let ExaltType::Solar(solar) = &self.exalt_type;
            if solar.supernal_ability() == AbilityName::MartialArts {
                martial_arts_supernal = true;
            }

            if !martial_arts_supernal {
                dbg!(solar.supernal_ability());
                return Err(CharacterMutationError::CharmError(
                    CharmError::PrerequisitesNotMet,
                ));
            }
        }

        let mut unmet_charm_prerequisites = martial_arts_charm
            .charms_required()
            .collect::<HashSet<&str>>();

        for known_charm_id in style.charms().map(|(known_charm_name, _)| known_charm_name) {
            if known_charm_id == name {
                return Err(CharacterMutationError::CharmError(
                    CharmError::DuplicateCharm,
                ));
            }

            unmet_charm_prerequisites.remove(known_charm_id);
        }

        if !unmet_charm_prerequisites.is_empty() {
            Err(CharacterMutationError::CharmError(
                CharmError::PrerequisitesNotMet,
            ))
        } else {
            self.martial_arts_styles_mut()
                .get_mut(&martial_arts_charm.style())
                .ok_or(CharacterMutationError::MartialArtsError(
                    MartialArtsError::StyleNotFound,
                ))?
                .charms
                .push((name, martial_arts_charm));
            Ok(self)
        }
    }

    pub(crate) fn correct_martial_arts_charms(&mut self, force_remove: &[&str]) -> bool {
        let actual_essence = self.essence.rating;
        let is_martial_arts_supernal = {
            let ExaltType::Solar(solar) = &self.exalt_type;
            solar.supernal_ability() == AbilityName::MartialArts
        };

        let mut any_removed = false;

        for (_, martial_artist) in self.martial_arts_styles.iter_mut() {
            let actual_ability = martial_artist.ability.dots();

            let ids_to_remove: HashSet<&str> = martial_artist.charms.iter().fold(
                HashSet::from_iter(force_remove.iter().copied()),
                |mut ids_to_remove, (known_charm_name, known_charm)| {
                    if known_charm.ability_required() > actual_ability {
                        ids_to_remove.insert(*known_charm_name);
                    }

                    if known_charm.essence_required() > actual_essence.get()
                        && !is_martial_arts_supernal
                    {
                        ids_to_remove.insert(*known_charm_name);
                    }

                    for prereq_charm_id in known_charm.charms_required() {
                        if ids_to_remove.contains(&prereq_charm_id) {
                            ids_to_remove.insert(*known_charm_name);
                        }
                    }

                    ids_to_remove
                },
            );

            let old_len = martial_artist.charms.len();
            martial_artist
                .charms
                .retain(|(known_id, _)| !ids_to_remove.contains(known_id));
            if old_len > martial_artist.charms.len() {
                any_removed = true;
            }
        }

        any_removed
    }

    pub fn get_eclipse_charm(&self, name: &str) -> Option<Charm<'source>> {
        match &self.exalt_type {
            ExaltType::Solar(solar) => solar.get_eclipse_charm(name),
        }
    }

    pub fn eclipse_charms_iter(&self) -> impl Iterator<Item = &'source str> + '_ {
        match &self.exalt_type {
            ExaltType::Solar(solar) => solar.eclipse_charms_iter(),
        }
    }
}
