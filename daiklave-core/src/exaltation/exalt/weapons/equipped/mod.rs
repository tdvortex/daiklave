use std::{
    collections::{hash_map::Entry, HashMap},
    num::NonZeroU8,
};

use crate::{
    exaltation::{exalt::essence::EssenceError, mortal::MortalEquippedWeapons},
    hearthstones::{HearthstoneError, SlottedHearthstone, UnslottedHearthstone},
    weapons::{
        weapon::{
            artifact::{
                ArtifactWeapon, HandlessArtifactWeapon, HandlessArtifactWeaponNoAttunement,
                WornArtifactWeaponView,
            },
            equipped::{EquippedOneHandedWeapon, EquippedTwoHandedWeapon},
            mundane::{HandlessMundaneWeapon, MundaneWeaponView, WornMundaneWeaponView},
            Equipped, Weapon, WeaponName, WeaponType,
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
    pub handless_artifact: HashMap<&'source str, HandlessArtifactWeapon<'source>>,
    pub hands: ExaltHands<'source>,
}

impl<'source> From<&'source ExaltEquippedWeaponsMemo> for ExaltEquippedWeapons<'source> {
    fn from(value: &'source ExaltEquippedWeaponsMemo) -> Self {
        Self {
            handless_mundane: value
                .handless_mundane
                .iter()
                .map(|(name, weapon)| (name.as_str(), weapon.into()))
                .collect(),
            handless_artifact: value
                .handless_artifact
                .iter()
                .map(|(name, weapon)| (name.as_str(), weapon.into()))
                .collect(),
            hands: (&value.hands).into(),
        }
    }
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

impl<'source> ExaltEquippedWeapons<'source> {
    pub fn get_weapon(
        &self,
        weapon_name: WeaponName<'_>,
        equipped: Equipped,
    ) -> Option<Weapon<'source>> {
        match (weapon_name, equipped) {
            (WeaponName::Unarmed, Equipped::Natural) => {
                Some(crate::weapons::weapon::mundane::unarmed())
            }
            (WeaponName::Unarmed, _) => None,
            (WeaponName::Mundane(name), Equipped::Natural) => {
                match self.handless_mundane.get_key_value(name)? {
                    (name, HandlessMundaneWeapon::Natural(weapon)) => {
                        Some(Weapon(WeaponType::Mundane(
                            name,
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
                            name,
                            MundaneWeaponView::Worn(weapon.clone(), true),
                            NonZeroU8::new(1).unwrap(),
                        )))
                    }
                    (_, HandlessMundaneWeapon::Natural(_)) => None,
                }
            }
            (WeaponName::Artifact(name), Equipped::Natural) => {
                let (&name, handless_artifact_weapon) =
                    self.handless_artifact.get_key_value(name)?;
                let (no_attunement, attunement) =
                    (&handless_artifact_weapon.0, handless_artifact_weapon.1);

                match no_attunement {
                    HandlessArtifactWeaponNoAttunement::Natural(weapon) => {
                        Some(Weapon(WeaponType::Artifact(
                            name,
                            ArtifactWeapon::Natural(weapon.clone()),
                            attunement,
                        )))
                    }
                    HandlessArtifactWeaponNoAttunement::Worn(_) => None,
                }
            }
            (WeaponName::Artifact(name), Equipped::Worn) => {
                let (&name, handless_artifact_weapon) =
                    self.handless_artifact.get_key_value(name)?;
                let (no_attunement, attunement) =
                    (&handless_artifact_weapon.0, handless_artifact_weapon.1);

                match no_attunement {
                    HandlessArtifactWeaponNoAttunement::Worn(weapon) => {
                        Some(Weapon(WeaponType::Artifact(
                            name,
                            ArtifactWeapon::Worn(weapon.clone(), true),
                            attunement,
                        )))
                    }
                    HandlessArtifactWeaponNoAttunement::Natural(_) => None,
                }
            }
            (_, equipped) => self.hands.get_weapon(weapon_name, equipped),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (WeaponName<'source>, Option<Equipped>)> + '_ {
        std::iter::once((WeaponName::Unarmed, Some(Equipped::Natural)))
            .chain(self.hands.iter())
            .chain(self.handless_mundane.iter().map(|(name, weapon)| {
                (
                    WeaponName::Mundane(name),
                    match weapon {
                        HandlessMundaneWeapon::Natural(_) => Some(Equipped::Natural),
                        HandlessMundaneWeapon::Worn(_) => Some(Equipped::Worn),
                    },
                )
            }))
            .chain(self.handless_artifact.iter().map(|(name, weapon)| {
                (
                    WeaponName::Artifact(name),
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
        name: &str,
    ) -> Option<(&'source str, WornMundaneWeaponView<'source>)> {
        match self.handless_mundane.remove_entry(name) {
            Some((name, HandlessMundaneWeapon::Worn(worn_weapon))) => Some((name, worn_weapon)),
            Some((name, not_worn)) => {
                // Not worn, put it back
                self.handless_mundane.insert(name, not_worn);
                None
            }
            None => None,
        }
    }

    pub fn remove_worn_artifact(
        &mut self,
        name: &str,
    ) -> Option<(&'source str, WornArtifactWeaponView<'source>, Option<u8>)> {
        match self.handless_artifact.remove_entry(name) {
            Some((
                name,
                HandlessArtifactWeapon(
                    HandlessArtifactWeaponNoAttunement::Worn(worn_weapon),
                    attunement,
                ),
            )) => Some((name, worn_weapon, attunement)),
            Some((name, not_worn)) => {
                // Not worn, put it back
                self.handless_artifact.insert(name, not_worn);
                None
            }
            None => None,
        }
    }

    pub fn slot_hearthstone(
        &mut self,
        artifact_weapon_name: &str,
        hearthstone_name: &'source str,
        unslotted: UnslottedHearthstone<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        *self
            .handless_artifact
            .get_mut(artifact_weapon_name)
            .map(|weapon| weapon.hearthstone_slots_mut())
            .or_else(|| match &mut self.hands {
                ExaltHands::Empty => None,
                ExaltHands::MainHand(one_handed) | ExaltHands::OffHand(one_handed) => {
                    match one_handed {
                        EquippedOneHandedWeapon::Mundane(_, _) => None,
                        EquippedOneHandedWeapon::Artifact(held_name, held_weapon, _) => {
                            if held_name == &artifact_weapon_name {
                                Some(held_weapon.hearthstone_slots_mut())
                            } else {
                                None
                            }
                        }
                    }
                }
                ExaltHands::Both(arr) => arr.iter_mut().find_map(|one| {
                    if let EquippedOneHandedWeapon::Artifact(held_name, held_weapon, _) = one {
                        if held_name == &artifact_weapon_name {
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
                    EquippedTwoHandedWeapon::Artifact(held_name, held_weapon, _) => {
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
            .handless_artifact
            .get_mut(artifact_weapon_name)
            .map(|weapon| weapon.hearthstone_slots_mut())
            .or_else(|| match &mut self.hands {
                ExaltHands::Empty => None,
                ExaltHands::MainHand(one) | ExaltHands::OffHand(one) => match one {
                    EquippedOneHandedWeapon::Mundane(_, _) => None,
                    EquippedOneHandedWeapon::Artifact(held_name, held_weapon, _) => {
                        if held_name == &artifact_weapon_name {
                            Some(held_weapon.hearthstone_slots_mut())
                        } else {
                            None
                        }
                    }
                },
                ExaltHands::Both(arr) => arr.iter_mut().find_map(|one| match one {
                    EquippedOneHandedWeapon::Mundane(_, _) => None,
                    EquippedOneHandedWeapon::Artifact(held_name, held_weapon, _) => {
                        if held_name == &artifact_weapon_name {
                            Some(held_weapon.hearthstone_slots_mut())
                        } else {
                            None
                        }
                    }
                }),
                ExaltHands::TwoHanded(two) => match two {
                    EquippedTwoHandedWeapon::Mundane(_, _) => None,
                    EquippedTwoHandedWeapon::Artifact(held_name, held_weapon, _) => {
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
                    .map_or(false, |hearthstone| hearthstone.name == hearthstone_name)
                {
                    maybe_hearthstone.take()
                } else {
                    None
                }
            })
            .ok_or(CharacterMutationError::HearthstoneError(
                HearthstoneError::NotFound,
            ))?;

        Ok((name, UnslottedHearthstone { details, origin }))
    }

    pub fn attune_artifact_weapon(
        &mut self,
        artifact_weapon_name: &str,
        personal_committed: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        if let Some(handless) = self.handless_artifact.get_mut(artifact_weapon_name) {
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
                    EquippedOneHandedWeapon::Artifact(held_name, _, attunement) => {
                        if held_name != &artifact_weapon_name {
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
                            if let EquippedOneHandedWeapon::Artifact(held_name, _, attunement) = one
                            {
                                if held_name == &artifact_weapon_name {
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
                    EquippedTwoHandedWeapon::Artifact(held_name, _, attunement) => {
                        if held_name != &artifact_weapon_name {
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
        artifact_weapon_name: &str,
    ) -> Result<(u8, u8), CharacterMutationError> {
        if let Some(handless) = self.handless_artifact.get_mut(artifact_weapon_name) {
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
                    EquippedOneHandedWeapon::Artifact(held_name, _, attunement) => {
                        if held_name == &artifact_weapon_name {
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
                        EquippedOneHandedWeapon::Artifact(held_name, _, attunement) => {
                            if held_name == &artifact_weapon_name {
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
                    EquippedTwoHandedWeapon::Artifact(held_name, _, attunement) => {
                        if held_name == &artifact_weapon_name {
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
