mod armor;
pub(crate) mod martial_arts;
mod mortal_memo;
mod weapons;
mod wonders;
use std::collections::{hash_map::Entry, HashMap};
pub(crate) use weapons::{
    MortalEquippedWeapons, MortalHands, MortalUnequippedWeapons, MortalWeapons,
};

pub(crate) use armor::MortalArmor;
pub(crate) use mortal_memo::MortalMemo;
pub(crate) use wonders::MortalWonders;

use crate::{
    abilities::AbilityRating,
    armor::armor_item::{
        artifact::{ArtifactArmorId, ArtifactArmorView, ArtifactError},
        mundane::MundaneArmor,
        ArmorId, ArmorItem, BaseArmorId,
    },
    artifact::wonders::{OwnedWonder, Wonder, WonderId},
    hearthstones::{HearthstoneId, UnslottedHearthstone},
    martial_arts::{MartialArtsError, MartialArtsStyle, MartialArtsStyleId},
    merits::merit::MeritError,
    sorcery::{
        circles::terrestrial::{sorcerer::TerrestrialCircleSorcerer, AddTerrestrialSorceryView},
        SorceryArchetypeId, SorceryArchetypeMerit, SorceryArchetypeMeritId, SorceryError, spell::{SpellId, SpellMutation},
    },
    weapons::{
        weapon::{
            artifact::ArtifactWeaponView,
            mundane::{HandlessMundaneWeapon, MundaneWeapon},
            ArtifactWeaponId, BaseWeaponId, EquipHand, Equipped, Weapon, WeaponId,
        },
        WeaponError,
    },
    CharacterMutationError, charms::CharmError,
};

use self::martial_arts::MortalMartialArtist;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct Mortal<'source> {
    pub armor: MortalArmor<'source>,
    pub martial_arts_styles: HashMap<MartialArtsStyleId, MortalMartialArtist<'source>>,
    pub sorcery: Option<TerrestrialCircleSorcerer<'source>>,
    pub weapons: MortalWeapons<'source>,
    pub wonders: MortalWonders<'source>,
    pub exalted_healing: bool,
}

impl<'source> Mortal<'source> {
    pub fn as_memo(&self) -> MortalMemo {
        MortalMemo::new(
            self.armor.as_memo(),
            self.martial_arts_styles
                .iter()
                .map(|(k, v)| (*k, v.as_memo()))
                .collect(),
            self.sorcery.as_ref().map(|sorcery| sorcery.as_memo()),
            self.weapons.as_memo(),
            self.wonders.as_memo(),
            self.exalted_healing,
        )
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
        self.martial_arts_styles
            .insert(id, MortalMartialArtist::new(style, AbilityRating::Zero));
        Ok(self)
    }

