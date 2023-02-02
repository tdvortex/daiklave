mod memo;
use std::{
    collections::{hash_map::Entry, HashMap},
    num::NonZeroU8,
};

pub(crate) use memo::MortalUnequippedWeaponsMemo;

use crate::{
    exaltation::exalt::ExaltUnequippedWeapons,
    hearthstones::{HearthstoneError, SlottedHearthstone, UnslottedHearthstone},
    weapons::{
        weapon::{
            artifact::{ArtifactWeapon, NonnaturalArtifactWeaponNoAttunement},
            mundane::{MundaneWeaponView, NonnaturalMundaneWeapon},
            Equipped, Weapon, WeaponName, WeaponType,
        },
        WeaponError,
    },
    CharacterMutationError,
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct MortalUnequippedWeapons<'source> {
    pub mundane: HashMap<&'source str, (NonnaturalMundaneWeapon<'source>, NonZeroU8)>,
    pub artifact: HashMap<&'source str, NonnaturalArtifactWeaponNoAttunement<'source>>,
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
    pub fn get_weapon(&'view self, name: WeaponName<'_>) -> Option<Weapon<'source>> {
        match name {
            WeaponName::Unarmed => Some(crate::weapons::weapon::mundane::unarmed()),
            WeaponName::Mundane(name) => match self.mundane.get_key_value(name)? {
                (name, (NonnaturalMundaneWeapon::Worn(worn_weapon), count)) => {
                    Some(Weapon(WeaponType::Mundane(
                        *name,
                        MundaneWeaponView::Worn(worn_weapon.clone(), false),
                        *count,
                    )))
                }
                (name, (NonnaturalMundaneWeapon::OneHanded(one), count)) => {
                    Some(Weapon(WeaponType::Mundane(
                        *name,
                        MundaneWeaponView::OneHanded(one.clone(), None),
                        *count,
                    )))
                }
                (name, (NonnaturalMundaneWeapon::TwoHanded(two), count)) => {
                    Some(Weapon(WeaponType::Mundane(
                        *name,
                        MundaneWeaponView::TwoHanded(two.clone(), false),
                        *count,
                    )))
                }
            },
            WeaponName::Artifact(name) => match self.artifact.get_key_value(name)? {
                (name, NonnaturalArtifactWeaponNoAttunement::Worn(worn)) => Some(Weapon(
                    WeaponType::Artifact(*name, ArtifactWeapon::Worn(worn.clone(), false), None),
                )),
                (name, NonnaturalArtifactWeaponNoAttunement::OneHanded(one)) => Some(Weapon(
                    WeaponType::Artifact(*name, ArtifactWeapon::OneHanded(one.clone(), None), None),
                )),
                (name, NonnaturalArtifactWeaponNoAttunement::TwoHanded(two)) => {
                    Some(Weapon(WeaponType::Artifact(
                        *name,
                        ArtifactWeapon::TwoHanded(two.clone(), false),
                        None,
                    )))
                }
            },
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (WeaponName<'source>, Option<Equipped>)> + '_ {
        self.mundane
            .iter()
            .map(|(name, _)| (WeaponName::Mundane(*name), None))
            .chain(
                self.artifact
                    .iter()
                    .map(|(name, _)| (WeaponName::Artifact(*name), None)),
            )
    }

    pub fn stow_mundane(&mut self, name: &'source str, weapon: NonnaturalMundaneWeapon<'source>) {
        match self.mundane.entry(name) {
            Entry::Occupied(mut e) => {
                e.get_mut().1 = NonZeroU8::new(e.get().1.get().saturating_add(1)).unwrap()
            }
            Entry::Vacant(e) => {
                e.insert((weapon, NonZeroU8::new(1).unwrap()));
            }
        }
    }

    pub fn unstow_mundane(
        &mut self,
        name: &str,
    ) -> Option<(&'source str, NonnaturalMundaneWeapon<'source>)> {
        let (_, count) = self.mundane.get_mut(name)?;
        if let Some(new_nonzero) = NonZeroU8::new(count.get() - 1) {
            *count = new_nonzero;
            self.mundane
                .get_key_value(name)
                .map(|(name, (weapon, _))| (*name, weapon.clone()))
        } else {
            self.mundane
                .remove_entry(name)
                .map(|(name, (weapon, _))| (name, weapon))
        }
    }

    pub fn stow_artifact(
        &mut self,
        name: &'source str,
        weapon: NonnaturalArtifactWeaponNoAttunement<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        if let Entry::Vacant(e) = self.artifact.entry(name) {
            e.insert(weapon);
            Ok(self)
        } else {
            Err(CharacterMutationError::WeaponError(
                WeaponError::DuplicateArtifact,
            ))
        }
    }

    pub fn unstow_artifact(
        &mut self,
        name: &str,
    ) -> Option<(&'source str, NonnaturalArtifactWeaponNoAttunement<'source>)> {
        self.artifact.remove_entry(name)
    }

    pub fn slot_hearthstone(
        &mut self,
        artifact_weapon_name: &str,
        hearthstone_name: &'source str,
        unslotted: UnslottedHearthstone<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        *self
            .artifact
            .get_mut(artifact_weapon_name)
            .ok_or(CharacterMutationError::WeaponError(WeaponError::NotFound))?
            .hearthstone_slots_mut()
            .iter_mut()
            .find(|maybe_hearthstone| maybe_hearthstone.is_none())
            .ok_or(CharacterMutationError::HearthstoneError(
                HearthstoneError::AllSlotsFilled,
            ))? = Some(SlottedHearthstone {
            name: hearthstone_name,
            details: unslotted.details,
            origin: unslotted.origin,
        });
        Ok(self)
    }

    pub fn unslot_hearthstone(
        &mut self,
        artifact_weapon_name: &str,
        hearthstone_name: &str,
    ) -> Result<(&'source str, UnslottedHearthstone<'source>), CharacterMutationError> {
        let SlottedHearthstone {
            name,
            details,
            origin,
        } = self
            .artifact
            .get_mut(artifact_weapon_name)
            .ok_or(CharacterMutationError::WeaponError(WeaponError::NotFound))?
            .hearthstone_slots_mut()
            .iter_mut()
            .find_map(|maybe_hearthstone| {
                if maybe_hearthstone
                    .as_ref()
                    .map_or(false, |hearthstone| hearthstone.name == hearthstone_name)
                {
                    maybe_hearthstone.take()
                } else {
                    None
                }
            })
            .ok_or(CharacterMutationError::HearthstoneError(
                HearthstoneError::NotSlotted,
            ))?;

        Ok((name, UnslottedHearthstone { details, origin }))
    }
}
