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
    armor::armor_item::{
        artifact::{ArtifactArmorView, ArtifactError},
        mundane::MundaneArmor,
        ArmorItem, ArmorName, ArmorWeightClass, EquippedArmor,
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
    martial_arts::{
        charm::MartialArtsCharmDetails, style::MartialArtsStyleDetails, MartialArtsError,
    },
    merits::merit::SorceryArchetypeMeritDetails,
    sorcery::{
        circles::{celestial::AddCelestialSorcery, solar::AddSolarSorcery},
        spell::SpellMutation,
        AddTerrestrialSorcery, Sorcery, SorceryError,
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
    essence::{Essence, EssenceError, EssenceState, MoteCommitmentName, MotePoolName},
    exalt_type::{solar::charm::SolarCharmDetails, ExaltType},
    martial_arts::ExaltMartialArtistDetails,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Exalt<'source> {
    pub(crate) armor: ExaltArmor<'source>,
    pub(crate) essence: EssenceState<'source>,
    pub(crate) evocations: Vec<(&'source str, &'source Evocation)>,
    pub(crate) martial_arts_styles: HashMap<&'source str, ExaltMartialArtistDetails<'source>>,
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

    pub fn martial_arts_styles(
        &self,
    ) -> &HashMap<&'source str, ExaltMartialArtistDetails<'source>> {
        &self.martial_arts_styles
    }

    pub fn martial_arts_styles_mut(
        &mut self,
    ) -> &mut HashMap<&'source str, ExaltMartialArtistDetails<'source>> {
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

    pub fn spend_motes(
        &mut self,
        first: MotePoolName,
        amount: NonZeroU8,
    ) -> Result<&mut Self, CharacterMutationError> {
        let amount = amount.get();
        let (peripheral, personal) = self.essence().motes().peripheral_and_personal();

        let (peripheral_available, personal_available) = (peripheral.available, personal.available);

        if amount > (peripheral_available + personal_available) {
            return Err(CharacterMutationError::EssenceError(
                EssenceError::InsufficientMotes,
            ));
        }

        let (peripheral_spent, personal_spent) = if let MotePoolName::Peripheral = first {
            let peripheral_spent = peripheral_available.min(amount);
            let personal_spent = amount - peripheral_spent;
            (peripheral_spent, personal_spent)
        } else {
            let personal_spent = personal_available.min(amount);
            let peripheral_spent = amount - personal_spent;
            (peripheral_spent, personal_spent)
        };

        self.essence.motes.peripheral_available -= peripheral_spent;
        self.essence.motes.personal_available -= personal_spent;
        self.essence.motes.peripheral_spent += peripheral_spent;
        self.essence.motes.personal_spent += personal_spent;
        Ok(self)
    }

    pub fn commit_motes(
        &mut self,
        name: &'source str,
        first: MotePoolName,
        amount: NonZeroU8,
    ) -> Result<&mut Self, CharacterMutationError> {
        if self.essence.motes.other_commitments.contains_key(name) {
            return Err(CharacterMutationError::EssenceError(
                EssenceError::DuplicateCommitment,
            ));
        }

        let amount = amount.get();
        let (peripheral, personal) = self.essence().motes().peripheral_and_personal();
        let (peripheral_available, personal_available) = (peripheral.available, personal.available);

        if amount > (peripheral_available + personal_available) {
            return Err(CharacterMutationError::EssenceError(
                EssenceError::InsufficientMotes,
            ));
        }

        let (peripheral_committed, personal_committed) = if let MotePoolName::Peripheral = first {
            let peripheral_committed = peripheral_available.min(amount);
            let personal_committed = amount - peripheral_committed;
            (peripheral_committed, personal_committed)
        } else {
            let personal_committed = personal_available.min(amount);
            let peripheral_committed = amount - personal_committed;
            (peripheral_committed, personal_committed)
        };

        self.essence.motes.peripheral_available -= peripheral_committed;
        self.essence.motes.personal_available -= personal_committed;
        self.essence
            .motes
            .other_commitments
            .insert(name, (peripheral_committed, personal_committed));
        Ok(self)
    }

    pub fn recover_motes(
        &mut self,
        amount: NonZeroU8,
    ) -> Result<&mut Self, CharacterMutationError> {
        let amount = amount.get();
        let (peripheral, personal) = self.essence().motes().peripheral_and_personal();
        let (peripheral_spent, personal_spent) = (peripheral.spent, personal.spent);

        if amount > peripheral_spent + peripheral_spent {
            return Err(CharacterMutationError::EssenceError(
                EssenceError::InsufficientMotes,
            ));
        }

        let peripheral_recovered = peripheral_spent.min(amount);
        let personal_recovered = personal_spent.min(amount - peripheral_recovered);

        self.essence.motes.peripheral_spent -= peripheral_recovered;
        self.essence.motes.peripheral_available += peripheral_recovered;
        self.essence.motes.personal_spent -= peripheral_recovered;
        self.essence.motes.personal_available += personal_recovered;
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
            MoteCommitmentName::Other(other_name) => self
                .essence
                .motes
                .other_commitments
                .remove(other_name)
                .ok_or(CharacterMutationError::EssenceError(EssenceError::NotFound))?,
        };
        self.essence.motes.peripheral_spent = self
            .essence
            .motes
            .peripheral_spent
            .saturating_add(peripheral);
        self.essence.motes.personal_spent =
            self.essence.motes.personal_spent.saturating_add(personal);
        Ok(self)
    }

    pub fn set_essence_rating(
        &mut self,
        rating: NonZeroU8,
    ) -> Result<&mut Self, CharacterMutationError> {
        if self.essence().rating() == rating.get() {
            return Ok(self);
        }

        if rating > NonZeroU8::new(5).unwrap() {
            return Err(CharacterMutationError::EssenceError(
                EssenceError::InvalidRating,
            ));
        }

        let (new_peripheral, new_personal) = match self.exalt_type {
            ExaltType::Solar(_) => (rating.get() * 7 + 26, rating.get() * 3 + 10),
        };

        let commitment_names = self
            .essence()
            .motes()
            .committed()
            .map(|commitment| commitment.name())
            .collect::<Vec<MoteCommitmentName>>();
        for name in commitment_names.into_iter() {
            self.uncommit_motes(name)?;
        }
        self.essence.motes.peripheral_available = new_peripheral;
        self.essence.motes.peripheral_spent = 0;
        self.essence.motes.personal_available = new_personal;
        self.essence.motes.personal_spent = 0;
        self.essence.rating = rating;
        Ok(self)
    }

    pub(crate) fn add_martial_arts_style(
        &mut self,
        name: &'source str,
        style: &'source MartialArtsStyleDetails,
    ) -> Result<&mut Self, CharacterMutationError> {
        if let Entry::Vacant(e) = self.martial_arts_styles.entry(name) {
            e.insert(ExaltMartialArtistDetails {
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
        name: &str,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.martial_arts_styles
            .remove(name)
            .ok_or(CharacterMutationError::MartialArtsError(
                MartialArtsError::StyleNotFound,
            ))?;
        Ok(self)
    }

    pub(crate) fn set_martial_arts_dots(
        &mut self,
        name: &str,
        dots: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        if dots > 5 {
            Err(CharacterMutationError::AbilityError(
                AbilityError::InvalidRating,
            ))
        } else if let Some(style) = self.martial_arts_styles.get_mut(name) {
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
            if matches!(attunement, Some(_)) {
                self.uncommit_motes(MoteCommitmentName::AttunedArtifact(ArtifactName::Weapon(
                    name,
                )))?;
            }
            self.weapons.unequipped.artifact.remove(name);
            Ok(self)
        } else if let Some(HandlessArtifactWeapon(
            HandlessArtifactWeaponNoAttunement::Natural(_),
            attunement,
        )) = self.weapons.equipped.handless_artifact.get(name)
        {
            if matches!(attunement, Some(_)) {
                self.uncommit_motes(MoteCommitmentName::AttunedArtifact(ArtifactName::Weapon(
                    name,
                )))?;
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
        match artifact_name {
            ArtifactName::Weapon(artifact_weapon_name) => {
                self.attune_artifact_weapon(artifact_weapon_name, first)
            }
            ArtifactName::Armor(artifact_armor_name) => {
                self.attune_artifact_armor(artifact_armor_name, first)
            }
            ArtifactName::Wonder(wonder_name) => self.attune_wonder(wonder_name, first),
        }
    }

    pub fn attune_artifact_weapon(
        &mut self,
        artifact_weapon_name: &str,
        first: MotePoolName,
    ) -> Result<&mut Self, CharacterMutationError> {
        if self.essence.motes.peripheral_available + self.essence.motes.personal_available < 5 {
            return Err(CharacterMutationError::EssenceError(
                EssenceError::InsufficientMotes,
            ));
        }
        let (peripheral_committed, personal_committed) = if first == MotePoolName::Peripheral {
            let peripheral_committed = self.essence.motes.peripheral_available.min(5);
            let personal_committed = 5 - peripheral_committed;
            (peripheral_committed, personal_committed)
        } else {
            let personal_committed = self.essence.motes.personal_available.min(5);
            let peripheral_committed = 5 - personal_committed;
            (peripheral_committed, personal_committed)
        };

        self.weapons
            .attune_artifact_weapon(artifact_weapon_name, personal_committed)?;
        self.essence.motes.peripheral_available -= peripheral_committed;
        self.essence.motes.personal_available -= personal_committed;
        Ok(self)
    }

    pub fn attune_artifact_armor(
        &mut self,
        artifact_armor_name: &str,
        first: MotePoolName,
    ) -> Result<&mut Self, CharacterMutationError> {
        let mut maybe_armor = None;
        if let Some(armor) = &mut self.armor.equipped {
            if let EquippedArmor::Artifact(name, armor) = armor {
                if name == &artifact_armor_name {
                    maybe_armor = Some(armor);
                }
            }
        }

        if maybe_armor.is_none() {
            maybe_armor = self.armor.unequipped_artifact.get_mut(artifact_armor_name);
        }

        let armor_mut = maybe_armor.ok_or(CharacterMutationError::ArtifactError(
            ArtifactError::NotFound,
        ))?;
        if armor_mut.1.is_some() {
            Err(CharacterMutationError::EssenceError(
                EssenceError::AlreadyAttuned,
            ))
        } else {
            let attunement_cost = match armor_mut.0.base_armor.weight_class {
                ArmorWeightClass::Light => 4,
                ArmorWeightClass::Medium => 5,
                ArmorWeightClass::Heavy => 6,
            };

            if attunement_cost
                > self.essence.motes.peripheral_available + self.essence.motes.peripheral_spent
            {
                return Err(CharacterMutationError::EssenceError(
                    EssenceError::InsufficientMotes,
                ));
            }
            let (peripheral_committed, personal_committed) = if first == MotePoolName::Peripheral {
                let peripheral_committed = self.essence.motes.peripheral_available.min(5);
                let personal_committed = 5 - peripheral_committed;
                (peripheral_committed, personal_committed)
            } else {
                let personal_committed = self.essence.motes.personal_available.min(5);
                let peripheral_committed = 5 - personal_committed;
                (peripheral_committed, personal_committed)
            };
            self.essence.motes.peripheral_available -= peripheral_committed;
            self.essence.motes.personal_available -= personal_committed;
            armor_mut.1 = Some(personal_committed);
            Ok(self)
        }
    }

    pub fn attune_wonder(
        &mut self,
        wonder_name: &str,
        first: MotePoolName,
    ) -> Result<&mut Self, CharacterMutationError> {
        let (wonder, attunement) =
            self.wonders
                .0
                .get_mut(wonder_name)
                .ok_or(CharacterMutationError::ArtifactError(
                    ArtifactError::NotFound,
                ))?;
        if attunement.is_some() {
            return Err(CharacterMutationError::EssenceError(
                EssenceError::AlreadyAttuned,
            ));
        }

        let attunement_cost =
            wonder
                .attunement_cost
                .ok_or(CharacterMutationError::EssenceError(
                    EssenceError::NoAttunementCost,
                ))?;
        if attunement_cost
            > self.essence.motes.peripheral_available + self.essence.motes.peripheral_spent
        {
            return Err(CharacterMutationError::EssenceError(
                EssenceError::InsufficientMotes,
            ));
        }
        let (peripheral_committed, personal_committed) = if first == MotePoolName::Peripheral {
            let peripheral_committed = self.essence.motes.peripheral_available.min(5);
            let personal_committed = 5 - peripheral_committed;
            (peripheral_committed, personal_committed)
        } else {
            let personal_committed = self.essence.motes.personal_available.min(5);
            let peripheral_committed = 5 - personal_committed;
            (peripheral_committed, personal_committed)
        };
        self.essence.motes.peripheral_available -= peripheral_committed;
        self.essence.motes.personal_available -= personal_committed;
        *attunement = Some(personal_committed);
        Ok(self)
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
        details: &'source SolarCharmDetails,
        ability_dots: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        let essence_rating = self.essence.rating;
        match &mut self.exalt_type {
            ExaltType::Solar(solar) => {
                solar.add_solar_charm(name, details, ability_dots, essence_rating.get())?;
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
        style_name: &str,
        name: &'source str,
        martial_arts_charm: &'source MartialArtsCharmDetails,
    ) -> Result<&mut Self, CharacterMutationError> {
        let (&style_name, style_details) =
            self.martial_arts_styles.get_key_value(style_name).ok_or(
                CharacterMutationError::MartialArtsError(MartialArtsError::StyleNotFound),
            )?;
        let required_ability = martial_arts_charm.ability_required;
        let actual_ability = style_details.ability().dots();

        if required_ability.get() > actual_ability {
            return Err(CharacterMutationError::CharmError(
                CharmError::PrerequisitesNotMet,
            ));
        }

        let required_essence = martial_arts_charm.essence_required;
        let actual_essence = self.essence.rating;
        if actual_essence < required_essence {
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
            .charms_required
            .iter()
            .map(|prerequisite_charm_name| prerequisite_charm_name.as_str())
            .collect::<HashSet<&str>>();

        for known_charm_id in style_details
            .charms(style_name)
            .map(|known_charm| known_charm.name())
        {
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
                .get_mut(style_name)
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
                    if known_charm.ability_required.get() > actual_ability {
                        ids_to_remove.insert(*known_charm_name);
                    }

                    if known_charm.essence_required > actual_essence && !is_martial_arts_supernal {
                        ids_to_remove.insert(*known_charm_name);
                    }

                    for prereq_charm_name in known_charm.charms_required.iter() {
                        if ids_to_remove.contains(prereq_charm_name.as_str()) {
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
