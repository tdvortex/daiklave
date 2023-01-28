use std::collections::hash_map::Entry;

use crate::{
    exaltation::exalt::ExaltWeapons,
    hearthstones::UnslottedHearthstone,
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
            EquipHand, Equipped, Weapon, WeaponName,
        },
        WeaponError,
    },
    CharacterMutationError,
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
        name: WeaponName<'_>,
        equipped: Option<Equipped>,
    ) -> Option<Weapon<'source>> {
        if matches!(name, WeaponName::Unarmed) {
            if matches!(equipped, Some(Equipped::Natural)) {
                Some(crate::weapons::weapon::mundane::unarmed())
            } else {
                None
            }
        } else if let Some(equipped) = equipped {
            self.equipped.get_weapon(name, equipped)
        } else {
            self.unequipped.get_weapon(name)
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (WeaponName<'source>, Option<Equipped>)> + '_ {
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
        name: WeaponName<'_>,
        hand: Option<EquipHand>,
    ) -> Result<&mut Self, CharacterMutationError> {
        match name {
            WeaponName::Unarmed => Err(CharacterMutationError::WeaponError(
                WeaponError::EquipNatural,
            )),
            WeaponName::Mundane(name) => self.equip_mundane_weapon(name, hand),
            WeaponName::Artifact(name) => self.equip_artifact_weapon(name, hand),
        }
    }

    fn equip_mundane_weapon(
        &mut self,
        name: &str,
        hand: Option<EquipHand>,
    ) -> Result<&mut Self, CharacterMutationError> {
        // Try to unstow weapon, error if not found
        let (name, nonnatural_mundane) = self
            .unequipped
            .unstow_mundane(name)
            .ok_or(CharacterMutationError::WeaponError(WeaponError::NotFound))?;
        match (nonnatural_mundane, hand) {
            (NonnaturalMundaneWeapon::OneHanded(one_handed_mundane), None) => {
                // Don't lose the weapon we unstowed above
                self.unequipped
                    .stow_mundane(name, NonnaturalMundaneWeapon::OneHanded(one_handed_mundane));
                Err(CharacterMutationError::WeaponError(
                    WeaponError::HandRequired,
                ))
            }
            (NonnaturalMundaneWeapon::Worn(worn), _) => {
                if let Entry::Vacant(e) = self.equipped.handless_mundane.entry(name) {
                    e.insert(HandlessMundaneWeapon::Worn(worn));
                    Ok(self)
                } else {
                    // Don't lose the weapon we unstowed above
                    self.unequipped
                        .stow_mundane(name, NonnaturalMundaneWeapon::Worn(worn));
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
                    EquippedOneHandedWeaponNoAttunement::Mundane(name, one_handed_mundane),
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
                    EquippedOneHandedWeaponNoAttunement::Mundane(name, one_handed_mundane),
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
                    .set_two_handed(EquippedTwoHandedWeaponNoAttunement::Mundane(
                        name,
                        two_handed_mundane,
                    ));
                Ok(self)
            }
        }
    }

    fn equip_artifact_weapon(
        &mut self,
        name: &str,
        hand: Option<EquipHand>,
    ) -> Result<&mut Self, CharacterMutationError> {
        // Try to unstow weapon, error if not found
        let (name, nonnatural_artifact) = self
            .unequipped
            .unstow_artifact(name)
            .ok_or(CharacterMutationError::WeaponError(WeaponError::NotFound))?;
        match (nonnatural_artifact, hand) {
            (NonnaturalArtifactWeaponNoAttunement::OneHanded(one_handed_artifact), None) => {
                // Don't lose the weapon we unstowed above
                self.unequipped.stow_artifact(
                    name,
                    NonnaturalArtifactWeaponNoAttunement::OneHanded(one_handed_artifact),
                )?;
                Err(CharacterMutationError::WeaponError(
                    WeaponError::HandRequired,
                ))
            }
            (NonnaturalArtifactWeaponNoAttunement::Worn(worn), _) => {
                if let Entry::Vacant(e) = self.equipped.handless_artifact.entry(name) {
                    e.insert(HandlessArtifactWeaponNoAttunement::Worn(worn));
                    Ok(self)
                } else {
                    // Don't lose the weapon we unstowed above
                    self.unequipped
                        .stow_artifact(name, NonnaturalArtifactWeaponNoAttunement::Worn(worn))?;
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
                                    name,
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
                                    name,
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
                    EquippedOneHandedWeaponNoAttunement::Artifact(name, one_handed_artifact),
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
                                    name,
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
                                    name,
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
                    EquippedOneHandedWeaponNoAttunement::Artifact(name, one_handed_artifact),
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
                                    name,
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
                                    name,
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
                                    name,
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
                        name,
                        two_handed_artifact,
                    ));
                Ok(self)
            }
        }
    }

    pub fn unequip_weapon(
        &mut self,
        weapon_name: WeaponName<'_>,
        equipped: Equipped,
    ) -> Result<&mut Self, CharacterMutationError> {
        match weapon_name {
            WeaponName::Unarmed => Err(CharacterMutationError::WeaponError(
                WeaponError::UnequipNatural,
            )),
            WeaponName::Mundane(name) => self.unequip_mundane_weapon(name, equipped),
            WeaponName::Artifact(name) => self.unequip_artifact_weapon(name, equipped),
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
                .map(|(name, worn_mundane)| (name, NonnaturalMundaneWeapon::Worn(worn_mundane))),
            Equipped::MainHand => self
                .equipped
                .hands
                .free_hand(WeaponName::Mundane(name), EquipHand::MainHand)
                .ok_or(CharacterMutationError::WeaponError(WeaponError::NotFound))
                .and_then(|one_handed_equipped| match one_handed_equipped {
                    EquippedOneHandedWeaponNoAttunement::Mundane(name, one_handed_mundane) => {
                        Ok((name, NonnaturalMundaneWeapon::OneHanded(one_handed_mundane)))
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
                .free_hand(WeaponName::Mundane(name), EquipHand::OffHand)
                .ok_or(CharacterMutationError::WeaponError(WeaponError::NotFound))
                .and_then(|one_handed_equipped| match one_handed_equipped {
                    EquippedOneHandedWeaponNoAttunement::Mundane(name, one_handed_mundane) => {
                        Ok((name, NonnaturalMundaneWeapon::OneHanded(one_handed_mundane)))
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
                .free_two_handed(WeaponName::Mundane(name))
                .ok_or(CharacterMutationError::WeaponError(WeaponError::NotFound))
                .and_then(|two_handed_equipped| match two_handed_equipped {
                    EquippedTwoHandedWeaponNoAttunement::Mundane(name, two_handed_mundane) => {
                        Ok((name, NonnaturalMundaneWeapon::TwoHanded(two_handed_mundane)))
                    }
                    EquippedTwoHandedWeaponNoAttunement::Artifact(_, _) => {
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
        name: &str,
        equipped: Equipped,
    ) -> Result<&mut Self, CharacterMutationError> {
        let (name, nonnatural_artifact) = match equipped {
            Equipped::Natural => Err(CharacterMutationError::WeaponError(
                WeaponError::UnequipNatural,
            )),
            Equipped::Worn => self
                .equipped
                .remove_worn_artifact(name)
                .ok_or(CharacterMutationError::WeaponError(WeaponError::NotFound))
                .map(|(name, worn_artifact)| {
                    (
                        name,
                        NonnaturalArtifactWeaponNoAttunement::Worn(worn_artifact),
                    )
                }),
            Equipped::MainHand => self
                .equipped
                .hands
                .free_hand(WeaponName::Artifact(name), EquipHand::MainHand)
                .ok_or(CharacterMutationError::WeaponError(WeaponError::NotFound))
                .and_then(|one_handed_equipped| match one_handed_equipped {
                    EquippedOneHandedWeaponNoAttunement::Artifact(name, one_handed_artifact) => {
                        Ok((
                            name,
                            NonnaturalArtifactWeaponNoAttunement::OneHanded(one_handed_artifact),
                        ))
                    }
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
                .free_hand(WeaponName::Artifact(name), EquipHand::OffHand)
                .ok_or(CharacterMutationError::WeaponError(WeaponError::NotFound))
                .and_then(|one_handed_equipped| match one_handed_equipped {
                    EquippedOneHandedWeaponNoAttunement::Artifact(name, one_handed_artifact) => {
                        Ok((
                            name,
                            NonnaturalArtifactWeaponNoAttunement::OneHanded(one_handed_artifact),
                        ))
                    }
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
                .free_two_handed(WeaponName::Artifact(name))
                .ok_or(CharacterMutationError::WeaponError(WeaponError::NotFound))
                .and_then(|two_handed_equipped| match two_handed_equipped {
                    EquippedTwoHandedWeaponNoAttunement::Artifact(name, two_handed_artifact) => {
                        Ok((
                            name,
                            NonnaturalArtifactWeaponNoAttunement::TwoHanded(two_handed_artifact),
                        ))
                    }
                    EquippedTwoHandedWeaponNoAttunement::Mundane(_, _) => {
                        // This shouldn't happen but if it does put it back and say not found instead
                        self.equipped.hands.set_two_handed(two_handed_equipped);
                        Err(CharacterMutationError::WeaponError(WeaponError::NotFound))
                    }
                }),
        }?;

        self.unequipped.stow_artifact(name, nonnatural_artifact)?;
        Ok(self)
    }

    pub fn add_artifact_weapon(
        &mut self,
        name: &'source str,
        weapon: ArtifactWeaponView<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        match weapon {
            ArtifactWeaponView::Natural(natural) => {
                if self.equipped.handless_artifact.contains_key(&name) {
                    Err(CharacterMutationError::WeaponError(
                        WeaponError::DuplicateArtifact,
                    ))
                } else if let Entry::Vacant(e) = self.equipped.handless_artifact.entry(name) {
                    e.insert(HandlessArtifactWeaponNoAttunement::Natural(natural));
                    Ok(self)
                } else {
                    Err(CharacterMutationError::WeaponError(
                        WeaponError::DuplicateArtifact,
                    ))
                }
            }
            ArtifactWeaponView::Worn(worn, _) => {
                if self.equipped.handless_artifact.contains_key(&name) {
                    Err(CharacterMutationError::WeaponError(
                        WeaponError::DuplicateArtifact,
                    ))
                } else {
                    self.unequipped
                        .stow_artifact(name, NonnaturalArtifactWeaponNoAttunement::Worn(worn))?;
                    Ok(self)
                }
            }
            ArtifactWeaponView::OneHanded(one_handed, _) => {
                if self
                    .equipped
                    .hands
                    .get_weapon(WeaponName::Artifact(name), Equipped::MainHand)
                    .is_some()
                    || self
                        .equipped
                        .hands
                        .get_weapon(WeaponName::Artifact(name), Equipped::OffHand)
                        .is_some()
                {
                    Err(CharacterMutationError::WeaponError(
                        WeaponError::DuplicateArtifact,
                    ))
                } else {
                    self.unequipped.stow_artifact(
                        name,
                        NonnaturalArtifactWeaponNoAttunement::OneHanded(one_handed),
                    )?;
                    Ok(self)
                }
            }
            ArtifactWeaponView::TwoHanded(two_handed, _) => {
                if self
                    .equipped
                    .hands
                    .get_weapon(WeaponName::Artifact(name), Equipped::TwoHanded)
                    .is_some()
                {
                    Err(CharacterMutationError::WeaponError(
                        WeaponError::DuplicateArtifact,
                    ))
                } else {
                    self.unequipped.stow_artifact(
                        name,
                        NonnaturalArtifactWeaponNoAttunement::TwoHanded(two_handed),
                    )?;
                    Ok(self)
                }
            }
        }
    }

    pub fn slot_hearthstone(
        &mut self,
        artifact_weapon_name: &str,
        hearthstone_name: &'source str,
        unslotted: UnslottedHearthstone<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        let try_slot =
            self.unequipped
                .slot_hearthstone(artifact_weapon_name, hearthstone_name, unslotted);
        match try_slot {
            Ok(_) => Ok(self),
            Err(CharacterMutationError::WeaponError(WeaponError::NotFound)) => {
                self.equipped.slot_hearthstone(
                    artifact_weapon_name,
                    hearthstone_name,
                    unslotted,
                )?;
                Ok(self)
            }
            Err(e) => Err(e),
        }
    }

    pub fn unslot_hearthstone(
        &mut self,
        artifact_weapon_name: &str,
        hearthstone_name: &str,
    ) -> Result<(&'source str, UnslottedHearthstone<'source>), CharacterMutationError> {
        let try_unslotted = self
            .unequipped
            .unslot_hearthstone(artifact_weapon_name, hearthstone_name);
        match try_unslotted {
            Ok(unslotted) => Ok(unslotted),
            Err(CharacterMutationError::WeaponError(WeaponError::NotFound)) => self
                .equipped
                .unslot_hearthstone(artifact_weapon_name, hearthstone_name),
            Err(e) => Err(e),
        }
    }
}
