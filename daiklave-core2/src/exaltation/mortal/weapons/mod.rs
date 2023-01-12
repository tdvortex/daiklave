use std::collections::hash_map::Entry;

use crate::{
    exaltation::exalt::ExaltWeapons,
    weapons::{
        weapon::{
            artifact::{
                ArtifactWeaponView, HandlessArtifactWeaponNoAttunement,
                NonnaturalArtifactWeaponNoAttunement,
            },
            equipped::{EquippedOneHandedWeaponNoAttunement, EquippedTwoHandedWeaponNoAttunement},
            mundane::{
                HandlessMundaneWeapon, MundaneWeapon, MundaneWeaponHandedness,
                NaturalMundaneWeaponView, NonnaturalMundaneWeapon, OneHandedMundaneWeaponView,
                TwoHandedMundaneWeaponView, WornMundaneWeaponView,
            },
            ArtifactWeaponId, BaseWeaponId, EquipHand, Equipped, Weapon, WeaponId,
        },
        WeaponError,
    },
    CharacterMutationError, hearthstones::{UnslottedHearthstone, HearthstoneId},
};

mod equipped;
pub(crate) use equipped::{MortalEquippedWeapons, MortalHands};
mod unequipped;
pub(crate) use unequipped::MortalUnequippedWeapons;
mod memo;
pub(crate) use memo::MortalWeaponsMemo;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct MortalWeapons<'source> {
    pub equipped: MortalEquippedWeapons<'source>,
    pub unequipped: MortalUnequippedWeapons<'source>,
}

impl<'source> From<ExaltWeapons<'source>> for MortalWeapons<'source> {
    fn from(exalt: ExaltWeapons<'source>) -> Self {
        Self {
            equipped: exalt.equipped.into(),
            unequipped: exalt.unequipped.into(),
        }
    }
}

