mod hands;
mod memo;
pub(crate) use hands::MortalHands;
pub(crate) use memo::MortalEquippedWeaponsMemo;

use std::{
    collections::{hash_map::Entry, HashMap},
    num::NonZeroU8,
};

use crate::{
    exaltation::exalt::ExaltEquippedWeapons,
    hearthstones::{HearthstoneError, HearthstoneId, SlottedHearthstone, UnslottedHearthstone},
    weapons::{
        weapon::{
            artifact::{
                ArtifactWeaponView, HandlessArtifactWeaponNoAttunement, WornArtifactWeaponView,
            },
            equipped::{EquippedOneHandedWeaponNoAttunement, EquippedTwoHandedWeaponNoAttunement},
            mundane::{HandlessMundaneWeapon, MundaneWeaponView, WornMundaneWeaponView},
            Equipped, Weapon, WeaponType, WeaponName,
        },
        WeaponError,
    },
    CharacterMutationError,
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct MortalEquippedWeapons<'source> {
    pub handless_mundane: HashMap<&'source str, HandlessMundaneWeapon<'source>>,
    pub handless_artifact: HashMap<&'source str, HandlessArtifactWeaponNoAttunement<'source>>,
    pub hands: MortalHands<'source>,
}

impl<'view, 'source> MortalEquippedWeapons<'source> {
    pub fn as_memo(&self) -> MortalEquippedWeaponsMemo {
        MortalEquippedWeaponsMemo {
            handless_mundane: self
                .handless_mundane
                .iter()
                .map(|(k, v)| ((*k).to_owned(), v.as_memo()))
                .collect(),
            handless_artifact: self
                .handless_artifact
                .iter()
                .map(|(k, v)| ((*k).to_owned(), v.as_memo()))
                .collect(),
            hands: self.hands.as_memo(),
        }
    }

    pub fn get_weapon(
        &self,
        name: WeaponName<'_>,
        equipped: Equipped,
    ) -> Option<Weapon<'source>> {
        match (name, equipped) {
            (WeaponName::Unarmed, Equipped::Natural) => {
                Some(crate::weapons::weapon::mundane::unarmed())
            }
            (WeaponName::Unarmed, _) => None,
            (WeaponName::Mundane(name), Equipped::Natural) => {
                match self.handless_mundane.get_key_value(name)? {
                    (name, HandlessMundaneWeapon::Natural(weapon)) => {
                        Some(Weapon(WeaponType::Mundane(
                            *name,
                            MundaneWeaponView::Natural(weapon.clone()),
                            NonZeroU8::new(1).unwrap(),
                        )))
                    }
                    (_, HandlessMundaneWeapon::Worn(_)) => None,
                }
            }
            (WeaponName::Mundane(name), Equipped::Worn) => {
                match self.handless_mundane.get_key_value(name)? {
                    (name, HandlessMundaneWeapon::Worn(weapon)) => {
                        Some(Weapon(WeaponType::Mundane(
                            *name,
                            MundaneWeaponView::Worn(weapon.clone(), true),
                            NonZeroU8::new(1).unwrap(),
                        )))
                    }
                    (_, HandlessMundaneWeapon::Natural(_)) => None,
                }
            }
            (WeaponName::Artifact(name), Equipped::Natural) => {
                match self.handless_artifact.get_key_value(name)? {
                    (name, HandlessArtifactWeaponNoAttunement::Natural(weapon)) => {
                        Some(Weapon(WeaponType::Artifact(
                            *name,
                            ArtifactWeaponView::Natural(weapon.clone()),
                            None,
                        )))
                    }
                    (_, HandlessArtifactWeaponNoAttunement::Worn(_)) => None,
                }
            }
            (WeaponName::Artifact(name), Equipped::Worn) => {
                match self.handless_artifact.get_key_value(name)? {
                    (&name, HandlessArtifactWeaponNoAttunement::Worn(weapon)) => {
                        Some(Weapon(WeaponType::Artifact(
                            name,
                            ArtifactWeaponView::Worn(weapon.clone(), true),
                            None,
                        )))
                    }
                    (_, HandlessArtifactWeaponNoAttunement::Natural(_)) => None,
                }
            }
            (_, equipped) => self.hands.get_weapon(name, equipped),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (WeaponName<'source>, Option<Equipped>)> + '_ {
        let unarmed_iter = std::iter::once((WeaponName::Unarmed, Some(Equipped::Natural)));
        let handless_mundane_iter = self.handless_mundane.iter().map(|(name, weapon)| {
            (
                WeaponName::Mundane(*name),
                match weapon {
                    HandlessMundaneWeapon::Natural(_) => Some(Equipped::Natural),
                    HandlessMundaneWeapon::Worn(_) => Some(Equipped::Worn),
                },
            )
        });
        let handless_artifact_iter = self.handless_artifact.iter().map(|(name, weapon)| {
            (
                WeaponName::Artifact(*name),
                match weapon {
                    HandlessArtifactWeaponNoAttunement::Natural(_) => Some(Equipped::Natural),
                    HandlessArtifactWeaponNoAttunement::Worn(_) => Some(Equipped::Worn),
                },
            )
        });
        let hands_iter = self.hands.iter();

        unarmed_iter
            .chain(handless_artifact_iter)
            .chain(handless_mundane_iter)
            .chain(hands_iter)
    }

    pub fn add_natural_mundane_weapon(
        &mut self,
        name: &'source str,
        weapon: HandlessMundaneWeapon<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        if let Entry::Vacant(e) = self.handless_mundane.entry(name) {
            e.insert(weapon);
            Ok(self)
        } else {
            Err(CharacterMutationError::WeaponError(
                WeaponError::DuplicateNatural,
            ))
        }
    }

    pub fn remove_worn_mundane(
        &mut self,
        name: &str,
    ) -> Option<(&'source str, WornMundaneWeaponView<'source>)> {
        if matches!(
            self.handless_mundane.get(name),
            Some(HandlessMundaneWeapon::Worn(_))
        ) {
            self.handless_mundane
                .remove_entry(name)
                .and_then(|(name, handless_mundane)| {
                    if let HandlessMundaneWeapon::Worn(worn_mundane) = handless_mundane {
                        Some((name, worn_mundane))
                    } else {
                        None
                    }
                })
        } else {
            None
        }
    }

    pub fn remove_worn_artifact(
        &mut self,
        name: &str,
    ) -> Option<(&'source str, WornArtifactWeaponView<'source>)> {
        if matches!(
            self.handless_artifact.get(name),
            Some(HandlessArtifactWeaponNoAttunement::Worn(_))
        ) {
            self.handless_artifact
                .remove_entry(name)
                .and_then(|(name, handless_artifact)| {
                    if let HandlessArtifactWeaponNoAttunement::Worn(worn_artifact) =
                        handless_artifact
                    {
                        Some((name, worn_artifact))
                    } else {
                        None
                    }
                })
        } else {
            None
        }
    }

    pub fn slot_hearthstone(
        &mut self,
        artifact_weapon_name: &str,
        hearthstone_id: HearthstoneId,
        unslotted: UnslottedHearthstone<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        *self
            .handless_artifact
            .get_mut(artifact_weapon_name)
            .map(|weapon| weapon.hearthstone_slots_mut())
            .or_else(|| match &mut self.hands {
                MortalHands::Empty => None,
                MortalHands::MainHand(one_handed) | MortalHands::OffHand(one_handed) => {
                    match one_handed {
                        EquippedOneHandedWeaponNoAttunement::Mundane(_, _) => None,
                        EquippedOneHandedWeaponNoAttunement::Artifact(held_name, held_weapon) => {
                            if held_name != &artifact_weapon_name {
                                None
                            } else {
                                Some(held_weapon.hearthstone_slots_mut())
                            }
                        }
                    }
                }
                MortalHands::Both(arr) => arr.iter_mut().find_map(|one| {
                    if let EquippedOneHandedWeaponNoAttunement::Artifact(held_name, held_weapon) = one
                    {
                        if held_name != &artifact_weapon_name {
                            None
                        } else {
                            Some(held_weapon.hearthstone_slots_mut())
                        }
                    } else {
                        None
                    }
                }),
                MortalHands::TwoHanded(two_handed) => match two_handed {
                    EquippedTwoHandedWeaponNoAttunement::Mundane(_, _) => None,
                    EquippedTwoHandedWeaponNoAttunement::Artifact(held_name, held_weapon) => {
                        if held_name != &artifact_weapon_name {
                            None
                        } else {
                            Some(held_weapon.hearthstone_slots_mut())
                        }
                    }
                },
            })
            .ok_or(CharacterMutationError::WeaponError(WeaponError::NotFound))?
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
        artifact_weapon_name: &str,
        hearthstone_id: HearthstoneId,
    ) -> Result<UnslottedHearthstone<'source>, CharacterMutationError> {
        let SlottedHearthstone {
            hearthstone_id: _,
            details,
            origin,
        } = self
            .handless_artifact
            .get_mut(artifact_weapon_name)
            .map(|weapon| weapon.hearthstone_slots_mut())
            .or_else(|| match &mut self.hands {
                MortalHands::Empty => None,
                MortalHands::MainHand(one) | MortalHands::OffHand(one) => match one {
                    EquippedOneHandedWeaponNoAttunement::Mundane(_, _) => None,
                    EquippedOneHandedWeaponNoAttunement::Artifact(held_name, held_weapon) => {
                        if held_name == &artifact_weapon_name {
                            Some(held_weapon.hearthstone_slots_mut())
                        } else {
                            None
                        }
                    }
                },
                MortalHands::Both(arr) => arr.iter_mut().find_map(|one| match one {
                    EquippedOneHandedWeaponNoAttunement::Mundane(_, _) => None,
                    EquippedOneHandedWeaponNoAttunement::Artifact(held_name, held_weapon) => {
                        if held_name == &artifact_weapon_name {
                            Some(held_weapon.hearthstone_slots_mut())
                        } else {
                            None
                        }
                    }
                }),
                MortalHands::TwoHanded(two) => match two {
                    EquippedTwoHandedWeaponNoAttunement::Mundane(_, _) => None,
                    EquippedTwoHandedWeaponNoAttunement::Artifact(held_name, held_weapon) => {
                        if held_name == &artifact_weapon_name {
                            Some(held_weapon.hearthstone_slots_mut())
                        } else {
                            None
                        }
                    }
                },
            })
            .ok_or(CharacterMutationError::WeaponError(WeaponError::NotFound))?
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
                HearthstoneError::NotFound,
            ))?;

        Ok(UnslottedHearthstone { details, origin })
    }
}

impl<'source> From<ExaltEquippedWeapons<'source>> for MortalEquippedWeapons<'source> {
    fn from(exalt: ExaltEquippedWeapons<'source>) -> Self {
        Self {
            handless_mundane: exalt.handless_mundane,
            handless_artifact: exalt
                .handless_artifact
                .into_iter()
                .map(|(k, v)| (k, v.0))
                .collect(),
            hands: exalt.hands.into(),
        }
    }
}
