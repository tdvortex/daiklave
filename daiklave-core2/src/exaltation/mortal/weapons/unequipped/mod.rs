mod memo;
use std::collections::{hash_map::Entry, HashMap};

pub(crate) use memo::MortalUnequippedWeaponsMemo;

use crate::{
    exaltation::exalt::ExaltUnequippedWeapons,
    hearthstones::{HearthstoneError, HearthstoneId, SlottedHearthstone, UnslottedHearthstone},
    weapons::{
        weapon::{
            artifact::{ArtifactWeaponView, NonnaturalArtifactWeaponNoAttunement},
            mundane::{MundaneWeaponView, NonnaturalMundaneWeapon},
            ArtifactWeaponId, BaseWeaponId, Equipped, Weapon, WeaponId, WeaponType,
        },
        WeaponError,
    },
    CharacterMutationError,
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct MortalUnequippedWeapons<'source> {
    pub mundane: HashMap<BaseWeaponId, (NonnaturalMundaneWeapon<'source>, u8)>,
    pub artifact: HashMap<ArtifactWeaponId, NonnaturalArtifactWeaponNoAttunement<'source>>,
}

impl<'source> From<ExaltUnequippedWeapons<'source>> for MortalUnequippedWeapons<'source> {
    fn from(exalt: ExaltUnequippedWeapons<'source>) -> Self {
        Self {
            mundane: exalt.mundane,
            artifact: exalt.artifact.into_iter().map(|(k, v)| (k, v.0)).collect(),
        }
    }
}

