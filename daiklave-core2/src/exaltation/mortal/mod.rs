pub(crate) mod martial_arts;
mod mortal_memo;
mod weapons;
use std::collections::HashMap;
pub(crate) use weapons::{
    MortalEquippedWeapons, MortalHands, MortalUnequippedWeapons, MortalWeapons,
};

pub(crate) use mortal_memo::MortalMemo;

use crate::{
    abilities::AbilityRating,
    martial_arts::{
        AddMartialArtsStyleError, MartialArtsStyle, MartialArtsStyleId,
        RemoveMartialArtsStyleError, SetMartialArtsDotsError,
    },
    sorcery::{
        circles::terrestrial::sorcerer::TerrestrialCircleSorcerer, ShapingRitual, ShapingRitualId,
        SorceryArchetype, SorceryArchetypeId, SpellId, TerrestrialSpell,
    },
    weapons::weapon::{mundane::MundaneWeaponMemo, BaseWeaponId, Weapon, WeaponId},
    CharacterMutationError,
};

use self::martial_arts::MortalMartialArtist;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct Mortal<'source> {
    pub martial_arts_styles: HashMap<MartialArtsStyleId, MortalMartialArtist<'source>>,
    pub sorcery: Option<TerrestrialCircleSorcerer<'source>>,
    pub weapons: MortalWeapons<'source>,
}

impl<'source> Mortal<'source> {
    pub fn as_memo(&self) -> MortalMemo {
        MortalMemo::new(
            self.martial_arts_styles
                .iter()
                .map(|(k, v)| (*k, v.as_memo()))
                .collect(),
            self.sorcery.as_ref().map(|sorcery| sorcery.as_memo()),
            self.weapons.as_memo(),
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

    pub fn get_weapon(&self, weapon_id: WeaponId) -> Option<Weapon<'source>> {
        if matches!(weapon_id, WeaponId::Unarmed) {
            Some(crate::weapons::weapon::mundane::unarmed())
        } else {
            self.weapons.get_weapon(weapon_id)
        }
    }

    pub fn iter_weapons(&self) -> impl Iterator<Item = WeaponId> + '_ {
        self.weapons.iter()
    }

    pub fn add_mundane_weapon(
        &mut self,
        weapon_id: BaseWeaponId,
        weapon: &'source MundaneWeaponMemo,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.weapons.add_mundane_weapon(weapon_id, weapon)?;
        Ok(self)
    }
}
