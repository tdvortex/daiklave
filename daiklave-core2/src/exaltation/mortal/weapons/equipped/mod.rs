mod hands;
mod memo;
pub(crate) use hands::MortalHands;
pub(crate) use memo::MortalEquippedWeaponsMemo;

use std::collections::{hash_map::Entry, HashMap};

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
            ArtifactWeaponId, BaseWeaponId, Equipped, Weapon, WeaponId, WeaponType,
        },
        WeaponError,
    },
    CharacterMutationError,
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct MortalEquippedWeapons<'source> {
    pub handless_mundane: HashMap<BaseWeaponId, HandlessMundaneWeapon<'source>>,
    pub handless_artifact: HashMap<ArtifactWeaponId, HandlessArtifactWeaponNoAttunement<'source>>,
    pub hands: MortalHands<'source>,
}

impl<'view, 'source> MortalEquippedWeapons<'source> {
    pub fn as_memo(&self) -> MortalEquippedWeaponsMemo {
        MortalEquippedWeaponsMemo {
            handless_mundane: self
                .handless_mundane
                .iter()
                .map(|(k, v)| (*k, v.as_memo()))
                .collect(),
            handless_artifact: self
                .handless_artifact
                .iter()
                .map(|(k, v)| (*k, v.as_memo()))
                .collect(),
            hands: self.hands.as_memo(),
        }
    }

    pub fn get_weapon(
        &'view self,
        weapon_id: WeaponId,
        equipped: Equipped,
    ) -> Option<Weapon<'source>> {
        match (weapon_id, equipped) {
            (WeaponId::Unarmed, Equipped::Natural) => {
                Some(crate::weapons::weapon::mundane::unarmed())
            }
            (WeaponId::Unarmed, _) => None,
            (WeaponId::Mundane(base_weapon_id), Equipped::Natural) => {
                match self.handless_mundane.get(&base_weapon_id)? {
                    HandlessMundaneWeapon::Natural(weapon) => Some(Weapon(WeaponType::Mundane(
                        base_weapon_id,
                        MundaneWeaponView::Natural(weapon.clone()),
                        1,
                    ))),
                    HandlessMundaneWeapon::Worn(_) => None,
                }
            }
            (WeaponId::Mundane(base_weapon_id), Equipped::Worn) => {
                match self.handless_mundane.get(&base_weapon_id)? {
                    HandlessMundaneWeapon::Worn(weapon) => Some(Weapon(WeaponType::Mundane(
                        base_weapon_id,
                        MundaneWeaponView::Worn(weapon.clone(), true),
                        1,
                    ))),
                    HandlessMundaneWeapon::Natural(_) => None,
                }
            }
            (WeaponId::Artifact(artifact_weapon_id), Equipped::Natural) => {
                match self.handless_artifact.get(&artifact_weapon_id)? {
                    HandlessArtifactWeaponNoAttunement::Natural(weapon) => {
                        Some(Weapon(WeaponType::Artifact(
                            artifact_weapon_id,
                            ArtifactWeaponView::Natural(weapon.clone()),
                            None,
                        )))
                    }
                    HandlessArtifactWeaponNoAttunement::Worn(_) => None,
                }
            }
            (WeaponId::Artifact(artifact_weapon_id), Equipped::Worn) => {
                match self.handless_artifact.get(&artifact_weapon_id)? {
                    HandlessArtifactWeaponNoAttunement::Worn(weapon) => {
                        Some(Weapon(WeaponType::Artifact(
                            artifact_weapon_id,
                            ArtifactWeaponView::Worn(weapon.clone(), true),
                            None,
                        )))
                    }
                    HandlessArtifactWeaponNoAttunement::Natural(_) => None,
                }
            }
            (_, equipped) => self.hands.get_weapon(weapon_id, equipped),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (WeaponId, Option<Equipped>)> + '_ {
        let unarmed_iter = std::iter::once((WeaponId::Unarmed, Some(Equipped::Worn)));
        let handless_mundane_iter = self.handless_mundane.iter().map(|(base_id, weapon)| {
            (
                WeaponId::Mundane(*base_id),
                match weapon {
                    HandlessMundaneWeapon::Natural(_) => Some(Equipped::Natural),
                    HandlessMundaneWeapon::Worn(_) => Some(Equipped::Worn),
                },
            )
        });
        let handless_artifact_iter = self.handless_artifact.iter().map(|(artifact_id, weapon)| {
            (
                WeaponId::Artifact(*artifact_id),
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
        weapon_id: BaseWeaponId,
        weapon: HandlessMundaneWeapon<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        if let Entry::Vacant(e) = self.handless_mundane.entry(weapon_id) {
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
        weapon_id: BaseWeaponId,
    ) -> Option<WornMundaneWeaponView<'source>> {
        if matches!(
            self.handless_mundane.get(&weapon_id),
            Some(HandlessMundaneWeapon::Worn(_))
        ) {
            self.handless_mundane
                .remove(&weapon_id)
                .and_then(|handless_mundane| {
                    if let HandlessMundaneWeapon::Worn(worn_mundane) = handless_mundane {
                        Some(worn_mundane)
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
        weapon_id: ArtifactWeaponId,
    ) -> Option<WornArtifactWeaponView<'source>> {
        if matches!(
            self.handless_artifact.get(&weapon_id),
            Some(HandlessArtifactWeaponNoAttunement::Worn(_))
        ) {
            self.handless_artifact
                .remove(&weapon_id)
                .and_then(|handless_artifact| {
                    if let HandlessArtifactWeaponNoAttunement::Worn(worn_artifact) =
                        handless_artifact
                    {
                        Some(worn_artifact)
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
        artifact_weapon_id: ArtifactWeaponId,
        hearthstone_id: HearthstoneId,
        unslotted: UnslottedHearthstone<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        *self.handless_artifact.get_mut(&artifact_weapon_id).map(|weapon| weapon.hearthstone_slots).or_else(|| 
            match &mut self.hands {
            MortalHands::Empty => {
               None
            }
            MortalHands::MainHand(one_handed) | MortalHands::OffHand(one_handed) => {
                match one_handed {
                    EquippedOneHandedWeaponNoAttunement::Mundane(_, _) => {
                        None
                    }
                    EquippedOneHandedWeaponNoAttunement::Artifact(held_id, held_weapon) => {
                        if held_id != &mut artifact_weapon_id {
                            None
                        } else {
                            Some(held_weapon.hearthstone_slots)
                        }
                    }
                }
            }
            MortalHands::Both(arr) => {
                arr.iter_mut()
                    .find_map(|one| {
                        if let EquippedOneHandedWeaponNoAttunement::Artifact(
                            held_id,
                            held_weapon,
                        ) = one
                        {
                            if *held_id == artifact_weapon_id {
                                Some(held_weapon.hearthstone_slots)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
            }
            MortalHands::TwoHanded(two_handed) => match two_handed {
                EquippedTwoHandedWeaponNoAttunement::Mundane(_, _) => {
                    None
                }
                EquippedTwoHandedWeaponNoAttunement::Artifact(held_id, held_weapon) => {
                    if held_id != &mut artifact_weapon_id {
                        None
                    } else {
                        Some(held_weapon.hearthstone_slots)
                    }
                }
            },
        }).ok_or(CharacterMutationError::WeaponError(WeaponError::NotFound))?
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
            origin
        } = self.handless_artifact.get_mut(&artifact_weapon_id).map(|weapon| weapon.hearthstone_slots).or_else(|| match self.hands {
            MortalHands::Empty => None,
            MortalHands::MainHand(one)
            | MortalHands::OffHand(one) => {
                match one {
                    EquippedOneHandedWeaponNoAttunement::Mundane(_, _) => None,
                    EquippedOneHandedWeaponNoAttunement::Artifact(held_id, held_weapon) => {
                        if held_id == artifact_weapon_id {
                            Some(held_weapon.hearthstone_slots)
                        } else {
                            None
                        }
                    }
                }
            }
            MortalHands::Both(arr) => {
                arr.iter_mut().find_map(|one| match one {
                    EquippedOneHandedWeaponNoAttunement::Mundane(_, _) => None,
                    EquippedOneHandedWeaponNoAttunement::Artifact(held_id, held_weapon) => {
                        if held_id == &artifact_weapon_id {
                            Some(held_weapon.hearthstone_slots)
                        } else {
                            None
                        }
                    }
                })
            }
            MortalHands::TwoHanded(two) => {
                match two {
                    EquippedTwoHandedWeaponNoAttunement::Mundane(_, _) => None,
                    EquippedTwoHandedWeaponNoAttunement::Artifact(held_id, held_weapon) => {
                        if held_id == artifact_weapon_id {
                            Some(held_weapon.hearthstone_slots)
                        } else {
                            None
                        }
                    }
                }
            }
        }).ok_or(CharacterMutationError::WeaponError(WeaponError::NotFound))?
        .iter_mut()
        .find_map(|maybe_hearthstone| if maybe_hearthstone.as_ref().map_or(false, |hearthstone| hearthstone.id() == hearthstone_id) {
            maybe_hearthstone.take()
        } else {None})
        .ok_or(CharacterMutationError::HearthstoneError(HearthstoneError::NotFound))?;

        Ok(UnslottedHearthstone { details, origin})
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