impl<'view, 'source> MortalWeapons<'source> {
    pub fn as_memo(&self) -> MortalWeaponsMemo {
        MortalWeaponsMemo {
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
        weapon_id: BaseWeaponId,
        weapon: &'source MundaneWeapon,
    ) -> Result<&mut Self, CharacterMutationError> {
        let nonnatural_mundane = match &weapon.0 {
            MundaneWeaponHandedness::Natural(weapon) => {
                let handless_mundane =
                    HandlessMundaneWeapon::Natural(NaturalMundaneWeaponView(&weapon.0));
                self.equipped
                    .add_natural_mundane_weapon(weapon_id, handless_mundane)?;
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

        self.unequipped.stow_mundane(weapon_id, nonnatural_mundane);
        Ok(self)
    }

    pub fn equip_weapon(
        &mut self,
        weapon_id: WeaponId,
        hand: Option<EquipHand>,
    ) -> Result<&mut Self, CharacterMutationError> {
        match weapon_id {
            WeaponId::Unarmed => Err(CharacterMutationError::WeaponError(
                WeaponError::EquipNatural,
            )),
            WeaponId::Mundane(base_weapon_id) => self.equip_mundane_weapon(base_weapon_id, hand),
            WeaponId::Artifact(artifact_weapon_id) => {
                self.equip_artifact_weapon(artifact_weapon_id, hand)
            }
        }
    }

    fn equip_mundane_weapon(
        &mut self,
        weapon_id: BaseWeaponId,
        hand: Option<EquipHand>,
    ) -> Result<&mut Self, CharacterMutationError> {
        // Try to unstow weapon, error if not found
        let nonnatural_mundane = self
            .unequipped
            .unstow_mundane(weapon_id)
            .ok_or(CharacterMutationError::WeaponError(WeaponError::NotFound))?;
        match (nonnatural_mundane, hand) {
            (NonnaturalMundaneWeapon::OneHanded(one_handed_mundane), None) => {
                // Don't lose the weapon we unstowed above
                self.unequipped.stow_mundane(
                    weapon_id,
                    NonnaturalMundaneWeapon::OneHanded(one_handed_mundane),
                );
                Err(CharacterMutationError::WeaponError(
                    WeaponError::HandRequired,
                ))
            }
            (NonnaturalMundaneWeapon::Worn(worn), _) => {
                if let Entry::Vacant(e) = self.equipped.handless_mundane.entry(weapon_id) {
                    e.insert(HandlessMundaneWeapon::Worn(worn));
                    Ok(self)
                } else {
                    // Don't lose the weapon we unstowed above
                    self.unequipped
                        .stow_mundane(weapon_id, NonnaturalMundaneWeapon::Worn(worn));
                    Err(CharacterMutationError::WeaponError(
                        WeaponError::DuplicateEquippedWorn,
                    ))
                }
            }
            (NonnaturalMundaneWeapon::OneHanded(one_handed_mundane), Some(EquipHand::MainHand)) => {
                for (hand_weapon_id, maybe_equipped) in self.equipped.hands.iter() {
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
                                    weapon_id,
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
                                    weapon_id,
                                    NonnaturalMundaneWeapon::OneHanded(one_handed_mundane),
                                );
                                return Err(e);
                            }
                        }
                    }
                }

                self.equipped.hands.set_hand(
                    EquippedOneHandedWeaponNoAttunement::Mundane(weapon_id, one_handed_mundane),
                    EquipHand::MainHand,
                );
                Ok(self)
            }
            (NonnaturalMundaneWeapon::OneHanded(one_handed_mundane), Some(EquipHand::OffHand)) => {
                for (hand_weapon_id, maybe_equipped) in self.equipped.hands.iter() {
                    match maybe_equipped {
                        None
                        | Some(Equipped::Natural)
                        | Some(Equipped::Worn)
                        | Some(Equipped::MainHand) => { /* Do nothing */ }
                        Some(Equipped::OffHand) => {
                            if let Err(e) = self.unequip_weapon(hand_weapon_id, Equipped::OffHand) {
                                // Don't lose the weapon we unstowed above
                                self.unequipped.stow_mundane(
                                    weapon_id,
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
                                    weapon_id,
                                    NonnaturalMundaneWeapon::OneHanded(one_handed_mundane),
                                );
                                return Err(e);
                            }
                        }
                    }
                }

                self.equipped.hands.set_hand(
                    EquippedOneHandedWeaponNoAttunement::Mundane(weapon_id, one_handed_mundane),
                    EquipHand::OffHand,
                );
                Ok(self)
            }
            (NonnaturalMundaneWeapon::TwoHanded(two_handed_mundane), _) => {
                for (hand_weapon_id, maybe_equipped) in self.equipped.hands.iter() {
                    match maybe_equipped {
                        None | Some(Equipped::Natural) | Some(Equipped::Worn) => { /* Do nothing */
                        }
                        Some(Equipped::MainHand) => {
                            if let Err(e) = self.unequip_weapon(hand_weapon_id, Equipped::MainHand)
                            {
                                // Don't lose the weapon we unstowed above
                                self.unequipped.stow_mundane(
                                    weapon_id,
                                    NonnaturalMundaneWeapon::TwoHanded(two_handed_mundane),
                                );
                                return Err(e);
                            }
                        }
                        Some(Equipped::OffHand) => {
                            if let Err(e) = self.unequip_weapon(hand_weapon_id, Equipped::OffHand) {
                                // Don't lose the weapon we unstowed above
                                self.unequipped.stow_mundane(
                                    weapon_id,
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
                                    weapon_id,
                                    NonnaturalMundaneWeapon::TwoHanded(two_handed_mundane),
                                );
                                return Err(e);
                            }
                        }
                    }
                }

                self.equipped
                    .hands
                    .set_two_handed(EquippedTwoHandedWeaponNoAttunement::Mundane(
                        weapon_id,
                        two_handed_mundane,
                    ));
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
        match (nonnatural_artifact, hand) {
            (NonnaturalArtifactWeaponNoAttunement::OneHanded(one_handed_artifact), None) => {
                // Don't lose the weapon we unstowed above
                self.unequipped.stow_artifact(
                    weapon_id,
                    NonnaturalArtifactWeaponNoAttunement::OneHanded(one_handed_artifact),
                )?;
                Err(CharacterMutationError::WeaponError(
                    WeaponError::HandRequired,
                ))
            }
            (NonnaturalArtifactWeaponNoAttunement::Worn(worn), _) => {
                if let Entry::Vacant(e) = self.equipped.handless_artifact.entry(weapon_id) {
                    e.insert(HandlessArtifactWeaponNoAttunement::Worn(worn));
                    Ok(self)
                } else {
                    // Don't lose the weapon we unstowed above
                    self.unequipped.stow_artifact(
                        weapon_id,
                        NonnaturalArtifactWeaponNoAttunement::Worn(worn),
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
                for (hand_weapon_id, maybe_equipped) in self.equipped.hands.iter() {
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
                                    NonnaturalArtifactWeaponNoAttunement::OneHanded(
                                        one_handed_artifact,
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
                                    NonnaturalArtifactWeaponNoAttunement::OneHanded(
                                        one_handed_artifact,
                                    ),
                                )?;
                                return Err(e);
                            }
                        }
                    }
                }

                self.equipped.hands.set_hand(
                    EquippedOneHandedWeaponNoAttunement::Artifact(weapon_id, one_handed_artifact),
                    EquipHand::MainHand,
                );
                Ok(self)
            }
            (
                NonnaturalArtifactWeaponNoAttunement::OneHanded(one_handed_artifact),
                Some(EquipHand::OffHand),
            ) => {
                for (hand_weapon_id, maybe_equipped) in self.equipped.hands.iter() {
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
                                    NonnaturalArtifactWeaponNoAttunement::OneHanded(
                                        one_handed_artifact,
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
                                    NonnaturalArtifactWeaponNoAttunement::OneHanded(
                                        one_handed_artifact,
                                    ),
                                )?;
                                return Err(e);
                            }
                        }
                    }
                }

                self.equipped.hands.set_hand(
                    EquippedOneHandedWeaponNoAttunement::Artifact(weapon_id, one_handed_artifact),
                    EquipHand::OffHand,
                );
                Ok(self)
            }
            (NonnaturalArtifactWeaponNoAttunement::TwoHanded(two_handed_artifact), _) => {
                for (hand_weapon_id, maybe_equipped) in self.equipped.hands.iter() {
                    match maybe_equipped {
                        None | Some(Equipped::Natural) | Some(Equipped::Worn) => { /* Do nothing */
                        }
                        Some(Equipped::MainHand) => {
                            if let Err(e) = self.unequip_weapon(hand_weapon_id, Equipped::MainHand)
                            {
                                // Don't lose the weapon we unstowed above
                                self.unequipped.stow_artifact(
                                    weapon_id,
                                    NonnaturalArtifactWeaponNoAttunement::TwoHanded(
                                        two_handed_artifact,
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
                                    NonnaturalArtifactWeaponNoAttunement::TwoHanded(
                                        two_handed_artifact,
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
                                    NonnaturalArtifactWeaponNoAttunement::TwoHanded(
                                        two_handed_artifact,
                                    ),
                                )?;
                                return Err(e);
                            }
                        }
                    }
                }

                self.equipped
                    .hands
                    .set_two_handed(EquippedTwoHandedWeaponNoAttunement::Artifact(
                        weapon_id,
                        two_handed_artifact,
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
            WeaponId::Mundane(base_weapon_id) => {
                self.unequip_mundane_weapon(base_weapon_id, equipped)
            }
            WeaponId::Artifact(artifact_weapon_id) => {
                self.unequip_artifact_weapon(artifact_weapon_id, equipped)
            }
        }
    }

    fn unequip_mundane_weapon(
        &mut self,
        weapon_id: BaseWeaponId,
        equipped: Equipped,
    ) -> Result<&mut Self, CharacterMutationError> {
        let nonnatural_mundane = match equipped {
            Equipped::Natural => Err(CharacterMutationError::WeaponError(
                WeaponError::UnequipNatural,
            )),
            Equipped::Worn => self
                .equipped
                .remove_worn_mundane(weapon_id)
                .ok_or(CharacterMutationError::WeaponError(WeaponError::NotFound))
                .map(NonnaturalMundaneWeapon::Worn),
            Equipped::MainHand => self
                .equipped
                .hands
                .free_hand(WeaponId::Mundane(weapon_id), EquipHand::MainHand)
                .ok_or(CharacterMutationError::WeaponError(WeaponError::NotFound))
                .and_then(|one_handed_equipped| match one_handed_equipped {
                    EquippedOneHandedWeaponNoAttunement::Mundane(_, one_handed_mundane) => {
                        Ok(NonnaturalMundaneWeapon::OneHanded(one_handed_mundane))
                    }
                    EquippedOneHandedWeaponNoAttunement::Artifact(_, _) => {
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
                .free_hand(WeaponId::Mundane(weapon_id), EquipHand::OffHand)
                .ok_or(CharacterMutationError::WeaponError(WeaponError::NotFound))
                .and_then(|one_handed_equipped| match one_handed_equipped {
                    EquippedOneHandedWeaponNoAttunement::Mundane(_, one_handed_mundane) => {
                        Ok(NonnaturalMundaneWeapon::OneHanded(one_handed_mundane))
                    }
                    EquippedOneHandedWeaponNoAttunement::Artifact(_, _) => {
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
                .free_two_handed(WeaponId::Mundane(weapon_id))
                .ok_or(CharacterMutationError::WeaponError(WeaponError::NotFound))
                .and_then(|two_handed_equipped| match two_handed_equipped {
                    EquippedTwoHandedWeaponNoAttunement::Mundane(_, two_handed_mundane) => {
                        Ok(NonnaturalMundaneWeapon::TwoHanded(two_handed_mundane))
                    }
                    EquippedTwoHandedWeaponNoAttunement::Artifact(_, _) => {
                        // This shouldn't happen but if it does put it back and say not found instead
                        self.equipped.hands.set_two_handed(two_handed_equipped);
                        Err(CharacterMutationError::WeaponError(WeaponError::NotFound))
                    }
                }),
        }?;

        self.unequipped.stow_mundane(weapon_id, nonnatural_mundane);
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
                .map(NonnaturalArtifactWeaponNoAttunement::Worn),
            Equipped::MainHand => self
                .equipped
                .hands
                .free_hand(WeaponId::Artifact(weapon_id), EquipHand::MainHand)
                .ok_or(CharacterMutationError::WeaponError(WeaponError::NotFound))
                .and_then(|one_handed_equipped| match one_handed_equipped {
                    EquippedOneHandedWeaponNoAttunement::Artifact(_, one_handed_artifact) => Ok(
                        NonnaturalArtifactWeaponNoAttunement::OneHanded(one_handed_artifact),
                    ),
                    EquippedOneHandedWeaponNoAttunement::Mundane(_, _) => {
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
                    EquippedOneHandedWeaponNoAttunement::Artifact(_, one_handed_artifact) => Ok(
                        NonnaturalArtifactWeaponNoAttunement::OneHanded(one_handed_artifact),
                    ),
                    EquippedOneHandedWeaponNoAttunement::Mundane(_, _) => {
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
                    EquippedTwoHandedWeaponNoAttunement::Artifact(_, two_handed_artifact) => Ok(
                        NonnaturalArtifactWeaponNoAttunement::TwoHanded(two_handed_artifact),
                    ),
                    EquippedTwoHandedWeaponNoAttunement::Mundane(_, _) => {
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
                    e.insert(HandlessArtifactWeaponNoAttunement::Natural(natural));
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
                        NonnaturalArtifactWeaponNoAttunement::Worn(worn),
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
                        NonnaturalArtifactWeaponNoAttunement::OneHanded(one_handed),
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
                        NonnaturalArtifactWeaponNoAttunement::TwoHanded(two_handed),
                    )?;
                    Ok(self)
                }
            }
        }
    }

    pub fn slot_hearthstone(&mut self, artifact_weapon_id: ArtifactWeaponId, hearthstone_id: HearthstoneId, unslotted: UnslottedHearthstone<'source>) -> Result<&mut Self, CharacterMutationError> {
        let try_slot = self.unequipped.slot_hearthstone(artifact_weapon_id, hearthstone_id, unslotted); 
        match try_slot {
            Ok(_) => Ok(self),
            Err(CharacterMutationError::WeaponError(WeaponError::NotFound)) => {
                self.equipped.slot_hearthstone(artifact_weapon_id, hearthstone_id, unslotted)?;
                Ok(self)
            }
            Err(e) => Err(e),
        }
    }

    pub fn unslot_hearthstone(&mut self, artifact_weapon_id: ArtifactWeaponId, hearthstone_id: HearthstoneId) -> Result<UnslottedHearthstone<'source>, CharacterMutationError> {
        let try_unslotted = self.unequipped.unslot_hearthstone(artifact_weapon_id, hearthstone_id); 
        match try_unslotted {
            Ok(unslotted) => Ok(unslotted),
            Err(CharacterMutationError::WeaponError(WeaponError::NotFound)) => {
                self.equipped.unslot_hearthstone(artifact_weapon_id, hearthstone_id)
            }
            Err(e) => Err(e),
        }
    }
}
