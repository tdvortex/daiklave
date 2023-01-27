use std::collections::hash_map::Entry;

use crate::{
    exaltation::mortal::MortalWeapons,
    hearthstones::{HearthstoneId, UnslottedHearthstone},
    weapons::{
        weapon::{
            artifact::{
                ArtifactWeaponView, HandlessArtifactWeapon, HandlessArtifactWeaponNoAttunement,
                NonnaturalArtifactWeapon, NonnaturalArtifactWeaponNoAttunement,
            },
            equipped::{EquippedOneHandedWeapon, EquippedTwoHandedWeapon},
            mundane::{
                HandlessMundaneWeapon, MundaneWeapon, MundaneWeaponHandedness,
                NaturalMundaneWeaponView, NonnaturalMundaneWeapon, OneHandedMundaneWeaponView,
                TwoHandedMundaneWeaponView, WornMundaneWeaponView,
            },
            ArtifactWeaponId, EquipHand, Equipped, Weapon, WeaponId, WeaponName,
        },
        WeaponError,
    },
    CharacterMutationError,
};

mod equipped;
mod memo;
mod unequipped;

pub(crate) use equipped::{ExaltEquippedWeapons, ExaltHands};
pub(crate) use memo::ExaltWeaponsMemo;
pub(crate) use unequipped::ExaltUnequippedWeapons;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct ExaltWeapons<'source> {
    pub equipped: ExaltEquippedWeapons<'source>,
    pub unequipped: ExaltUnequippedWeapons<'source>,
}