    pub(crate) fn check_remove_martial_arts_style(
        &self,
        id: MartialArtsStyleId,
    ) -> Result<(), CharacterMutationError> {
        if !self.martial_arts_styles.contains_key(&id) {
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

    pub(crate) fn set_martial_arts_dots(
        &mut self,
        id: MartialArtsStyleId,
        dots: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        if let Some(style) = self.martial_arts_styles.get_mut(&id) {
            // Mortals have no charms to lose if dots are zero
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
        add_terrestrial: AddTerrestrialSorceryView<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        if self.sorcery.is_some() {
            return Err(CharacterMutationError::SorceryError(
                SorceryError::CircleSequence,
            ));
        }

        self.sorcery = Some(TerrestrialCircleSorcerer::new(
            add_terrestrial.archetype_id,
            add_terrestrial.archetype,
            add_terrestrial.shaping_ritual_id,
            add_terrestrial.shaping_ritual,
            add_terrestrial.control_spell_id,
            add_terrestrial.control_spell,
        )?);

        Ok(self)
    }

    pub fn remove_terrestrial_sorcery(&mut self) -> Result<&mut Self, CharacterMutationError> {
        if self.sorcery.is_none() {
            Err(CharacterMutationError::SorceryError(
                SorceryError::CircleSequence,
            ))
        } else {
            self.sorcery = None;
            Ok(self)
        }
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

    pub fn add_mundane_weapon(
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
        self.weapons
            .unequipped
            .artifact
            .remove(&artifact_weapon_id)
            .ok_or(CharacterMutationError::WeaponError(WeaponError::NotFound))?;
        Ok(self)
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

    pub fn wonders_iter(&self) -> impl Iterator<Item = WonderId> + '_ {
        self.wonders.iter()
    }

    pub fn get_wonder(&self, wonder_id: WonderId) -> Option<OwnedWonder<'source>> {
        self.wonders.get(wonder_id)
    }

    pub fn add_wonder(
        &mut self,
        wonder_id: WonderId,
        wonder: &'source Wonder,
    ) -> Result<&mut Self, CharacterMutationError> {
        if let Entry::Vacant(e) = self.wonders.0.entry(wonder_id) {
            e.insert(wonder.0.as_ref());
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

    pub fn add_sorcery_archetype_merit(
        &mut self,
        sorcery_archetype_id: SorceryArchetypeId,
        sorcery_archetype_merit_id: SorceryArchetypeMeritId,
        sorcery_archetype_merit: &'source SorceryArchetypeMerit,
    ) -> Result<&mut Self, CharacterMutationError> {
        if let Some(terrestrial) = &mut self.sorcery {
            if terrestrial.archetype_id != sorcery_archetype_id {
                Err(CharacterMutationError::SorceryError(
                    SorceryError::MissingArchetype,
                ))
            } else if let Entry::Vacant(e) = terrestrial
                .archetype_merits
                .entry(sorcery_archetype_merit_id)
            {
                e.insert(sorcery_archetype_merit);
                Ok(self)
            } else {
                Err(CharacterMutationError::MeritError(
                    MeritError::DuplicateMerit,
                ))
            }
        } else {
            Err(CharacterMutationError::SorceryError(
                SorceryError::MissingArchetype,
            ))
        }
    }

    pub fn remove_sorcery_archetype_merit(
        &mut self,
        sorcery_archetype_merit_id: SorceryArchetypeMeritId,
    ) -> Result<&mut Self, CharacterMutationError> {
        if let Some(terrestrial) = &mut self.sorcery {
            if terrestrial
                .archetype_merits
                .remove(&sorcery_archetype_merit_id)
                .is_none()
            {
                Err(CharacterMutationError::MeritError(MeritError::NotFound))
            } else {
                Ok(self)
            }
        } else {
            Err(CharacterMutationError::MeritError(MeritError::NotFound))
        }
    }

    pub fn add_spell(
        &mut self,
        spell_id: SpellId,
        spell: &'source SpellMutation,
    ) -> Result<&mut Self, CharacterMutationError> {
        if let Some(terrestrial) = &mut self.sorcery {
            match spell {
                SpellMutation::Terrestrial(terrestrial_spell) => {
                    if terrestrial.control_spell_id == spell_id || terrestrial.other_spells.contains_key(&spell_id) {
                        Err(CharacterMutationError::CharmError(CharmError::DuplicateCharm))
                    } else {
                        terrestrial.other_spells.insert(spell_id, terrestrial_spell);
                        Ok(self)
                    }
                }
                _ => Err(CharacterMutationError::CharmError(CharmError::PrerequisitesNotMet))
            }
        } else {
            Err(CharacterMutationError::CharmError(CharmError::PrerequisitesNotMet))
        }
    }

    pub fn remove_spell(&mut self, spell_id: SpellId) -> Result<&mut Self, CharacterMutationError> {
        if let Some(terrestrial) = &mut self.sorcery {
            if terrestrial.other_spells.remove(&spell_id).is_some() {
                Ok(self)
            } else if terrestrial.control_spell_id == spell_id {
                Err(CharacterMutationError::SorceryError(SorceryError::RemoveControlSpell))
            } else {
                Err(CharacterMutationError::CharmError(CharmError::NotFound))
            }
        } else {
            Err(CharacterMutationError::CharmError(CharmError::PrerequisitesNotMet))
        }
    }
}
