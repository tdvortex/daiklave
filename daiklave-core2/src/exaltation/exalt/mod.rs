/// Structs and methods related to the Essence rating and mote pools for a
/// character.
pub mod essence;

/// Structs and methods related to various Exalt subtypes (Solar, Lunar, etc).
pub mod exalt_type;

mod armor;
mod limit;
pub(crate) mod martial_arts;
mod memo;
mod sorcery;
mod weapons;
mod wonders;

pub(crate) use armor::ExaltArmor;
pub(crate) use limit::{Limit, LimitMemo};
pub(crate) use memo::ExaltMemo;
pub(crate) use sorcery::ExaltSorcery;
pub(crate) use weapons::{ExaltEquippedWeapons, ExaltHands, ExaltUnequippedWeapons, ExaltWeapons};
pub(crate) use wonders::ExaltWonders;

use std::collections::{hash_map::Entry, HashMap};

use crate::{
    abilities::{AbilityRating, AbilityError},
    armor::{
        armor_item::{
            artifact::{ArtifactArmorId, ArtifactArmorView, ArtifactError},
            mundane::MundaneArmor,
            ArmorId, ArmorItem, BaseArmorId,
        },
        ArmorError,
    },
    artifact::{
        wonders::{OwnedWonder, Wonder, WonderId},
        ArtifactId,
    },
    exaltation::sorcery::ExaltationSorcery,
    hearthstones::{HearthstoneId, UnslottedHearthstone},
    martial_arts::{MartialArtsCharmId, MartialArtsStyle, MartialArtsStyleId, MartialArtsError,
    },
    sorcery::{
        ShapingRitual, ShapingRitualId, Sorcery, SorceryArchetype, SorceryArchetypeId, SpellId,
        TerrestrialSpell,
    },
    weapons::{
        weapon::{
            artifact::{
                ArtifactWeaponView, HandlessArtifactWeapon, HandlessArtifactWeaponNoAttunement,
                NonnaturalArtifactWeapon,
            },
            mundane::{HandlessMundaneWeapon, MundaneWeapon},
            ArtifactWeaponId, BaseWeaponId, EquipHand, Equipped, Weapon, WeaponId,
        },
        WeaponError,
    },
    CharacterMutationError,
};

use self::{
    essence::{
        Essence, EssenceError, EssenceState, MoteCommitment, MoteCommitmentId, MotePoolName,
        OtherMoteCommitmentId,
    },
    exalt_type::{solar::Solar, ExaltType},
    martial_arts::ExaltMartialArtist,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Exalt<'source> {
    armor: ExaltArmor<'source>,
    essence: EssenceState<'source>,
    martial_arts_styles: HashMap<MartialArtsStyleId, ExaltMartialArtist<'source>>,
    exalt_type: ExaltType<'source>,
    weapons: ExaltWeapons<'source>,
    wonders: ExaltWonders<'source>,
}

