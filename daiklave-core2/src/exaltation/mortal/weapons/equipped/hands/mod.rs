mod memo;

pub(crate) use memo::MortalHandsMemo;

use crate::{
    exaltation::exalt::ExaltHands,
    weapons::weapon::{
        equipped::{
            EquipHand, EquippedOneHandedWeapon, EquippedOneHandedWeaponNoAttunement,
            EquippedTwoHandedWeapon, EquippedTwoHandedWeaponNoAttunement,
        },
        Equipped, Weapon, WeaponId,
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

impl<'view, 'source> MortalHands<'source> {
    pub fn as_memo(&self) -> MortalHandsMemo {
        match self {
            MortalHands::Empty => MortalHandsMemo::Empty,
            MortalHands::MainHand(view) => MortalHandsMemo::MainHand(view.as_memo()),
            MortalHands::OffHand(view) => MortalHandsMemo::OffHand(view.as_memo()),
            MortalHands::Both(arr) => MortalHandsMemo::Both(Box::new(
                arr.iter()
                    .map(|el| el.as_memo())
                    .enumerate()
                    .fold([None, None], |mut opt_arr, (i, memo)| {
                        opt_arr[i] = Some(memo);
                        opt_arr
                    })
                    .map(|opt| opt.unwrap()),
            )),
            MortalHands::TwoHanded(view) => MortalHandsMemo::TwoHanded(view.as_memo()),
        }
    }

    pub fn get_weapon(
        &'view self,
        weapon_id: WeaponId,
        equipped: Equipped,
    ) -> Option<Weapon<'source>> {
        match (self, equipped) {
            (MortalHands::Empty, _) | (_, Equipped::Natural) | (_, Equipped::Worn) => None,
            (MortalHands::MainHand(one), Equipped::MainHand) => {
                one.get_weapon(weapon_id, EquipHand::MainHand)
            }
            (MortalHands::OffHand(one), Equipped::OffHand) => {
                one.get_weapon(weapon_id, EquipHand::OffHand)
            }
            (MortalHands::TwoHanded(two), Equipped::TwoHanded) => two.get_weapon(weapon_id),
            (MortalHands::Both(arr), Equipped::MainHand) => {
                arr[0].get_weapon(weapon_id, EquipHand::MainHand)
            }
            (MortalHands::Both(arr), Equipped::OffHand) => {
                arr[1].get_weapon(weapon_id, EquipHand::MainHand)
            }
            _ => None,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (WeaponId, Option<Equipped>)> {
        match self {
            MortalHands::Empty => vec![],
            MortalHands::MainHand(one) => one
                .iter()
                .map(|id| (id, Some(Equipped::MainHand)))
                .collect::<Vec<(WeaponId, Option<Equipped>)>>(),
            MortalHands::OffHand(one) => one
                .iter()
                .map(|id| (id, Some(Equipped::OffHand)))
                .collect::<Vec<(WeaponId, Option<Equipped>)>>(),
            MortalHands::Both(arr) => arr[0]
                .iter()
                .map(|id| (id, Some(Equipped::MainHand)))
                .chain(arr[1].iter().map(|id| (id, Some(Equipped::OffHand))))
                .collect::<Vec<(WeaponId, Option<Equipped>)>>(),
            MortalHands::TwoHanded(two) => two
                .iter()
                .map(|id| (id, Some(Equipped::TwoHanded)))
                .collect::<Vec<(WeaponId, Option<Equipped>)>>(),
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
        weapon_id: WeaponId,
        hand: EquipHand,
    ) -> Option<EquippedOneHandedWeaponNoAttunement<'source>> {
        match (&self, hand) {
            (MortalHands::Empty, _)
            | (MortalHands::TwoHanded(_), _)
            | (MortalHands::MainHand(_), EquipHand::OffHand)
            | (MortalHands::OffHand(_), EquipHand::MainHand) => None,
            (MortalHands::MainHand(one_handed_equipped), EquipHand::MainHand) => {
                match (one_handed_equipped, weapon_id) {
                    (
                        EquippedOneHandedWeaponNoAttunement::Mundane(actual_id, _),
                        WeaponId::Mundane(wanted_id),
                    ) => {
                        if *actual_id == wanted_id {
                            let output = one_handed_equipped.clone();
                            *self = MortalHands::Empty;
                            Some(output)
                        } else {
                            None
                        }
                    }
                    (
                        EquippedOneHandedWeaponNoAttunement::Artifact(actual_id, _),
                        WeaponId::Artifact(wanted_id),
                    ) => {
                        if *actual_id == wanted_id {
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
                match (one_handed_equipped, weapon_id) {
                    (
                        EquippedOneHandedWeaponNoAttunement::Mundane(actual_id, _),
                        WeaponId::Mundane(wanted_id),
                    ) => {
                        if *actual_id == wanted_id {
                            let output = one_handed_equipped.clone();
                            *self = MortalHands::Empty;
                            Some(output)
                        } else {
                            None
                        }
                    }
                    (
                        EquippedOneHandedWeaponNoAttunement::Artifact(actual_id, _),
                        WeaponId::Artifact(wanted_id),
                    ) => {
                        if *actual_id == wanted_id {
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
                match (one_handed_equipped, weapon_id) {
                    (
                        EquippedOneHandedWeaponNoAttunement::Mundane(actual_id, _),
                        WeaponId::Mundane(wanted_id),
                    ) => {
                        if *actual_id == wanted_id {
                            let output = one_handed_equipped.clone();
                            *self = MortalHands::OffHand(arr[1].clone());
                            Some(output)
                        } else {
                            None
                        }
                    }
                    (
                        EquippedOneHandedWeaponNoAttunement::Artifact(actual_id, _),
                        WeaponId::Artifact(wanted_id),
                    ) => {
                        if *actual_id == wanted_id {
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
                match (one_handed_equipped, weapon_id) {
                    (
                        EquippedOneHandedWeaponNoAttunement::Mundane(actual_id, _),
                        WeaponId::Mundane(wanted_id),
                    ) => {
                        if *actual_id == wanted_id {
                            let output = one_handed_equipped.clone();
                            *self = MortalHands::MainHand(arr[0].clone());
                            Some(output)
                        } else {
                            None
                        }
                    }
                    (
                        EquippedOneHandedWeaponNoAttunement::Artifact(actual_id, _),
                        WeaponId::Artifact(wanted_id),
                    ) => {
                        if *actual_id == wanted_id {
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
        weapon_id: WeaponId,
    ) -> Option<EquippedTwoHandedWeaponNoAttunement<'source>> {
        if let MortalHands::TwoHanded(two_handed_equipped) = self {
            match (&two_handed_equipped, weapon_id) {
                (
                    EquippedTwoHandedWeaponNoAttunement::Mundane(actual_id, _),
                    WeaponId::Mundane(wanted_id),
                ) => {
                    if *actual_id == wanted_id {
                        let output = two_handed_equipped.clone();
                        *self = MortalHands::Empty;
                        Some(output)
                    } else {
                        None
                    }
                }
                (
                    EquippedTwoHandedWeaponNoAttunement::Artifact(actual_id, _),
                    WeaponId::Artifact(wanted_id),
                ) => {
                    if *actual_id == wanted_id {
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
