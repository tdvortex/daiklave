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
    martial_arts::{
        AddMartialArtsStyleError, MartialArtsStyle, MartialArtsStyleId,
        RemoveMartialArtsStyleError, SetMartialArtsDotsError,
    },
    sorcery::{
        circles::terrestrial::sorcerer::TerrestrialCircleSorcerer, ShapingRitual, ShapingRitualId,
        SorceryArchetype, SorceryArchetypeId, SpellId, TerrestrialSpell,
    },
    weapons::{
        weapon::{
            artifact::ArtifactWeaponView,
            mundane::{HandlessMundaneWeapon, MundaneWeapon},
            ArtifactWeaponId, BaseWeaponId, EquipHand, Equipped, Weapon, WeaponId,
        },
        WeaponError,
    },
    CharacterMutationError,
};

use self::martial_arts::MortalMartialArtist;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct Mortal<'source> {
    pub armor: MortalArmor<'source>,
    pub martial_arts_styles: HashMap<MartialArtsStyleId, MortalMartialArtist<'source>>,
    pub sorcery: Option<TerrestrialCircleSorcerer<'source>>,
    pub weapons: MortalWeapons<'source>,
    pub wonders: MortalWonders<'source>,
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
        )
    }

    pub(crate) fn check_add_martial_arts_style(
        &self,
        id: MartialArtsStyleId,
        _style: &MartialArtsStyle,
    ) -> Result<(), CharacterMutationError> {
        if self.martial_arts_styles.contains_key(&id) {
            Err(CharacterMutationError::AddMartialArtsStyleError(
                AddMartialArtsStyleError::DuplicateStyle,
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
            Err(CharacterMutationError::RemoveMartialArtsStyleError(
                RemoveMartialArtsStyleError::NotFound,
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
        _dots: u8,
    ) -> Result<(), CharacterMutationError> {
        if self.martial_arts_styles.contains_key(&id) {
            Ok(())
        } else {
            Err(CharacterMutationError::SetMartialArtsDotsError(
                SetMartialArtsDotsError::NotFound,
            ))
        }
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
            Err(CharacterMutationError::SetMartialArtsDotsError(
                SetMartialArtsDotsError::NotFound,
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
        self.sorcery = Some(TerrestrialCircleSorcerer::new(
            archetype_id,
            archetype,
            shaping_ritual_id,
            shaping_ritual,
            control_spell_id,
            control_spell,
        )?);

        Ok(self)
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
}
