use std::{collections::{hash_map::Entry, HashMap}, num::NonZeroU8};

use crate::{
    exaltation::{exalt::essence::EssenceError, mortal::MortalEquippedWeapons},
    hearthstones::{HearthstoneError, HearthstoneId, SlottedHearthstone, UnslottedHearthstone},
    weapons::{
        weapon::{
            artifact::{
                ArtifactWeaponView, HandlessArtifactWeapon, HandlessArtifactWeaponNoAttunement,
                WornArtifactWeaponView,
            },
            equipped::{EquippedOneHandedWeapon, EquippedTwoHandedWeapon},
            mundane::{HandlessMundaneWeapon, MundaneWeaponView, WornMundaneWeaponView},
            ArtifactWeaponId, Equipped, Weapon, WeaponId, WeaponType,
        },
        WeaponError,
    },
    CharacterMutationError,
};

mod hands;
mod memo;
pub(crate) use hands::ExaltHands;
pub(crate) use memo::ExaltEquippedWeaponsMemo;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct ExaltEquippedWeapons<'source> {
    pub handless_mundane: HashMap<&'source str, HandlessMundaneWeapon<'source>>,
    pub handless_artifact: HashMap<ArtifactWeaponId, HandlessArtifactWeapon<'source>>,
    pub hands: ExaltHands<'source>,
}

impl<'source> From<MortalEquippedWeapons<'source>> for ExaltEquippedWeapons<'source> {
    fn from(mortal: MortalEquippedWeapons<'source>) -> Self {
        Self {
            handless_mundane: mortal.handless_mundane,
            handless_artifact: mortal
                .handless_artifact
                .into_iter()
                .map(|(k, v)| (k, HandlessArtifactWeapon(v, None)))
                .collect(),
            hands: mortal.hands.into(),
        }
    }
}