impl<'view, 'source> Exalt<'source> {
    pub fn new(
        armor: ExaltArmor<'source>,
        essence: EssenceState<'source>,
        martial_arts_styles: HashMap<MartialArtsStyleId, ExaltMartialArtist<'source>>,
        exalt_type: ExaltType<'source>,
        weapons: ExaltWeapons<'source>,
        wonders: ExaltWonders<'source>,
    ) -> Self {
        Self {
            essence,
            martial_arts_styles,
            exalt_type,
            weapons,
            armor,
            wonders,
        }
    }

    pub fn as_memo(&self) -> ExaltMemo {
        ExaltMemo::new(
            self.armor.as_memo(),
            self.essence.as_memo(),
            self.martial_arts_styles
                .iter()
                .map(|(k, v)| (*k, v.as_memo()))
                .collect(),
            self.exalt_type.as_memo(),
            self.weapons.as_memo(),
            self.wonders.as_memo(),
        )
    }

    pub fn exalt_type(&self) -> &ExaltType<'source> {
        &self.exalt_type
    }

    pub fn essence(&'view self) -> Essence<'view, 'source> {
        Essence(self)
    }

    pub fn martial_arts_styles(&self) -> &HashMap<MartialArtsStyleId, ExaltMartialArtist<'source>> {
        &self.martial_arts_styles
    }

    pub fn martial_arts_styles_mut(
        &mut self,
    ) -> &mut HashMap<MartialArtsStyleId, ExaltMartialArtist<'source>> {
        &mut self.martial_arts_styles
    }

    pub fn weapons_mut(&mut self) -> &mut ExaltWeapons<'source> {
        &mut self.weapons
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
            self.weapons.get_weapon(weapon_id, equipped)
        }
    }

    pub fn iter_weapons(&self) -> impl Iterator<Item = (WeaponId, Option<Equipped>)> + '_ {
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

    pub fn check_commit_motes(
        &self,
        _id: &OtherMoteCommitmentId,
        _name: &str,
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

    pub fn commit_motes(
        &mut self,
        id: &OtherMoteCommitmentId,
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

        if let Entry::Vacant(e) = self.essence.motes.commitments_mut().entry(*id) {
            e.insert(MoteCommitment {
                name,
                peripheral: peripheral_committed,
                personal: personal_committed,
            });
            Ok(self)
        } else {
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

    pub fn check_uncommit_motes(
        &self,
        id: &MoteCommitmentId,
    ) -> Result<(), CharacterMutationError> {
        if match id {
            MoteCommitmentId::AttunedArtifact(artifact_id) => match artifact_id {
                ArtifactId::Weapon(artifact_weapon_id) => self
                    .weapons
                    .iter()
                    .find_map(|(weapon_id, equipped)| {
                        if weapon_id == WeaponId::Artifact(*artifact_weapon_id) {
                            self.weapons.get_weapon(weapon_id, equipped)
                        } else {
                            None
                        }
                    })
                    .ok_or(CharacterMutationError::WeaponError(WeaponError::NotFound))?
                    .is_attuned(),
                ArtifactId::Armor(artifact_armor_id) => self
                    .armor
                    .get(ArmorId::Artifact(*artifact_armor_id))
                    .ok_or(CharacterMutationError::ArmorError(ArmorError::NotFound))?
                    .is_attuned(),
                ArtifactId::Wonder(wonder_id) => self
                    .wonders
                    .0
                    .get(wonder_id)
                    .ok_or(CharacterMutationError::ArtifactError(
                        ArtifactError::NotFound,
                    ))?
                    .1
                    .is_some(),
            },
            MoteCommitmentId::Other(other_id) => {
                self.essence.motes.commitments().contains_key(other_id)
            }
        } {
            Ok(())
        } else {
            Err(CharacterMutationError::EssenceError(EssenceError::NotFound))
        }
    }

    pub fn uncommit_motes(
        &mut self,
        id: &MoteCommitmentId,
    ) -> Result<&mut Self, CharacterMutationError> {
        let (peripheral, personal) = match id {
            MoteCommitmentId::AttunedArtifact(artifact_id) => match artifact_id {
                ArtifactId::Weapon(artifact_weapon_id) => {
                    self.weapons.unattune_artifact_weapon(*artifact_weapon_id)?
                }
                ArtifactId::Armor(artifact_armor_id) => {
                    self.armor.unattune_artifact_armor(*artifact_armor_id)?
                }
                ArtifactId::Wonder(wonder_id) => self.wonders.unattune_wonder(*wonder_id)?,
            },
            MoteCommitmentId::Other(other_id) => {
                let commitment = self
                    .essence
                    .motes
                    .commitments
                    .remove(other_id)
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

        if !(1..=5).contains(&rating) {
            return Err(CharacterMutationError::EssenceError(
                EssenceError::InvalidRating,
            ));
        }

        let (new_peripheral, new_personal) = match self.exalt_type {
            ExaltType::Solar(_) => (rating * 7 + 26, rating * 3 + 10),
        };

        let committed_ids = self
            .essence()
            .motes()
            .committed()
            .map(|x| x.0)
            .collect::<Vec<MoteCommitmentId>>();
        for id in committed_ids {
            self.uncommit_motes(&id).unwrap();
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

    pub fn is_solar(&self) -> bool {
        self.exalt_type.is_solar()
    }

    pub fn solar_traits(&self) -> Option<&Solar> {
        self.exalt_type.solar_traits()
    }

    pub(crate) fn check_add_martial_arts_style(
        &self,
        id: MartialArtsStyleId,
        _style: &MartialArtsStyle,
    ) -> Result<(), CharacterMutationError> {
        if self.martial_arts_styles.contains_key(&id) {
            Err(CharacterMutationError::MartialArtsError(
                MartialArtsError::DuplicateStyle,
            ))
        } else {
            Ok(())
        }
    }

    pub(crate) fn add_martial_arts_style(
        &mut self,
        id: MartialArtsStyleId,
        style: &'source MartialArtsStyle,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.check_add_martial_arts_style(id, style)?;
        self.martial_arts_styles.insert(
            id,
            ExaltMartialArtist::new(style, AbilityRating::Zero, HashMap::new()),
        );
        Ok(self)
    }

    pub(crate) fn check_remove_martial_arts_style(
        &self,
        id: MartialArtsStyleId,
    ) -> Result<(), CharacterMutationError> {
        if self.martial_arts_styles.contains_key(&id) {
            Err(CharacterMutationError::MartialArtsError(
                MartialArtsError::StyleNotFound,
            ))
        } else {
            Ok(())
        }
    }

    pub(crate) fn remove_martial_arts_style(
        &mut self,
        id: MartialArtsStyleId,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.check_remove_martial_arts_style(id)?;
        self.martial_arts_styles.remove(&id);
        Ok(self)
    }

    pub(crate) fn check_set_martial_arts_dots(
        &self,
        id: MartialArtsStyleId,
        dots: u8,
    ) -> Result<(), CharacterMutationError> {
        if dots > 5 {
            Err(CharacterMutationError::AbilityError(
                AbilityError::InvalidRating,
            ))
        } else if self.martial_arts_styles.contains_key(&id) {
            Ok(())
        } else {
            Err(CharacterMutationError::MartialArtsError(
                MartialArtsError::StyleNotFound,
            ))
        }
    }

    pub(crate) fn set_martial_arts_dots(
        &mut self,
        id: MartialArtsStyleId,
        dots: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        if dots > 5 {
            Err(CharacterMutationError::AbilityError(
                AbilityError::InvalidRating,
            ))
        } else if let Some(style) = self.martial_arts_styles.get_mut(&id) {
            if dots < style.ability().dots() {
                // May have to remove charms
                let mut prereq_charms_map =
                    HashMap::<MartialArtsCharmId, Vec<MartialArtsCharmId>>::new();
                let mut removal_stack = Vec::<MartialArtsCharmId>::new();

                for (charm_id, charm) in style.charms() {
                    for prereq_charm_id in charm.charms_required() {
                        prereq_charms_map
                            .entry(prereq_charm_id)
                            .or_default()
                            .push(charm_id);
                    }

                    if charm.ability_required() > dots {
                        removal_stack.push(charm_id);
                    }
                }

                while let Some(id_to_remove) = removal_stack.pop() {
                    style.charms_mut().remove(&id_to_remove);
                    if let Some(dependents) = prereq_charms_map.remove(&id_to_remove) {
                        for dependent_id in dependents.iter() {
                            removal_stack.push(*dependent_id);
                        }
                    }
                }
            }
            style.ability_mut().set_dots(dots)?;
            Ok(self)
        } else {
            Err(CharacterMutationError::MartialArtsError(
                MartialArtsError::StyleNotFound,
            ))
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
        match &mut self.exalt_type {
            ExaltType::Solar(solar) => {
                solar.add_terrestrial_sorcery(
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
        match self.exalt_type() {
            ExaltType::Solar(solar) => solar
                .sorcery()
                .map(|sorcerer| Sorcery(ExaltationSorcery::Exalt(ExaltSorcery::Solar(sorcerer)))),
        }
    }

    pub(crate) fn add_mundane_weapon(
        &mut self,
        weapon_id: BaseWeaponId,
        weapon: &'source MundaneWeapon,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.weapons.add_mundane_weapon(weapon_id, weapon)?;
        Ok(self)
    }

    pub fn equip_weapon(
        &mut self,
        weapon_id: WeaponId,
        hand: Option<EquipHand>,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.weapons.equip_weapon(weapon_id, hand)?;
        Ok(self)
    }

    pub fn unequip_weapon(
        &mut self,
        weapon_id: WeaponId,
        equipped: Equipped,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.weapons.unequip_weapon(weapon_id, equipped)?;
        Ok(self)
    }

    pub fn add_artifact_weapon(
        &mut self,
        weapon_id: ArtifactWeaponId,
        weapon: ArtifactWeaponView<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.weapons.add_artifact_weapon(weapon_id, weapon)?;
        Ok(self)
    }

    pub fn remove_artifact_weapon(
        &mut self,
        artifact_weapon_id: ArtifactWeaponId,
    ) -> Result<&mut Self, CharacterMutationError> {
        if let Some(NonnaturalArtifactWeapon(_, attunement)) =
            self.weapons.unequipped.artifact.get(&artifact_weapon_id)
        {
            if let Some(personal) = attunement {
                let peripheral = 5 - (*personal).min(5);
                self.essence.motes.peripheral_mut().uncommit(peripheral)?;
                self.essence.motes.personal_mut().uncommit(*personal)?;
            }

            self.weapons.unequipped.artifact.remove(&artifact_weapon_id);
            Ok(self)
        } else if let Some(HandlessArtifactWeapon(
            HandlessArtifactWeaponNoAttunement::Natural(_),
            attunement,
        )) = self
            .weapons
            .equipped
            .handless_artifact
            .get(&artifact_weapon_id)
        {
            if let Some(personal) = attunement {
                let peripheral = 5 - (*personal).min(5);
                self.essence.motes.peripheral_mut().uncommit(peripheral)?;
                self.essence.motes.personal_mut().uncommit(*personal)?;
            }

            self.weapons
                .equipped
                .handless_artifact
                .remove(&artifact_weapon_id);
            Ok(self)
        } else {
            Err(CharacterMutationError::WeaponError(WeaponError::NotFound))
        }
    }

    pub fn remove_mundane_weapon(
        &mut self,
        weapon_id: BaseWeaponId,
    ) -> Result<&mut Self, CharacterMutationError> {
        if let Some((_, count)) = self.weapons.unequipped.mundane.get(&weapon_id) {
            if *count <= 1 {
                self.weapons.unequipped.mundane.remove(&weapon_id);
            } else {
                self.weapons
                    .unequipped
                    .mundane
                    .get_mut(&weapon_id)
                    .unwrap()
                    .1 -= 1;
            }
            Ok(self)
        } else if let Some(weapon) = self.weapons.equipped.handless_mundane.get(&weapon_id) {
            if matches!(weapon, HandlessMundaneWeapon::Natural(_)) {
                self.weapons.equipped.handless_mundane.remove(&weapon_id);
            }
            Ok(self)
        } else {
            Err(CharacterMutationError::WeaponError(WeaponError::NotFound))
        }
    }

    pub fn worn_armor(&self) -> Option<ArmorItem<'source>> {
        self.armor.worn_armor()
    }

    pub fn armor_iter(&self) -> std::vec::IntoIter<ArmorId> {
        self.armor.iter()
    }

    pub fn get_armor(&self, armor_id: ArmorId) -> Option<ArmorItem<'source>> {
        self.armor.get(armor_id)
    }

    pub fn add_mundane_armor(
        &mut self,
        armor_id: BaseArmorId,
        armor: &'source MundaneArmor,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.armor.add_mundane(armor_id, armor)?;
        Ok(self)
    }

    pub fn remove_mundane_armor(
        &mut self,
        armor_id: BaseArmorId,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.armor.remove_mundane(armor_id)?;
        Ok(self)
    }

    pub fn equip_armor(&mut self, armor_id: ArmorId) -> Result<&mut Self, CharacterMutationError> {
        self.armor.equip(armor_id)?;
        Ok(self)
    }

    pub fn unequip_armor(&mut self) -> Result<&mut Self, CharacterMutationError> {
        self.armor.unequip()?;
        Ok(self)
    }

    pub fn add_artifact_armor(
        &mut self,
        armor_id: ArtifactArmorId,
        armor: ArtifactArmorView<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.armor.add_artifact(armor_id, armor)?;
        Ok(self)
    }

    pub fn remove_artifact_armor(
        &mut self,
        armor_id: ArtifactArmorId,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.armor.remove_artifact(armor_id)?;
        Ok(self)
    }

    pub fn armor_mut(&mut self) -> &mut ExaltArmor<'source> {
        &mut self.armor
    }

    pub fn wonders_iter(&self) -> impl Iterator<Item = WonderId> + '_ {
        self.wonders.iter()
    }

    pub fn get_wonder(&self, wonder_id: WonderId) -> Option<OwnedWonder<'source>> {
        self.wonders.get(wonder_id)
    }

    pub fn wonders_mut(&mut self) -> &mut ExaltWonders<'source> {
        &mut self.wonders
    }

    pub fn add_wonder(
        &mut self,
        wonder_id: WonderId,
        wonder: &'source Wonder,
    ) -> Result<&mut Self, CharacterMutationError> {
        if let Entry::Vacant(e) = self.wonders.0.entry(wonder_id) {
            e.insert((wonder.0.as_ref(), None));
            Ok(self)
        } else {
            Err(CharacterMutationError::ArtifactError(
                ArtifactError::NamedArtifactsUnique,
            ))
        }
    }

    pub fn remove_wonder(
        &mut self,
        wonder_id: WonderId,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.wonders
            .0
            .remove(&wonder_id)
            .ok_or(CharacterMutationError::ArtifactError(
                ArtifactError::NotFound,
            ))?;
        Ok(self)
    }

    pub fn slot_hearthstone_into_weapon(
        &mut self,
        artifact_weapon_id: ArtifactWeaponId,
        hearthstone_id: HearthstoneId,
        unslotted: UnslottedHearthstone<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.weapons
            .slot_hearthstone(artifact_weapon_id, hearthstone_id, unslotted)?;
        Ok(self)
    }

    pub fn slot_hearthstone_into_armor(
        &mut self,
        artifact_armor_id: ArtifactArmorId,
        hearthstone_id: HearthstoneId,
        unslotted: UnslottedHearthstone<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.armor
            .slot_hearthstone(artifact_armor_id, hearthstone_id, unslotted)?;
        Ok(self)
    }

    pub fn slot_hearthstone_into_wonder(
        &mut self,
        wonder_id: WonderId,
        hearthstone_id: HearthstoneId,
        unslotted: UnslottedHearthstone<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.wonders
            .slot_hearthstone(wonder_id, hearthstone_id, unslotted)?;
        Ok(self)
    }

    pub fn unslot_hearthstone_from_weapon(
        &mut self,
        artifact_weapon_id: ArtifactWeaponId,
        hearthstone_id: HearthstoneId,
    ) -> Result<UnslottedHearthstone<'source>, CharacterMutationError> {
        self.weapons
            .unslot_hearthstone(artifact_weapon_id, hearthstone_id)
    }

    pub fn unslot_hearthstone_from_armor(
        &mut self,
        artifact_armor_id: ArtifactArmorId,
        hearthstone_id: HearthstoneId,
    ) -> Result<UnslottedHearthstone<'source>, CharacterMutationError> {
        self.armor
            .unslot_hearthstone(artifact_armor_id, hearthstone_id)
    }

    pub fn unslot_hearthstone_from_wonder(
        &mut self,
        wonder_id: WonderId,
        hearthstone_id: HearthstoneId,
    ) -> Result<UnslottedHearthstone<'source>, CharacterMutationError> {
        self.wonders.unslot_hearthstone(wonder_id, hearthstone_id)
    }

    pub fn attune_artifact(
        &mut self,
        artifact_id: ArtifactId,
        first: MotePoolName,
    ) -> Result<&mut Self, CharacterMutationError> {
        let amount = self.attunement_cost(artifact_id)?;
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

        let outcome = match artifact_id {
            ArtifactId::Weapon(artifact_weapon_id) => self
                .weapons_mut()
                .attune_artifact_weapon(artifact_weapon_id, personal_committed)
                .err(),
            ArtifactId::Armor(artifact_armor_id) => self
                .armor_mut()
                .attune_artifact_armor(artifact_armor_id, personal_committed)
                .err(),
            ArtifactId::Wonder(wonder_id) => self
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

    pub fn attunement_cost(&self, artifact_id: ArtifactId) -> Result<u8, CharacterMutationError> {
        match artifact_id {
            ArtifactId::Weapon(artifact_weapon_id) => {
                if self
                    .weapons
                    .iter()
                    .any(|(weapon_id, _)| weapon_id == WeaponId::Artifact(artifact_weapon_id))
                {
                    Ok(5)
                } else {
                    Err(CharacterMutationError::WeaponError(WeaponError::NotFound))
                }
            }
            ArtifactId::Armor(artifact_armor_id) => self
                .armor
                .get(ArmorId::Artifact(artifact_armor_id))
                .ok_or(CharacterMutationError::ArmorError(ArmorError::NotFound))?
                .attunement_cost()
                .ok_or(CharacterMutationError::EssenceError(
                    EssenceError::NoAttunementCost,
                )),
            ArtifactId::Wonder(wonder_id) => self
                .wonders
                .get(wonder_id)
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
}