impl<'view, 'source> ExaltWeapons<'source> {
    pub fn as_memo(&'source self) -> ExaltWeaponsMemo {
        ExaltWeaponsMemo {
            equipped: self.equipped.as_memo(),
            unequipped: self.unequipped.as_memo(),
        }
    }

    pub fn get_weapon(
        &'view self,
        weapon_id: WeaponId,
        equipped: Option<Equipped>,
    ) -> Option<Weapon<'source>> {
        if matches!(weapon_id, WeaponId::Unarmed) {
            if matches!(equipped, Some(Equipped::Natural)) {
                Some(crate::weapons::weapon::mundane::unarmed())
            } else {
                None
            }
        } else if let Some(equipped) = equipped {
            self.equipped.get_weapon(weapon_id, equipped)
        } else {
            self.unequipped.get_weapon(weapon_id)
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (WeaponId, Option<Equipped>)> + '_ {
        self.equipped.iter().chain(self.unequipped.iter())
    }

    pub fn add_mundane_weapon(
        &mut self,
        name: &'source str,
        weapon: &'source MundaneWeapon,
    ) -> Result<&mut Self, CharacterMutationError> {
        let nonnatural_mundane = match &weapon.0 {
            MundaneWeaponHandedness::Natural(weapon) => {
                let handless_mundane =
                    HandlessMundaneWeapon::Natural(NaturalMundaneWeaponView(&weapon.0));
                self.equipped
                    .add_natural_mundane_weapon(name, handless_mundane)?;
                return Ok(self);
            }
            MundaneWeaponHandedness::Worn(weapon, _) => {
                NonnaturalMundaneWeapon::Worn(WornMundaneWeaponView(&weapon.0))
            }
            MundaneWeaponHandedness::OneHanded(weapon, _) => {
                NonnaturalMundaneWeapon::OneHanded(OneHandedMundaneWeaponView(&weapon.0))
            }
            MundaneWeaponHandedness::TwoHanded(weapon, _) => {
                NonnaturalMundaneWeapon::TwoHanded(TwoHandedMundaneWeaponView(&weapon.0))
            }
        };

        self.unequipped.stow_mundane(name, nonnatural_mundane);
        Ok(self)
    }

    pub fn equip_weapon(
        &mut self,
        name: &'source WeaponName,
        hand: Option<EquipHand>,
    ) -> Result<&mut Self, CharacterMutationError> {
        match name {
            WeaponName::Unarmed => Err(CharacterMutationError::WeaponError(
                WeaponError::EquipNatural,
            )),
            WeaponName::Mundane(name) => self.equip_mundane_weapon(name.as_str(), hand),
            WeaponName::Artifact(artifact_weapon_id) => {
                self.equip_artifact_weapon(*artifact_weapon_id, hand)
            }
        }
    }

    fn equip_mundane_weapon(
        &mut self,
        name: &'source str,
        hand: Option<EquipHand>,
    ) -> Result<&mut Self, CharacterMutationError> {
        // Try to unstow weapon, error if not found
        let nonnatural_mundane = self
            .unequipped
            .unstow_mundane(name)
            .ok_or(CharacterMutationError::WeaponError(WeaponError::NotFound))?;
        match (nonnatural_mundane, hand) {
            (NonnaturalMundaneWeapon::OneHanded(one_handed), None) => {
                // Don't lose the weapon we unstowed above
                self.unequipped
                    .stow_mundane(name, NonnaturalMundaneWeapon::OneHanded(one_handed));
                Err(CharacterMutationError::WeaponError(
                    WeaponError::HandRequired,
                ))
            }
            (NonnaturalMundaneWeapon::Worn(worn_mundane), _) => {
                if let Entry::Vacant(e) = self.equipped.handless_mundane.entry(name) {
                    e.insert(HandlessMundaneWeapon::Worn(worn_mundane));
                    Ok(self)
                } else {
                    // Don't lose the weapon we unstowed above
                    self.unequipped
                        .stow_mundane(name, NonnaturalMundaneWeapon::Worn(worn_mundane));
                    Err(CharacterMutationError::WeaponError(
                        WeaponError::DuplicateEquippedWorn,
                    ))
                }
            }
            (NonnaturalMundaneWeapon::OneHanded(one_handed_mundane), Some(EquipHand::MainHand)) => {
                let copied = self
                    .equipped
                    .hands
                    .iter()
                    .collect::<Vec<(WeaponId, Option<Equipped>)>>();
                for (hand_weapon_id, maybe_equipped) in copied.into_iter() {
                    match maybe_equipped {
                        None
                        | Some(Equipped::Natural)
                        | Some(Equipped::Worn)
                        | Some(Equipped::OffHand) => { /* Do nothing */ }
                        Some(Equipped::MainHand) => {
                            if let Err(e) = self.unequip_weapon(hand_weapon_id, Equipped::MainHand)
                            {
                                // Don't lose the weapon we unstowed above
                                self.unequipped.stow_mundane(
                                    name,
                                    NonnaturalMundaneWeapon::OneHanded(one_handed_mundane),
                                );
                                return Err(e);
                            }
                        }
                        Some(Equipped::TwoHanded) => {
                            if let Err(e) = self.unequip_weapon(hand_weapon_id, Equipped::TwoHanded)
                            {
                                // Don't lose the weapon we unstowed above
                                self.unequipped.stow_mundane(
                                    name,
                                    NonnaturalMundaneWeapon::OneHanded(one_handed_mundane),
                                );
                                return Err(e);
                            }
                        }
                    }
                }

                self.equipped.hands.set_hand(
                    EquippedOneHandedWeapon::Mundane(name, one_handed_mundane),
                    EquipHand::MainHand,
                );
                Ok(self)
            }
            (NonnaturalMundaneWeapon::OneHanded(one_handed_mundane), Some(EquipHand::OffHand)) => {
                let copied = self
                    .equipped
                    .hands
                    .iter()
                    .collect::<Vec<(WeaponId, Option<Equipped>)>>();
                for (hand_weapon_id, maybe_equipped) in copied.into_iter() {
                    match maybe_equipped {
                        None
                        | Some(Equipped::Natural)
                        | Some(Equipped::Worn)
                        | Some(Equipped::MainHand) => { /* Do nothing */ }
                        Some(Equipped::OffHand) => {
                            if let Err(e) = self.unequip_weapon(hand_weapon_id, Equipped::OffHand) {
                                // Don't lose the weapon we unstowed above
                                self.unequipped.stow_mundane(
                                    name,
                                    NonnaturalMundaneWeapon::OneHanded(one_handed_mundane),
                                );
                                return Err(e);
                            }
                        }
                        Some(Equipped::TwoHanded) => {
                            if let Err(e) = self.unequip_weapon(hand_weapon_id, Equipped::TwoHanded)
                            {
                                // Don't lose the weapon we unstowed above
                                self.unequipped.stow_mundane(
                                    name,
                                    NonnaturalMundaneWeapon::OneHanded(one_handed_mundane),
                                );
                                return Err(e);
                            }
                        }
                    }
                }

                self.equipped.hands.set_hand(
                    EquippedOneHandedWeapon::Mundane(name, one_handed_mundane),
                    EquipHand::OffHand,
                );
                Ok(self)
            }
            (NonnaturalMundaneWeapon::TwoHanded(two_handed_mundane), _) => {
                let copied = self
                    .equipped
                    .hands
                    .iter()
                    .collect::<Vec<(WeaponId, Option<Equipped>)>>();
                for (hand_weapon_id, maybe_equipped) in copied.into_iter() {
                    match maybe_equipped {
                        None | Some(Equipped::Natural) | Some(Equipped::Worn) => { /* Do nothing */
                        }
                        Some(Equipped::MainHand) => {
                            if let Err(e) = self.unequip_weapon(hand_weapon_id, Equipped::MainHand)
                            {
                                // Don't lose the weapon we unstowed above
                                self.unequipped.stow_mundane(
                                    name,
                                    NonnaturalMundaneWeapon::TwoHanded(two_handed_mundane),
                                );
                                return Err(e);
                            }
                        }
                        Some(Equipped::OffHand) => {
                            if let Err(e) = self.unequip_weapon(hand_weapon_id, Equipped::OffHand) {
                                // Don't lose the weapon we unstowed above
                                self.unequipped.stow_mundane(
                                    name,
                                    NonnaturalMundaneWeapon::TwoHanded(two_handed_mundane),
                                );
                                return Err(e);
                            }
                        }
                        Some(Equipped::TwoHanded) => {
                            if let Err(e) = self.unequip_weapon(hand_weapon_id, Equipped::TwoHanded)
                            {
                                // Don't lose the weapon we unstowed above
                                self.unequipped.stow_mundane(
                                    name,
                                    NonnaturalMundaneWeapon::TwoHanded(two_handed_mundane),
                                );
                                return Err(e);
                            }
                        }
                    }
                }

                self.equipped
                    .hands
                    .set_two_handed(EquippedTwoHandedWeapon::Mundane(name, two_handed_mundane));
                Ok(self)
            }
        }
    }

    fn equip_artifact_weapon(
        &mut self,
        weapon_id: ArtifactWeaponId,
        hand: Option<EquipHand>,
    ) -> Result<&mut Self, CharacterMutationError> {
        // Try to unstow weapon, error if not found
        let nonnatural_artifact = self
            .unequipped
            .unstow_artifact(weapon_id)
            .ok_or(CharacterMutationError::WeaponError(WeaponError::NotFound))?;
        let (no_attunement, attunement) = (nonnatural_artifact.0, nonnatural_artifact.1);

        match (no_attunement, hand) {
            (NonnaturalArtifactWeaponNoAttunement::OneHanded(one_handed), None) => {
                // Don't lose the weapon we unstowed above
                self.unequipped.stow_artifact(
                    weapon_id,
                    NonnaturalArtifactWeapon(
                        NonnaturalArtifactWeaponNoAttunement::OneHanded(one_handed),
                        attunement,
                    ),
                )?;
                Err(CharacterMutationError::WeaponError(
                    WeaponError::HandRequired,
                ))
            }
            (NonnaturalArtifactWeaponNoAttunement::Worn(worn), _) => {
                if let Entry::Vacant(e) = self.equipped.handless_artifact.entry(weapon_id) {
                    e.insert(HandlessArtifactWeapon(
                        HandlessArtifactWeaponNoAttunement::Worn(worn),
                        attunement,
                    ));
                    Ok(self)
                } else {
                    // Don't lose the weapon we unstowed above
                    self.unequipped.stow_artifact(
                        weapon_id,
                        NonnaturalArtifactWeapon(
                            NonnaturalArtifactWeaponNoAttunement::Worn(worn),
                            attunement,
                        ),
                    )?;
                    Err(CharacterMutationError::WeaponError(
                        WeaponError::DuplicateEquippedWorn,
                    ))
                }
            }
            (
                NonnaturalArtifactWeaponNoAttunement::OneHanded(one_handed_artifact),
                Some(EquipHand::MainHand),
            ) => {
                let copied = self
                    .equipped
                    .hands
                    .iter()
                    .collect::<Vec<(WeaponId, Option<Equipped>)>>();
                for (hand_weapon_id, maybe_equipped) in copied.into_iter() {
                    match maybe_equipped {
                        None
                        | Some(Equipped::Natural)
                        | Some(Equipped::Worn)
                        | Some(Equipped::OffHand) => { /* Do nothing */ }
                        Some(Equipped::MainHand) => {
                            if let Err(e) = self.unequip_weapon(hand_weapon_id, Equipped::MainHand)
                            {
                                // Don't lose the weapon we unstowed above
                                self.unequipped.stow_artifact(
                                    weapon_id,
                                    NonnaturalArtifactWeapon(
                                        NonnaturalArtifactWeaponNoAttunement::OneHanded(
                                            one_handed_artifact,
                                        ),
                                        attunement,
                                    ),
                                )?;
                                return Err(e);
                            }
                        }
                        Some(Equipped::TwoHanded) => {
                            if let Err(e) = self.unequip_weapon(hand_weapon_id, Equipped::TwoHanded)
                            {
                                // Don't lose the weapon we unstowed above
                                self.unequipped.stow_artifact(
                                    weapon_id,
                                    NonnaturalArtifactWeapon(
                                        NonnaturalArtifactWeaponNoAttunement::OneHanded(
                                            one_handed_artifact,
                                        ),
                                        attunement,
                                    ),
                                )?;
                                return Err(e);
                            }
                        }
                    }
                }

                self.equipped.hands.set_hand(
                    EquippedOneHandedWeapon::Artifact(weapon_id, one_handed_artifact, attunement),
                    EquipHand::MainHand,
                );
                Ok(self)
            }
            (
                NonnaturalArtifactWeaponNoAttunement::OneHanded(one_handed_artifact),
                Some(EquipHand::OffHand),
            ) => {
                let copied = self
                    .equipped
                    .hands
                    .iter()
                    .collect::<Vec<(WeaponId, Option<Equipped>)>>();
                for (hand_weapon_id, maybe_equipped) in copied.into_iter() {
                    match maybe_equipped {
                        None
                        | Some(Equipped::Natural)
                        | Some(Equipped::Worn)
                        | Some(Equipped::MainHand) => { /* Do nothing */ }
                        Some(Equipped::OffHand) => {
                            if let Err(e) = self.unequip_weapon(hand_weapon_id, Equipped::OffHand) {
                                // Don't lose the weapon we unstowed above
                                self.unequipped.stow_artifact(
                                    weapon_id,
                                    NonnaturalArtifactWeapon(
                                        NonnaturalArtifactWeaponNoAttunement::OneHanded(
                                            one_handed_artifact,
                                        ),
                                        attunement,
                                    ),
                                )?;
                                return Err(e);
                            }
                        }
                        Some(Equipped::TwoHanded) => {
                            if let Err(e) = self.unequip_weapon(hand_weapon_id, Equipped::TwoHanded)
                            {
                                // Don't lose the weapon we unstowed above
                                self.unequipped.stow_artifact(
                                    weapon_id,
                                    NonnaturalArtifactWeapon(
                                        NonnaturalArtifactWeaponNoAttunement::OneHanded(
                                            one_handed_artifact,
                                        ),
                                        attunement,
                                    ),
                                )?;
                                return Err(e);
                            }
                        }
                    }
                }

                self.equipped.hands.set_hand(
                    EquippedOneHandedWeapon::Artifact(weapon_id, one_handed_artifact, attunement),
                    EquipHand::OffHand,
                );
                Ok(self)
            }
            (NonnaturalArtifactWeaponNoAttunement::TwoHanded(two_handed_artifact), _) => {
                let copied = self
                    .equipped
                    .hands
                    .iter()
                    .collect::<Vec<(WeaponId, Option<Equipped>)>>();
                for (hand_weapon_id, maybe_equipped) in copied.into_iter() {
                    match maybe_equipped {
                        None | Some(Equipped::Natural) | Some(Equipped::Worn) => { /* Do nothing */
                        }
                        Some(Equipped::MainHand) => {
                            if let Err(e) = self.unequip_weapon(hand_weapon_id, Equipped::MainHand)
                            {
                                // Don't lose the weapon we unstowed above
                                self.unequipped.stow_artifact(
                                    weapon_id,
                                    NonnaturalArtifactWeapon(
                                        NonnaturalArtifactWeaponNoAttunement::TwoHanded(
                                            two_handed_artifact,
                                        ),
                                        attunement,
                                    ),
                                )?;
                                return Err(e);
                            }
                        }
                        Some(Equipped::OffHand) => {
                            if let Err(e) = self.unequip_weapon(hand_weapon_id, Equipped::OffHand) {
                                // Don't lose the weapon we unstowed above
                                self.unequipped.stow_artifact(
                                    weapon_id,
                                    NonnaturalArtifactWeapon(
                                        NonnaturalArtifactWeaponNoAttunement::TwoHanded(
                                            two_handed_artifact,
                                        ),
                                        attunement,
                                    ),
                                )?;
                                return Err(e);
                            }
                        }
                        Some(Equipped::TwoHanded) => {
                            if let Err(e) = self.unequip_weapon(hand_weapon_id, Equipped::TwoHanded)
                            {
                                // Don't lose the weapon we unstowed above
                                self.unequipped.stow_artifact(
                                    weapon_id,
                                    NonnaturalArtifactWeapon(
                                        NonnaturalArtifactWeaponNoAttunement::TwoHanded(
                                            two_handed_artifact,
                                        ),
                                        attunement,
                                    ),
                                )?;
                                return Err(e);
                            }
                        }
                    }
                }

                self.equipped
                    .hands
                    .set_two_handed(EquippedTwoHandedWeapon::Artifact(
                        weapon_id,
                        two_handed_artifact,
                        attunement,
                    ));
                Ok(self)
            }
        }
    }

    pub fn unequip_weapon(
        &mut self,
        weapon_id: WeaponId,
        equipped: Equipped,
    ) -> Result<&mut Self, CharacterMutationError> {
        match weapon_id {
            WeaponId::Unarmed => Err(CharacterMutationError::WeaponError(
                WeaponError::UnequipNatural,
            )),
            WeaponId::Mundane(name) => self.unequip_mundane_weapon(name, equipped),
            WeaponId::Artifact(artifact_weapon_id) => {
                self.unequip_artifact_weapon(artifact_weapon_id, equipped)
            }
        }
    }

    fn unequip_mundane_weapon(
        &mut self,
        name: &str,
        equipped: Equipped,
    ) -> Result<&mut Self, CharacterMutationError> {
        let (name, nonnatural_mundane) = match equipped {
            Equipped::Natural => Err(CharacterMutationError::WeaponError(
                WeaponError::UnequipNatural,
            )),
            Equipped::Worn => self
                .equipped
                .remove_worn_mundane(name)
                .ok_or(CharacterMutationError::WeaponError(WeaponError::NotFound))
                .map(|(name, worn_weapon)| (name, NonnaturalMundaneWeapon::Worn(worn_weapon))),
            Equipped::MainHand => self
                .equipped
                .hands
                .free_hand(WeaponId::Mundane(name), EquipHand::MainHand)
                .ok_or(CharacterMutationError::WeaponError(WeaponError::NotFound))
                .and_then(|one_handed_equipped| match one_handed_equipped {
                    EquippedOneHandedWeapon::Mundane(name, one_handed_mundane) => {
                        Ok((name, NonnaturalMundaneWeapon::OneHanded(one_handed_mundane)))
                    }
                    EquippedOneHandedWeapon::Artifact(_, _, _) => {
                        // This shouldn't happen but if it does put it back and say not found instead
                        self.equipped
                            .hands
                            .set_hand(one_handed_equipped, EquipHand::MainHand);
                        Err(CharacterMutationError::WeaponError(WeaponError::NotFound))
                    }
                }),
            Equipped::OffHand => self
                .equipped
                .hands
                .free_hand(WeaponId::Mundane(name), EquipHand::OffHand)
                .ok_or(CharacterMutationError::WeaponError(WeaponError::NotFound))
                .and_then(|one_handed_equipped| match one_handed_equipped {
                    EquippedOneHandedWeapon::Mundane(name, one_handed_mundane) => {
                        Ok((name, NonnaturalMundaneWeapon::OneHanded(one_handed_mundane)))
                    }
                    EquippedOneHandedWeapon::Artifact(_, _, _) => {
                        // This shouldn't happen but if it does put it back and say not found instead
                        self.equipped
                            .hands
                            .set_hand(one_handed_equipped, EquipHand::OffHand);
                        Err(CharacterMutationError::WeaponError(WeaponError::NotFound))
                    }
                }),
            Equipped::TwoHanded => self
                .equipped
                .hands
                .free_two_handed(WeaponId::Mundane(name))
                .ok_or(CharacterMutationError::WeaponError(WeaponError::NotFound))
                .and_then(|two_handed_equipped| match two_handed_equipped {
                    EquippedTwoHandedWeapon::Mundane(name, two_handed_mundane) => {
                        Ok((name, NonnaturalMundaneWeapon::TwoHanded(two_handed_mundane)))
                    }
                    EquippedTwoHandedWeapon::Artifact(_, _, _) => {
                        // This shouldn't happen but if it does put it back and say not found instead
                        self.equipped.hands.set_two_handed(two_handed_equipped);
                        Err(CharacterMutationError::WeaponError(WeaponError::NotFound))
                    }
                }),
        }?;

        self.unequipped.stow_mundane(name, nonnatural_mundane);
        Ok(self)
    }

    fn unequip_artifact_weapon(
        &mut self,
        weapon_id: ArtifactWeaponId,
        equipped: Equipped,
    ) -> Result<&mut Self, CharacterMutationError> {
        let nonnatural_artifact = match equipped {
            Equipped::Natural => Err(CharacterMutationError::WeaponError(
                WeaponError::UnequipNatural,
            )),
            Equipped::Worn => self
                .equipped
                .remove_worn_artifact(weapon_id)
                .ok_or(CharacterMutationError::WeaponError(WeaponError::NotFound))
                .map(|(worn_artifact, attunement)| {
                    NonnaturalArtifactWeapon(
                        NonnaturalArtifactWeaponNoAttunement::Worn(worn_artifact),
                        attunement,
                    )
                }),
            Equipped::MainHand => self
                .equipped
                .hands
                .free_hand(WeaponId::Artifact(weapon_id), EquipHand::MainHand)
                .ok_or(CharacterMutationError::WeaponError(WeaponError::NotFound))
                .and_then(|one_handed_equipped| match one_handed_equipped {
                    EquippedOneHandedWeapon::Artifact(_, one_handed_artifact, attunement) => {
                        Ok(NonnaturalArtifactWeapon(
                            NonnaturalArtifactWeaponNoAttunement::OneHanded(one_handed_artifact),
                            attunement,
                        ))
                    }
                    EquippedOneHandedWeapon::Mundane(_, _) => {
                        // This shouldn't happen but if it does put it back and say not found instead
                        self.equipped
                            .hands
                            .set_hand(one_handed_equipped, EquipHand::MainHand);
                        Err(CharacterMutationError::WeaponError(WeaponError::NotFound))
                    }
                }),
            Equipped::OffHand => self
                .equipped
                .hands
                .free_hand(WeaponId::Artifact(weapon_id), EquipHand::OffHand)
                .ok_or(CharacterMutationError::WeaponError(WeaponError::NotFound))
                .and_then(|one_handed_equipped| match one_handed_equipped {
                    EquippedOneHandedWeapon::Artifact(_, one_handed_artifact, attunement) => {
                        Ok(NonnaturalArtifactWeapon(
                            NonnaturalArtifactWeaponNoAttunement::OneHanded(one_handed_artifact),
                            attunement,
                        ))
                    }
                    EquippedOneHandedWeapon::Mundane(_, _) => {
                        // This shouldn't happen but if it does put it back and say not found instead
                        self.equipped
                            .hands
                            .set_hand(one_handed_equipped, EquipHand::OffHand);
                        Err(CharacterMutationError::WeaponError(WeaponError::NotFound))
                    }
                }),
            Equipped::TwoHanded => self
                .equipped
                .hands
                .free_two_handed(WeaponId::Artifact(weapon_id))
                .ok_or(CharacterMutationError::WeaponError(WeaponError::NotFound))
                .and_then(|two_handed_equipped| match two_handed_equipped {
                    EquippedTwoHandedWeapon::Artifact(_, two_handed_artifact, attunement) => {
                        Ok(NonnaturalArtifactWeapon(
                            NonnaturalArtifactWeaponNoAttunement::TwoHanded(two_handed_artifact),
                            attunement,
                        ))
                    }
                    EquippedTwoHandedWeapon::Mundane(_, _) => {
                        // This shouldn't happen but if it does put it back and say not found instead
                        self.equipped.hands.set_two_handed(two_handed_equipped);
                        Err(CharacterMutationError::WeaponError(WeaponError::NotFound))
                    }
                }),
        }?;

        self.unequipped
            .stow_artifact(weapon_id, nonnatural_artifact)?;
        Ok(self)
    }

    pub fn add_artifact_weapon(
        &mut self,
        weapon_id: ArtifactWeaponId,
        weapon: ArtifactWeaponView<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        match weapon {
            ArtifactWeaponView::Natural(natural) => {
                if self.equipped.handless_artifact.contains_key(&weapon_id) {
                    Err(CharacterMutationError::WeaponError(
                        WeaponError::NamedArtifactsUnique,
                    ))
                } else if let Entry::Vacant(e) = self.equipped.handless_artifact.entry(weapon_id) {
                    e.insert(HandlessArtifactWeapon(
                        HandlessArtifactWeaponNoAttunement::Natural(natural),
                        None,
                    ));
                    Ok(self)
                } else {
                    Err(CharacterMutationError::WeaponError(
                        WeaponError::NamedArtifactsUnique,
                    ))
                }
            }
            ArtifactWeaponView::Worn(worn, _) => {
                if self.equipped.handless_artifact.contains_key(&weapon_id) {
                    Err(CharacterMutationError::WeaponError(
                        WeaponError::NamedArtifactsUnique,
                    ))
                } else {
                    self.unequipped.stow_artifact(
                        weapon_id,
                        NonnaturalArtifactWeapon(
                            NonnaturalArtifactWeaponNoAttunement::Worn(worn),
                            None,
                        ),
                    )?;
                    Ok(self)
                }
            }
            ArtifactWeaponView::OneHanded(one_handed, _) => {
                if self
                    .equipped
                    .hands
                    .get_weapon(WeaponId::Artifact(weapon_id), Equipped::MainHand)
                    .is_some()
                    || self
                        .equipped
                        .hands
                        .get_weapon(WeaponId::Artifact(weapon_id), Equipped::OffHand)
                        .is_some()
                {
                    Err(CharacterMutationError::WeaponError(
                        WeaponError::NamedArtifactsUnique,
                    ))
                } else {
                    self.unequipped.stow_artifact(
                        weapon_id,
                        NonnaturalArtifactWeapon(
                            NonnaturalArtifactWeaponNoAttunement::OneHanded(one_handed),
                            None,
                        ),
                    )?;
                    Ok(self)
                }
            }
            ArtifactWeaponView::TwoHanded(two_handed, _) => {
                if self
                    .equipped
                    .hands
                    .get_weapon(WeaponId::Artifact(weapon_id), Equipped::TwoHanded)
                    .is_some()
                {
                    Err(CharacterMutationError::WeaponError(
                        WeaponError::NamedArtifactsUnique,
                    ))
                } else {
                    self.unequipped.stow_artifact(
                        weapon_id,
                        NonnaturalArtifactWeapon(
                            NonnaturalArtifactWeaponNoAttunement::TwoHanded(two_handed),
                            None,
                        ),
                    )?;
                    Ok(self)
                }
            }
        }
    }

    pub fn slot_hearthstone(
        &mut self,
        artifact_weapon_id: ArtifactWeaponId,
        hearthstone_id: HearthstoneId,
        unslotted: UnslottedHearthstone<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        let try_slot =
            self.unequipped
                .slot_hearthstone(artifact_weapon_id, hearthstone_id, unslotted);
        match try_slot {
            Ok(_) => Ok(self),
            Err(CharacterMutationError::WeaponError(WeaponError::NotFound)) => {
                self.equipped
                    .slot_hearthstone(artifact_weapon_id, hearthstone_id, unslotted)?;
                Ok(self)
            }
            Err(e) => Err(e),
        }
    }

    pub fn unslot_hearthstone(
        &mut self,
        artifact_weapon_id: ArtifactWeaponId,
        hearthstone_id: HearthstoneId,
    ) -> Result<UnslottedHearthstone<'source>, CharacterMutationError> {
        let try_unslotted = self
            .unequipped
            .unslot_hearthstone(artifact_weapon_id, hearthstone_id);
        match try_unslotted {
            Ok(unslotted) => Ok(unslotted),
            Err(CharacterMutationError::WeaponError(WeaponError::NotFound)) => self
                .equipped
                .unslot_hearthstone(artifact_weapon_id, hearthstone_id),
            Err(e) => Err(e),
        }
    }

    pub fn attune_artifact_weapon(
        &mut self,
        artifact_weapon_id: ArtifactWeaponId,
        personal_committed: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        let try_attune = self
            .unequipped
            .attune_artifact_weapon(artifact_weapon_id, personal_committed);

        match try_attune {
            Ok(_) => Ok(self),
            Err(CharacterMutationError::WeaponError(WeaponError::NotFound)) => {
                self.equipped
                    .attune_artifact_weapon(artifact_weapon_id, personal_committed)?;
                Ok(self)
            }
            Err(e) => Err(e),
        }
    }

    pub fn unattune_artifact_weapon(
        &mut self,
        artifact_weapon_id: ArtifactWeaponId,
    ) -> Result<(u8, u8), CharacterMutationError> {
        match self.unequipped.unattune_artifact_weapon(artifact_weapon_id) {
            Ok((peripheral, personal)) => Ok((peripheral, personal)),
            Err(CharacterMutationError::WeaponError(WeaponError::NotFound)) => {
                self.equipped.unattune_artifact_weapon(artifact_weapon_id)
            }
            Err(e) => Err(e),
        }
    }
}

impl<'source> From<MortalWeapons<'source>> for ExaltWeapons<'source> {
    fn from(mortal: MortalWeapons<'source>) -> Self {
        Self {
            equipped: mortal.equipped.into(),
            unequipped: mortal.unequipped.into(),
        }
    }
}
