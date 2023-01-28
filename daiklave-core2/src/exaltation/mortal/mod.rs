mod armor;
pub(crate) mod martial_arts;
mod mortal_memo;
mod weapons;
mod wonders;
use std::{
    collections::{hash_map::Entry, HashMap},
    num::NonZeroU8,
};
pub(crate) use weapons::{
    MortalEquippedWeapons, MortalHands, MortalUnequippedWeapons, MortalWeapons,
};

pub(crate) use armor::MortalArmor;
pub(crate) use mortal_memo::MortalMemo;
pub(crate) use wonders::MortalWonders;

use crate::{
    abilities::AbilityRating,
    armor::armor_item::{
        artifact::{ArtifactArmorView, ArtifactError},
        mundane::MundaneArmor,
        ArmorItem, ArmorName,
    },
    artifact::wonders::{OwnedWonder, Wonder},
    charms::CharmError,
    hearthstones::UnslottedHearthstone,
    martial_arts::{style::MartialArtsStyle, MartialArtsError},
    merits::merit::MeritError,
    sorcery::{
        circles::terrestrial::{sorcerer::TerrestrialCircleSorcerer, AddTerrestrialSorceryView},
        spell::SpellMutation,
        SorceryArchetypeMerit, SorceryArchetypeMeritId, SorceryError,
    },
    weapons::{
        weapon::{
            artifact::ArtifactWeaponView,
            mundane::{HandlessMundaneWeapon, MundaneWeapon},
            EquipHand, Equipped, Weapon, WeaponName,
        },
        WeaponError,
    },
    CharacterMutationError,
};

use self::martial_arts::MortalMartialArtist;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct Mortal<'source> {
    pub armor: MortalArmor<'source>,
    pub martial_arts_styles: HashMap<&'source str, MortalMartialArtist<'source>>,
    pub sorcery: Option<TerrestrialCircleSorcerer<'source>>,
    pub weapons: MortalWeapons<'source>,
    pub wonders: MortalWonders<'source>,
    pub exalted_healing: bool,
}

impl<'view, 'source> Mortal<'source> {
    pub fn as_memo(&self) -> MortalMemo {
        MortalMemo {
            armor: self.armor.as_memo(),
            martial_arts_styles: self
                .martial_arts_styles
                .iter()
                .map(|(k, v)| ((*k).to_owned(), v.as_memo()))
                .collect(),
            sorcery: self.sorcery.as_ref().map(|sorcery| sorcery.as_memo()),
            weapons: self.weapons.as_memo(),
            wonders: self.wonders.as_memo(),
            exalted_healing: self.exalted_healing,
        }
    }

    pub(crate) fn add_martial_arts_style(
        &mut self,
        name: &'source str,
        style: &'source MartialArtsStyle,
    ) -> Result<&mut Self, CharacterMutationError> {
        if let Entry::Vacant(e) = self.martial_arts_styles.entry(name) {
            e.insert(MortalMartialArtist {
                style,
                ability: AbilityRating::Zero,
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
            .remove(name)
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
        if let Some(style) = self.martial_arts_styles.get_mut(name) {
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

        self.sorcery = Some(TerrestrialCircleSorcerer {
            archetype_name: add_terrestrial.archetype_name,
            archetype: add_terrestrial.archetype,
            archetype_merits: HashMap::new(),
            shaping_ritual_name: add_terrestrial.shaping_ritual_name,
            shaping_ritual: add_terrestrial.shaping_ritual,
            control_spell_name: add_terrestrial.control_spell_name,
            control_spell: add_terrestrial.control_spell,
            other_spells: HashMap::new(),
        });

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

    pub fn add_mundane_weapon(
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
        weapon_name: WeaponName<'_>,
        equipped: Equipped,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.weapons.unequip_weapon(weapon_name, equipped)?;
        Ok(self)
    }

    pub fn add_artifact_weapon(
        &mut self,
        name: &'source str,
        weapon: ArtifactWeaponView<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.weapons.add_artifact_weapon(name, weapon)?;
        Ok(self)
    }

    pub fn remove_artifact_weapon(
        &mut self,
        name: &str,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.weapons
            .unequipped
            .artifact
            .remove(name)
            .ok_or(CharacterMutationError::WeaponError(WeaponError::NotFound))?;
        Ok(self)
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

    pub fn wonders_iter(&self) -> impl Iterator<Item = &'source str> + '_ {
        self.wonders.iter()
    }

    pub fn get_wonder(&self, name: &str) -> Option<OwnedWonder<'source>> {
        self.wonders.get(name)
    }

    pub fn add_wonder(
        &mut self,
        name: &'source str,
        wonder: &'source Wonder,
    ) -> Result<&mut Self, CharacterMutationError> {
        if let Entry::Vacant(e) = self.wonders.0.entry(name) {
            e.insert(wonder.0.as_ref());
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

    pub fn add_sorcery_archetype_merit(
        &mut self,
        sorcery_archetype_name: &str,
        sorcery_archetype_merit_id: SorceryArchetypeMeritId,
        sorcery_archetype_merit: &'source SorceryArchetypeMerit,
    ) -> Result<&mut Self, CharacterMutationError> {
        if let Some(terrestrial) = &mut self.sorcery {
            if terrestrial.archetype_name != sorcery_archetype_name {
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
        name: &'source str,
        spell: &'source SpellMutation,
    ) -> Result<&mut Self, CharacterMutationError> {
        if let Some(terrestrial) = &mut self.sorcery {
            match spell {
                SpellMutation::Terrestrial(terrestrial_spell) => {
                    if terrestrial.control_spell_name == name
                        || terrestrial.other_spells.contains_key(name)
                    {
                        Err(CharacterMutationError::CharmError(
                            CharmError::DuplicateCharm,
                        ))
                    } else {
                        terrestrial.other_spells.insert(name, terrestrial_spell);
                        Ok(self)
                    }
                }
                _ => Err(CharacterMutationError::CharmError(
                    CharmError::PrerequisitesNotMet,
                )),
            }
        } else {
            Err(CharacterMutationError::CharmError(
                CharmError::PrerequisitesNotMet,
            ))
        }
    }

    pub fn remove_spell(&mut self, name: &str) -> Result<&mut Self, CharacterMutationError> {
        if let Some(terrestrial) = &mut self.sorcery {
            if terrestrial.other_spells.remove(name).is_some() {
                Ok(self)
            } else if terrestrial.control_spell_name == name {
                Err(CharacterMutationError::SorceryError(
                    SorceryError::RemoveControlSpell,
                ))
            } else {
                Err(CharacterMutationError::CharmError(CharmError::NotFound))
            }
        } else {
            Err(CharacterMutationError::CharmError(
                CharmError::PrerequisitesNotMet,
            ))
        }
    }
}
