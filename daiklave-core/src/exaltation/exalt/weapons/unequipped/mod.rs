mod memo;
use std::{
    collections::{hash_map::Entry, HashMap},
    num::NonZeroU8,
};

pub(crate) use memo::ExaltUnequippedWeaponsMemo;

use crate::{
    exaltation::{exalt::essence::EssenceError, mortal::MortalUnequippedWeapons},
    hearthstones::{HearthstoneError, SlottedHearthstone, UnslottedHearthstone},
    weapons::{
        weapon::{
            artifact::{
                ArtifactWeapon, NonnaturalArtifactWeapon, NonnaturalArtifactWeaponNoAttunement,
            },
            mundane::{MundaneWeaponView, NonnaturalMundaneWeapon},
            Equipped, Weapon, WeaponName, WeaponType,
        },
        WeaponError,
    },
    CharacterMutationError,
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct ExaltUnequippedWeapons<'source> {
    pub mundane: HashMap<&'source str, (NonnaturalMundaneWeapon<'source>, NonZeroU8)>,
    pub artifact: HashMap<&'source str, NonnaturalArtifactWeapon<'source>>,
}

impl<'source> From<&'source ExaltUnequippedWeaponsMemo> for ExaltUnequippedWeapons<'source> {
    fn from(value: &'source ExaltUnequippedWeaponsMemo) -> Self {
        Self {
            mundane: value.mundane.iter().map(|(name, (weapon, quantity))| (name.as_str(), (weapon.into(), *quantity))).collect(),
            artifact: value.artifact.iter().map(|(name, weapon)| (name.as_str(), weapon.into())).collect(),
        }
    }
}

impl<'source> From<MortalUnequippedWeapons<'source>> for ExaltUnequippedWeapons<'source> {
    fn from(mortal: MortalUnequippedWeapons<'source>) -> Self {
        Self {
            mundane: mortal.mundane,
            artifact: mortal
                .artifact
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
        }
    }
}

impl<'view, 'source> ExaltUnequippedWeapons<'source> {
    pub fn get_weapon(&'view self, weapon_name: WeaponName<'_>) -> Option<Weapon<'source>> {
        match weapon_name {
            WeaponName::Unarmed => Some(crate::weapons::weapon::mundane::unarmed()),
            WeaponName::Mundane(name) => match self.mundane.get_key_value(name)? {
                (name, (NonnaturalMundaneWeapon::Worn(worn_weapon), count)) => {
                    Some(Weapon(WeaponType::Mundane(
                        name,
                        MundaneWeaponView::Worn(worn_weapon.clone(), false),
                        *count,
                    )))
                }
                (name, (NonnaturalMundaneWeapon::OneHanded(one), count)) => {
                    Some(Weapon(WeaponType::Mundane(
                        name,
                        MundaneWeaponView::OneHanded(one.clone(), None),
                        *count,
                    )))
                }
                (name, (NonnaturalMundaneWeapon::TwoHanded(two), count)) => {
                    Some(Weapon(WeaponType::Mundane(
                        name,
                        MundaneWeaponView::TwoHanded(two.clone(), false),
                        *count,
                    )))
                }
            },
            WeaponName::Artifact(name) => {
                let (&name, nonnatural_artifact_weapon) = self.artifact.get_key_value(name)?;
                let (without_attunement, attunement) =
                    (&nonnatural_artifact_weapon.0, nonnatural_artifact_weapon.1);

                match without_attunement {
                    NonnaturalArtifactWeaponNoAttunement::Worn(worn) => {
                        Some(Weapon(WeaponType::Artifact(
                            name,
                            ArtifactWeapon::Worn(worn.clone(), false),
                            attunement,
                        )))
                    }
                    NonnaturalArtifactWeaponNoAttunement::OneHanded(one) => {
                        Some(Weapon(WeaponType::Artifact(
                            name,
                            ArtifactWeapon::OneHanded(one.clone(), None),
                            attunement,
                        )))
                    }
                    NonnaturalArtifactWeaponNoAttunement::TwoHanded(two) => {
                        Some(Weapon(WeaponType::Artifact(
                            name,
                            ArtifactWeapon::TwoHanded(two.clone(), false),
                            attunement,
                        )))
                    }
                }
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (WeaponName<'source>, Option<Equipped>)> + '_ {
        self.mundane
            .iter()
            .map(|(name, _)| (WeaponName::Mundane(name), None))
            .chain(
                self.artifact
                    .iter()
                    .map(|(name, _)| (WeaponName::Artifact(name), None)),
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
        weapon: NonnaturalArtifactWeapon<'source>,
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
    ) -> Option<(&'source str, NonnaturalArtifactWeapon<'source>)> {
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
            .0
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
            .0
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

    pub fn attune_artifact_weapon(
        &mut self,
        artifact_weapon_name: &str,
        personal_committed: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        let attunement = &mut self
            .artifact
            .get_mut(artifact_weapon_name)
            .ok_or(CharacterMutationError::WeaponError(WeaponError::NotFound))?
            .1;

        if attunement.is_some() {
            Err(CharacterMutationError::EssenceError(
                EssenceError::AlreadyAttuned,
            ))
        } else {
            *attunement = Some(personal_committed);
            Ok(self)
        }
    }

    pub fn unattune_artifact_weapon(
        &mut self,
        artifact_weapon_name: &str,
    ) -> Result<(u8, u8), CharacterMutationError> {
        let attunement = self
            .artifact
            .get_mut(artifact_weapon_name)
            .ok_or(CharacterMutationError::WeaponError(WeaponError::NotFound))?
            .1
            .take();

        if let Some(personal) = attunement {
            Ok((5 - 5.min(personal), 5.min(personal)))
        } else {
            Err(CharacterMutationError::EssenceError(EssenceError::NotFound))
        }
    }
}
