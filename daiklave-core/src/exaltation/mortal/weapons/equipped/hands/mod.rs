mod memo;

pub(crate) use memo::MortalHandsMemo;

use crate::{
    exaltation::exalt::ExaltHands,
    weapons::weapon::{
        equipped::{
            EquipHand, EquippedOneHandedWeapon, EquippedOneHandedWeaponNoAttunement,
            EquippedTwoHandedWeapon, EquippedTwoHandedWeaponNoAttunement,
        },
        Equipped, Weapon, WeaponName,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum MortalHands<'source> {
    Empty,
    MainHand(EquippedOneHandedWeaponNoAttunement<'source>),
    OffHand(EquippedOneHandedWeaponNoAttunement<'source>),
    Both([EquippedOneHandedWeaponNoAttunement<'source>; 2]),
    TwoHanded(EquippedTwoHandedWeaponNoAttunement<'source>),
}

impl<'source> From<ExaltHands<'source>> for MortalHands<'source> {
    fn from(hands: ExaltHands<'source>) -> Self {
        match hands {
            ExaltHands::Empty => MortalHands::Empty,
            ExaltHands::MainHand(attuned) => MortalHands::MainHand(match attuned {
                EquippedOneHandedWeapon::Mundane(id, mundane) => {
                    EquippedOneHandedWeaponNoAttunement::Mundane(id, mundane)
                }
                EquippedOneHandedWeapon::Artifact(id, artifact, _) => {
                    EquippedOneHandedWeaponNoAttunement::Artifact(id, artifact)
                }
            }),
            ExaltHands::OffHand(attuned) => MortalHands::OffHand(match attuned {
                EquippedOneHandedWeapon::Mundane(id, mundane) => {
                    EquippedOneHandedWeaponNoAttunement::Mundane(id, mundane)
                }
                EquippedOneHandedWeapon::Artifact(id, artifact, _) => {
                    EquippedOneHandedWeaponNoAttunement::Artifact(id, artifact)
                }
            }),
            ExaltHands::Both(arr) => MortalHands::Both(arr.map(|attuned| match attuned {
                EquippedOneHandedWeapon::Mundane(id, mundane) => {
                    EquippedOneHandedWeaponNoAttunement::Mundane(id, mundane)
                }
                EquippedOneHandedWeapon::Artifact(id, artifact, _) => {
                    EquippedOneHandedWeaponNoAttunement::Artifact(id, artifact)
                }
            })),
            ExaltHands::TwoHanded(attuned) => MortalHands::TwoHanded(match attuned {
                EquippedTwoHandedWeapon::Mundane(id, mundane) => {
                    EquippedTwoHandedWeaponNoAttunement::Mundane(id, mundane)
                }
                EquippedTwoHandedWeapon::Artifact(id, artifact, _) => {
                    EquippedTwoHandedWeaponNoAttunement::Artifact(id, artifact)
                }
            }),
        }
    }
}

impl<'source> Default for MortalHands<'source> {
    fn default() -> Self {
        Self::Empty
    }
}

impl<'source> From<&'source MortalHandsMemo> for MortalHands<'source> {
    fn from(value: &'source MortalHandsMemo) -> Self {
        match value {
            MortalHandsMemo::Empty => Self::Empty,
            MortalHandsMemo::MainHand(weapon) => Self::MainHand(weapon.into()),
            MortalHandsMemo::OffHand(weapon) => Self::OffHand(weapon.into()),
            MortalHandsMemo::Both(weapons) => Self::Both([(&weapons[0]).into(), (&weapons[1]).into()]),
            MortalHandsMemo::TwoHanded(weapon) => Self::TwoHanded(weapon.into()),
        }
    }
}

impl<'view, 'source> MortalHands<'source> {
    pub fn get_weapon(
        &'view self,
        name: WeaponName<'_>,
        equipped: Equipped,
    ) -> Option<Weapon<'source>> {
        match (self, equipped) {
            (MortalHands::Empty, _) | (_, Equipped::Natural) | (_, Equipped::Worn) => None,
            (MortalHands::MainHand(one), Equipped::MainHand) => {
                one.get_weapon(name, EquipHand::MainHand)
            }
            (MortalHands::OffHand(one), Equipped::OffHand) => {
                one.get_weapon(name, EquipHand::OffHand)
            }
            (MortalHands::TwoHanded(two), Equipped::TwoHanded) => two.get_weapon(name),
            (MortalHands::Both(arr), Equipped::MainHand) => {
                arr[0].get_weapon(name, EquipHand::MainHand)
            }
            (MortalHands::Both(arr), Equipped::OffHand) => {
                arr[1].get_weapon(name, EquipHand::MainHand)
            }
            _ => None,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (WeaponName<'source>, Option<Equipped>)> {
        match self {
            MortalHands::Empty => vec![],
            MortalHands::MainHand(one) => one
                .iter()
                .map(|name| (name, Some(Equipped::MainHand)))
                .collect::<Vec<(WeaponName, Option<Equipped>)>>(),
            MortalHands::OffHand(one) => one
                .iter()
                .map(|name| (name, Some(Equipped::OffHand)))
                .collect::<Vec<(WeaponName, Option<Equipped>)>>(),
            MortalHands::Both(arr) => arr[0]
                .iter()
                .map(|name| (name, Some(Equipped::MainHand)))
                .chain(arr[1].iter().map(|id| (id, Some(Equipped::OffHand))))
                .collect::<Vec<(WeaponName, Option<Equipped>)>>(),
            MortalHands::TwoHanded(two) => two
                .iter()
                .map(|name| (name, Some(Equipped::TwoHanded)))
                .collect::<Vec<(WeaponName, Option<Equipped>)>>(),
        }
        .into_iter()
    }

    /// WARNING: If you set a hand that is not empty, the existing weapon will
    /// be deleted forever!
    pub fn set_hand(
        &mut self,
        one_handed_equipped: EquippedOneHandedWeaponNoAttunement<'source>,
        hand: EquipHand,
    ) {
        *self = match (&self, hand) {
            (MortalHands::Empty, EquipHand::MainHand)
            | (MortalHands::MainHand(_), EquipHand::MainHand)
            | (MortalHands::TwoHanded(_), EquipHand::MainHand) => {
                MortalHands::MainHand(one_handed_equipped)
            }
            (MortalHands::Empty, EquipHand::OffHand)
            | (MortalHands::OffHand(_), EquipHand::OffHand)
            | (MortalHands::TwoHanded(_), EquipHand::OffHand) => {
                MortalHands::OffHand(one_handed_equipped)
            }
            (MortalHands::OffHand(current_off), EquipHand::MainHand) => {
                MortalHands::Both([one_handed_equipped, current_off.clone()])
            }
            (MortalHands::MainHand(current_main), EquipHand::OffHand) => {
                MortalHands::Both([current_main.clone(), one_handed_equipped])
            }
            (MortalHands::Both(arr), EquipHand::MainHand) => {
                MortalHands::Both([one_handed_equipped, arr[1].clone()])
            }
            (MortalHands::Both(arr), EquipHand::OffHand) => {
                MortalHands::Both([arr[0].clone(), one_handed_equipped])
            }
        };
    }

    /// WARNING: If any weapons are currently equipped, they will be deleted
    /// forever!
    pub fn set_two_handed(
        &mut self,
        two_handed_equipped: EquippedTwoHandedWeaponNoAttunement<'source>,
    ) {
        *self = MortalHands::TwoHanded(two_handed_equipped)
    }

    pub fn free_hand(
        &mut self,
        name: WeaponName<'_>,
        hand: EquipHand,
    ) -> Option<EquippedOneHandedWeaponNoAttunement<'source>> {
        match (&self, hand) {
            (MortalHands::Empty, _)
            | (MortalHands::TwoHanded(_), _)
            | (MortalHands::MainHand(_), EquipHand::OffHand)
            | (MortalHands::OffHand(_), EquipHand::MainHand) => None,
            (MortalHands::MainHand(one_handed_equipped), EquipHand::MainHand) => {
                match (one_handed_equipped, name) {
                    (
                        EquippedOneHandedWeaponNoAttunement::Mundane(actual_name, _),
                        WeaponName::Mundane(wanted_name),
                    ) => {
                        if *actual_name == wanted_name {
                            let output = one_handed_equipped.clone();
                            *self = MortalHands::Empty;
                            Some(output)
                        } else {
                            None
                        }
                    }
                    (
                        EquippedOneHandedWeaponNoAttunement::Artifact(actual_name, _),
                        WeaponName::Artifact(wanted_name),
                    ) => {
                        if *actual_name == wanted_name {
                            let output = one_handed_equipped.clone();
                            *self = MortalHands::Empty;
                            Some(output)
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            }
            (MortalHands::OffHand(one_handed_equipped), EquipHand::OffHand) => {
                match (one_handed_equipped, name) {
                    (
                        EquippedOneHandedWeaponNoAttunement::Mundane(actual_name, _),
                        WeaponName::Mundane(wanted_name),
                    ) => {
                        if *actual_name == wanted_name {
                            let output = one_handed_equipped.clone();
                            *self = MortalHands::Empty;
                            Some(output)
                        } else {
                            None
                        }
                    }
                    (
                        EquippedOneHandedWeaponNoAttunement::Artifact(actual_name, _),
                        WeaponName::Artifact(wanted_name),
                    ) => {
                        if *actual_name == wanted_name {
                            let output = one_handed_equipped.clone();
                            *self = MortalHands::Empty;
                            Some(output)
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            }
            (MortalHands::Both(arr), EquipHand::MainHand) => {
                let one_handed_equipped = &arr[0];
                match (one_handed_equipped, name) {
                    (
                        EquippedOneHandedWeaponNoAttunement::Mundane(actual_name, _),
                        WeaponName::Mundane(wanted_name),
                    ) => {
                        if *actual_name == wanted_name {
                            let output = one_handed_equipped.clone();
                            *self = MortalHands::OffHand(arr[1].clone());
                            Some(output)
                        } else {
                            None
                        }
                    }
                    (
                        EquippedOneHandedWeaponNoAttunement::Artifact(actual_name, _),
                        WeaponName::Artifact(wanted_name),
                    ) => {
                        if *actual_name == wanted_name {
                            let output = one_handed_equipped.clone();
                            *self = MortalHands::OffHand(arr[1].clone());
                            Some(output)
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            }
            (MortalHands::Both(arr), EquipHand::OffHand) => {
                let one_handed_equipped = &arr[1];
                match (one_handed_equipped, name) {
                    (
                        EquippedOneHandedWeaponNoAttunement::Mundane(actual_name, _),
                        WeaponName::Mundane(wanted_name),
                    ) => {
                        if *actual_name == wanted_name {
                            let output = one_handed_equipped.clone();
                            *self = MortalHands::MainHand(arr[0].clone());
                            Some(output)
                        } else {
                            None
                        }
                    }
                    (
                        EquippedOneHandedWeaponNoAttunement::Artifact(actual_name, _),
                        WeaponName::Artifact(wanted_name),
                    ) => {
                        if *actual_name == wanted_name {
                            let output = one_handed_equipped.clone();
                            *self = MortalHands::MainHand(arr[0].clone());
                            Some(output)
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            }
        }
    }

    pub fn free_two_handed(
        &mut self,
        name: WeaponName<'_>,
    ) -> Option<EquippedTwoHandedWeaponNoAttunement<'source>> {
        if let MortalHands::TwoHanded(two_handed_equipped) = self {
            match (&two_handed_equipped, name) {
                (
                    EquippedTwoHandedWeaponNoAttunement::Mundane(actual_name, _),
                    WeaponName::Mundane(wanted_name),
                ) => {
                    if *actual_name == wanted_name {
                        let output = two_handed_equipped.clone();
                        *self = MortalHands::Empty;
                        Some(output)
                    } else {
                        None
                    }
                }
                (
                    EquippedTwoHandedWeaponNoAttunement::Artifact(actual_name, _),
                    WeaponName::Artifact(wanted_name),
                ) => {
                    if *actual_name == wanted_name {
                        let output = two_handed_equipped.clone();
                        *self = MortalHands::Empty;
                        Some(output)
                    } else {
                        None
                    }
                }
                _ => None,
            }
        } else {
            None
        }
    }
}