impl<'view, 'source> MortalUnequippedWeapons<'source> {
    pub fn as_memo(&self) -> MortalUnequippedWeaponsMemo {
        MortalUnequippedWeaponsMemo {
            mundane: self
                .mundane
                .iter()
                .map(|(k, (v, count))| (*k, (v.as_memo(), *count)))
                .collect(),
            artifact: self
                .artifact
                .iter()
                .map(|(k, v)| (*k, v.as_memo()))
                .collect(),
        }
    }

    pub fn get_weapon(&'view self, weapon_id: WeaponId) -> Option<Weapon<'source>> {
        match weapon_id {
            WeaponId::Unarmed => Some(crate::weapons::weapon::mundane::unarmed()),
            WeaponId::Mundane(target_id) => match self.mundane.get(&target_id)? {
                (NonnaturalMundaneWeapon::Worn(worn_weapon), count) => {
                    Some(Weapon(WeaponType::Mundane(
                        target_id,
                        MundaneWeaponView::Worn(worn_weapon.clone(), false),
                        *count,
                    )))
                }
                (NonnaturalMundaneWeapon::OneHanded(one), count) => {
                    Some(Weapon(WeaponType::Mundane(
                        target_id,
                        MundaneWeaponView::OneHanded(one.clone(), None),
                        *count,
                    )))
                }
                (NonnaturalMundaneWeapon::TwoHanded(two), count) => {
                    Some(Weapon(WeaponType::Mundane(
                        target_id,
                        MundaneWeaponView::TwoHanded(two.clone(), false),
                        *count,
                    )))
                }
            },
            WeaponId::Artifact(target_id) => match self.artifact.get(&target_id)? {
                NonnaturalArtifactWeaponNoAttunement::Worn(worn) => {
                    Some(Weapon(WeaponType::Artifact(
                        target_id,
                        ArtifactWeaponView::Worn(worn.clone(), false),
                        None,
                    )))
                }
                NonnaturalArtifactWeaponNoAttunement::OneHanded(one) => {
                    Some(Weapon(WeaponType::Artifact(
                        target_id,
                        ArtifactWeaponView::OneHanded(one.clone(), None),
                        None,
                    )))
                }
                NonnaturalArtifactWeaponNoAttunement::TwoHanded(two) => {
                    Some(Weapon(WeaponType::Artifact(
                        target_id,
                        ArtifactWeaponView::TwoHanded(two.clone(), false),
                        None,
                    )))
                }
            },
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (WeaponId, Option<Equipped>)> + '_ {
        self.mundane
            .iter()
            .map(|(base_id, _)| (WeaponId::Mundane(*base_id), None))
            .chain(
                self.artifact
                    .iter()
                    .map(|(artifact_id, _)| (WeaponId::Artifact(*artifact_id), None)),
            )
    }

    pub fn stow_mundane(
        &mut self,
        weapon_id: BaseWeaponId,
        weapon: NonnaturalMundaneWeapon<'source>,
    ) {
        let count_ptr = &mut self.mundane.entry(weapon_id).or_insert((weapon, 0)).1;
        if *count_ptr < u8::MAX {
            *count_ptr += 1;
        }
    }

    pub fn unstow_mundane(
        &mut self,
        weapon_id: BaseWeaponId,
    ) -> Option<NonnaturalMundaneWeapon<'source>> {
        let current_count = self.mundane.get(&weapon_id)?.1;

        match current_count {
            0 => {
                // This shouldn't happen, but if it does handle it by
                // removing the problem entry
                self.mundane.remove(&weapon_id);
                None
            }
            1 => self.mundane.remove(&weapon_id).map(|(weapon, _)| weapon),
            _ => self.mundane.get_mut(&weapon_id).map(|(weapon, count)| {
                *count -= 1;
                weapon.clone()
            }),
        }
    }

    pub fn stow_artifact(
        &mut self,
        weapon_id: ArtifactWeaponId,
        weapon: NonnaturalArtifactWeaponNoAttunement<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        if let Entry::Vacant(e) = self.artifact.entry(weapon_id) {
            e.insert(weapon);
            Ok(self)
        } else {
            Err(CharacterMutationError::WeaponError(
                WeaponError::NamedArtifactsUnique,
            ))
        }
    }

    pub fn unstow_artifact(
        &mut self,
        weapon_id: ArtifactWeaponId,
    ) -> Option<NonnaturalArtifactWeaponNoAttunement<'source>> {
        self.artifact.remove(&weapon_id)
    }

    pub fn slot_hearthstone(
        &mut self,
        artifact_weapon_id: ArtifactWeaponId,
        hearthstone_id: HearthstoneId,
        unslotted: UnslottedHearthstone<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        *self
            .artifact
            .get_mut(&artifact_weapon_id)
            .ok_or(CharacterMutationError::WeaponError(WeaponError::NotFound))?
            .hearthstone_slots
            .iter_mut()
            .find(|maybe_hearthstone| maybe_hearthstone.is_none())
            .ok_or(CharacterMutationError::HearthstoneError(
                HearthstoneError::AllSlotsFilled,
            ))? = Some(SlottedHearthstone {
            hearthstone_id,
            details: unslotted.details,
            origin: unslotted.origin,
        });
        Ok(self)
    }

    pub fn unslot_hearthstone(
        &mut self,
        artifact_weapon_id: ArtifactWeaponId,
        hearthstone_id: HearthstoneId,
    ) -> Result<UnslottedHearthstone<'source>, CharacterMutationError> {
        let SlottedHearthstone {
            hearthstone_id,
            details,
            origin,
        } = self
            .artifact
            .get_mut(&artifact_weapon_id)
            .ok_or(CharacterMutationError::WeaponError(WeaponError::NotFound))?
            .hearthstone_slots
            .iter_mut()
            .find_map(|maybe_hearthstone| {
                if maybe_hearthstone
                    .as_ref()
                    .map_or(false, |hearthstone| hearthstone.id() == hearthstone_id)
                {
                    maybe_hearthstone.take()
                } else {
                    None
                }
            })
            .ok_or(CharacterMutationError::HearthstoneError(
                HearthstoneError::NotSlotted,
            ))?;

        Ok(UnslottedHearthstone { details, origin })
    }
}