impl<'view, 'source> ExaltEquippedWeapons<'source> {
    pub fn as_memo(&self) -> ExaltEquippedWeaponsMemo {
        ExaltEquippedWeaponsMemo {
            handless_mundane: self
                .handless_mundane
                .iter()
                .map(|(k, v)| ((*k).to_owned(), v.as_memo()))
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
        weapon_id: WeaponId<'view>,
        equipped: Equipped,
    ) -> Option<Weapon<'source>> {
        match (weapon_id, equipped) {
            (WeaponId::Unarmed, Equipped::Natural) => {
                Some(crate::weapons::weapon::mundane::unarmed())
            }
            (WeaponId::Unarmed, _) => None,
            (WeaponId::Mundane(name), Equipped::Natural) => {
                match self.handless_mundane.get_key_value(name)? {
                    (name, HandlessMundaneWeapon::Natural(weapon)) => Some(Weapon(WeaponType::Mundane(
                        *name,
                        MundaneWeaponView::Natural(weapon.clone()),
                        NonZeroU8::new(1).unwrap(),
                    ))),
                    (_, HandlessMundaneWeapon::Worn(_)) => None,
                }
            }
            (WeaponId::Mundane(name), Equipped::Worn) => {
                match self.handless_mundane.get_key_value(name)? {
                    (name, HandlessMundaneWeapon::Worn(weapon)) => Some(Weapon(WeaponType::Mundane(
                        *name,
                        MundaneWeaponView::Worn(weapon.clone(), true),
                        NonZeroU8::new(1).unwrap(),
                    ))),
                    (_, HandlessMundaneWeapon::Natural(_)) => None,
                }
            }
            (WeaponId::Artifact(artifact_weapon_id), Equipped::Natural) => {
                let handless_artifact_weapon = self.handless_artifact.get(&artifact_weapon_id)?;
                let (no_attunement, attunement) =
                    (&handless_artifact_weapon.0, handless_artifact_weapon.1);

                match no_attunement {
                    HandlessArtifactWeaponNoAttunement::Natural(weapon) => {
                        Some(Weapon(WeaponType::Artifact(
                            artifact_weapon_id,
                            ArtifactWeaponView::Natural(weapon.clone()),
                            attunement,
                        )))
                    }
                    HandlessArtifactWeaponNoAttunement::Worn(_) => None,
                }
            }
            (WeaponId::Artifact(artifact_weapon_id), Equipped::Worn) => {
                let handless_artifact_weapon = self.handless_artifact.get(&artifact_weapon_id)?;
                let (no_attunement, attunement) =
                    (&handless_artifact_weapon.0, handless_artifact_weapon.1);

                match no_attunement {
                    HandlessArtifactWeaponNoAttunement::Worn(weapon) => {
                        Some(Weapon(WeaponType::Artifact(
                            artifact_weapon_id,
                            ArtifactWeaponView::Worn(weapon.clone(), true),
                            attunement,
                        )))
                    }
                    HandlessArtifactWeaponNoAttunement::Natural(_) => None,
                }
            }
            (_, equipped) => self.hands.get_weapon(weapon_id, equipped),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (WeaponId, Option<Equipped>)> + '_ {
        std::iter::once((WeaponId::Unarmed, Some(Equipped::Natural)))
            .chain(self.hands.iter())
            .chain(self.handless_mundane.iter().map(|(base_id, weapon)| {
                (
                    WeaponId::Mundane(*base_id),
                    match weapon {
                        HandlessMundaneWeapon::Natural(_) => Some(Equipped::Natural),
                        HandlessMundaneWeapon::Worn(_) => Some(Equipped::Worn),
                    },
                )
            }))
            .chain(self.handless_artifact.iter().map(|(artifact_id, weapon)| {
                (
                    WeaponId::Artifact(*artifact_id),
                    match weapon {
                        HandlessArtifactWeapon(
                            HandlessArtifactWeaponNoAttunement::Natural(_),
                            _,
                        ) => Some(Equipped::Natural),
                        HandlessArtifactWeapon(HandlessArtifactWeaponNoAttunement::Worn(_), _) => {
                            Some(Equipped::Worn)
                        }
                    },
                )
            }))
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
        name: &'view str,
    ) -> Option<(&'source str, WornMundaneWeaponView<'source>)> {
        match self.handless_mundane.remove_entry(name) {
            Some((name, HandlessMundaneWeapon::Natural(natural_weapon))) => {
                // Not worn, put it back
                self.handless_mundane.insert(name, HandlessMundaneWeapon::Natural(natural_weapon));
                None
            }
            Some((name, HandlessMundaneWeapon::Worn(worn_weapon))) => {
                Some((name, worn_weapon))
            }
            None => None,
        }
    }

    pub fn remove_worn_artifact(
        &mut self,
        weapon_id: ArtifactWeaponId,
    ) -> Option<(WornArtifactWeaponView<'source>, Option<u8>)> {
        if matches!(
            self.handless_artifact.get(&weapon_id),
            Some(HandlessArtifactWeapon(
                HandlessArtifactWeaponNoAttunement::Worn(_),
                _
            ))
        ) {
            self.handless_artifact
                .remove(&weapon_id)
                .and_then(|handless_artifact| {
                    let (no_attunement, attunement) = (handless_artifact.0, handless_artifact.1);
                    match no_attunement {
                        HandlessArtifactWeaponNoAttunement::Natural(_) => None,
                        HandlessArtifactWeaponNoAttunement::Worn(worn_artifact) => {
                            Some((worn_artifact, attunement))
                        }
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
        *self
            .handless_artifact
            .get_mut(&artifact_weapon_id)
            .map(|weapon| weapon.hearthstone_slots_mut())
            .or_else(|| match &mut self.hands {
                ExaltHands::Empty => None,
                ExaltHands::MainHand(one_handed) | ExaltHands::OffHand(one_handed) => {
                    match one_handed {
                        EquippedOneHandedWeapon::Mundane(_, _) => None,
                        EquippedOneHandedWeapon::Artifact(held_id, held_weapon, _) => {
                            if held_id != &artifact_weapon_id {
                                None
                            } else {
                                Some(held_weapon.hearthstone_slots_mut())
                            }
                        }
                    }
                }
                ExaltHands::Both(arr) => arr.iter_mut().find_map(|one| {
                    if let EquippedOneHandedWeapon::Artifact(held_id, held_weapon, _) = one {
                        if held_id == &artifact_weapon_id {
                            Some(held_weapon.hearthstone_slots_mut())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }),
                ExaltHands::TwoHanded(two_handed) => match two_handed {
                    EquippedTwoHandedWeapon::Mundane(_, _) => None,
                    EquippedTwoHandedWeapon::Artifact(held_id, held_weapon, _) => {
                        if held_id != &artifact_weapon_id {
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
        artifact_weapon_id: ArtifactWeaponId,
        hearthstone_id: HearthstoneId,
    ) -> Result<UnslottedHearthstone<'source>, CharacterMutationError> {
        let SlottedHearthstone {
            hearthstone_id: _,
            details,
            origin,
        } = self
            .handless_artifact
            .get_mut(&artifact_weapon_id)
            .map(|weapon| weapon.hearthstone_slots_mut())
            .or_else(|| match &mut self.hands {
                ExaltHands::Empty => None,
                ExaltHands::MainHand(one) | ExaltHands::OffHand(one) => match one {
                    EquippedOneHandedWeapon::Mundane(_, _) => None,
                    EquippedOneHandedWeapon::Artifact(held_id, held_weapon, _) => {
                        if held_id == &artifact_weapon_id {
                            Some(held_weapon.hearthstone_slots_mut())
                        } else {
                            None
                        }
                    }
                },
                ExaltHands::Both(arr) => arr.iter_mut().find_map(|one| match one {
                    EquippedOneHandedWeapon::Mundane(_, _) => None,
                    EquippedOneHandedWeapon::Artifact(held_id, held_weapon, _) => {
                        if held_id == &artifact_weapon_id {
                            Some(held_weapon.hearthstone_slots_mut())
                        } else {
                            None
                        }
                    }
                }),
                ExaltHands::TwoHanded(two) => match two {
                    EquippedTwoHandedWeapon::Mundane(_, _) => None,
                    EquippedTwoHandedWeapon::Artifact(held_id, held_weapon, _) => {
                        if held_id == &artifact_weapon_id {
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

    pub fn attune_artifact_weapon(
        &mut self,
        artifact_weapon_id: ArtifactWeaponId,
        personal_committed: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        if let Some(handless) = self.handless_artifact.get_mut(&artifact_weapon_id) {
            if handless.1.is_none() {
                handless.1 = Some(personal_committed);
                Ok(self)
            } else {
                Err(CharacterMutationError::EssenceError(
                    EssenceError::AlreadyAttuned,
                ))
            }
        } else {
            match &mut self.hands {
                ExaltHands::Empty => {
                    Err(CharacterMutationError::WeaponError(WeaponError::NotFound))
                }
                ExaltHands::MainHand(one) | ExaltHands::OffHand(one) => match one {
                    EquippedOneHandedWeapon::Mundane(_, _) => {
                        Err(CharacterMutationError::WeaponError(WeaponError::NotFound))
                    }
                    EquippedOneHandedWeapon::Artifact(held_id, _, attunement) => {
                        if held_id != &artifact_weapon_id {
                            Err(CharacterMutationError::WeaponError(WeaponError::NotFound))
                        } else if attunement.is_some() {
                            Err(CharacterMutationError::EssenceError(
                                EssenceError::AlreadyAttuned,
                            ))
                        } else {
                            *attunement = Some(personal_committed);
                            Ok(self)
                        }
                    }
                },
                ExaltHands::Both(arr) => {
                    arr.iter_mut()
                        .find_map(|one| {
                            if let EquippedOneHandedWeapon::Artifact(held_id, _, attunement) = one {
                                if held_id == &artifact_weapon_id {
                                    Some(attunement)
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        })
                        .ok_or(CharacterMutationError::WeaponError(WeaponError::NotFound))
                        .map(|attunement| {
                            *attunement = Some(personal_committed);
                        })?;
                    Ok(self)
                }
                ExaltHands::TwoHanded(two) => match two {
                    EquippedTwoHandedWeapon::Mundane(_, _) => {
                        Err(CharacterMutationError::WeaponError(WeaponError::NotFound))
                    }
                    EquippedTwoHandedWeapon::Artifact(held_id, _, attunement) => {
                        if held_id != &artifact_weapon_id {
                            Err(CharacterMutationError::WeaponError(WeaponError::NotFound))
                        } else if attunement.is_some() {
                            Err(CharacterMutationError::EssenceError(
                                EssenceError::AlreadyAttuned,
                            ))
                        } else {
                            *attunement = Some(personal_committed);
                            Ok(self)
                        }
                    }
                },
            }
        }
    }

    pub fn unattune_artifact_weapon(
        &mut self,
        artifact_weapon_id: ArtifactWeaponId,
    ) -> Result<(u8, u8), CharacterMutationError> {
        if let Some(handless) = self.handless_artifact.get_mut(&artifact_weapon_id) {
            if let Some(personal) = handless.1.take() {
                Ok((5 - 5.min(personal), 5.min(personal)))
            } else {
                Err(CharacterMutationError::EssenceError(EssenceError::NotFound))
            }
        } else {
            match &mut self.hands {
                ExaltHands::Empty => {
                    Err(CharacterMutationError::WeaponError(WeaponError::NotFound))
                }
                ExaltHands::MainHand(one) | ExaltHands::OffHand(one) => match one {
                    EquippedOneHandedWeapon::Mundane(_, _) => {
                        Err(CharacterMutationError::WeaponError(WeaponError::NotFound))
                    }
                    EquippedOneHandedWeapon::Artifact(held_id, _, attunement) => {
                        if held_id == &artifact_weapon_id {
                            if let Some(personal) = attunement.take() {
                                Ok((5 - 5.min(personal), 5.min(personal)))
                            } else {
                                Err(CharacterMutationError::EssenceError(EssenceError::NotFound))
                            }
                        } else {
                            Err(CharacterMutationError::WeaponError(WeaponError::NotFound))
                        }
                    }
                },
                ExaltHands::Both(arr) => arr
                    .iter_mut()
                    .find_map(|one| match one {
                        EquippedOneHandedWeapon::Mundane(_, _) => None,
                        EquippedOneHandedWeapon::Artifact(held_id, _, attunement) => {
                            if held_id == &artifact_weapon_id {
                                if let Some(personal) = attunement.take() {
                                    Some(Ok((5 - 5.min(personal), 5.min(personal))))
                                } else {
                                    Some(Err(CharacterMutationError::EssenceError(
                                        EssenceError::NotFound,
                                    )))
                                }
                            } else {
                                None
                            }
                        }
                    })
                    .ok_or(CharacterMutationError::WeaponError(WeaponError::NotFound))?,
                ExaltHands::TwoHanded(two) => match two {
                    EquippedTwoHandedWeapon::Mundane(_, _) => {
                        Err(CharacterMutationError::WeaponError(WeaponError::NotFound))
                    }
                    EquippedTwoHandedWeapon::Artifact(held_id, _, attunement) => {
                        if held_id == &artifact_weapon_id {
                            if let Some(personal) = attunement.take() {
                                Ok((5 - 5.min(personal), 5.min(personal)))
                            } else {
                                Err(CharacterMutationError::EssenceError(EssenceError::NotFound))
                            }
                        } else {
                            Err(CharacterMutationError::WeaponError(WeaponError::NotFound))
                        }
                    }
                },
            }
        }
    }
}
