mod memo;
pub(crate) use memo::ExaltHandsMemo;

use crate::{
    exaltation::mortal::MortalHands,
    weapons::weapon::{
        equipped::{EquipHand, EquippedOneHandedWeapon, EquippedTwoHandedWeapon},
        Equipped, Weapon, WeaponName,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ExaltHands<'source> {
    Empty,
    MainHand(EquippedOneHandedWeapon<'source>),
    OffHand(EquippedOneHandedWeapon<'source>),
    Both([EquippedOneHandedWeapon<'source>; 2]),
    TwoHanded(EquippedTwoHandedWeapon<'source>),
}

impl<'source> From<MortalHands<'source>> for ExaltHands<'source> {
    fn from(hands: MortalHands<'source>) -> Self {
        match hands {
            MortalHands::Empty => ExaltHands::Empty,
            MortalHands::MainHand(unattuned) => ExaltHands::MainHand(unattuned.into()),
            MortalHands::OffHand(unattuned) => ExaltHands::OffHand(unattuned.into()),
            MortalHands::Both(arr) => ExaltHands::Both(arr.map(|unattuned| unattuned.into())),
            MortalHands::TwoHanded(unattuned) => ExaltHands::TwoHanded(unattuned.into()),
        }
    }
}

impl<'source> Default for ExaltHands<'source> {
    fn default() -> Self {
        ExaltHands::Empty
    }
}

impl<'view, 'source> ExaltHands<'source> {
    pub fn as_memo(&self) -> ExaltHandsMemo {
        match self {
            ExaltHands::Empty => ExaltHandsMemo::Empty,
            ExaltHands::MainHand(view) => ExaltHandsMemo::MainHand(view.as_memo()),
            ExaltHands::OffHand(view) => ExaltHandsMemo::OffHand(view.as_memo()),
            ExaltHands::Both(arr) => ExaltHandsMemo::Both(Box::new(
                arr.iter()
                    .map(|el| el.as_memo())
                    .enumerate()
                    .fold([None, None], |mut opt_arr, (i, memo)| {
                        opt_arr[i] = Some(memo);
                        opt_arr
                    })
                    .map(|opt| opt.unwrap()),
            )),
            ExaltHands::TwoHanded(view) => ExaltHandsMemo::TwoHanded(view.as_memo()),
        }
    }

    pub fn get_weapon(
        &self,
        name: WeaponName<'_>,
        equipped: Equipped,
    ) -> Option<Weapon<'source>> {
        match (self, equipped) {
            (ExaltHands::Empty, _) | (_, Equipped::Natural) | (_, Equipped::Worn) => None,
            (ExaltHands::MainHand(one), Equipped::MainHand) => {
                one.get_weapon(name, EquipHand::MainHand)
            }
            (ExaltHands::OffHand(one), Equipped::OffHand) => {
                one.get_weapon(name, EquipHand::OffHand)
            }
            (ExaltHands::TwoHanded(two), Equipped::TwoHanded) => two.get_weapon(name),
            (ExaltHands::Both(arr), Equipped::MainHand) => {
                arr[0].get_weapon(name, EquipHand::MainHand)
            }
            (ExaltHands::Both(arr), Equipped::OffHand) => {
                arr[1].get_weapon(name, EquipHand::MainHand)
            }
            _ => None,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (WeaponName<'source>, Option<Equipped>)> + '_ {
        match self {
            ExaltHands::Empty => vec![],
            ExaltHands::MainHand(one) => one
                .iter()
                .map(|name| (name, Some(Equipped::MainHand)))
                .collect::<Vec<(WeaponName, Option<Equipped>)>>(),
            ExaltHands::OffHand(one) => one
                .iter()
                .map(|name| (name, Some(Equipped::OffHand)))
                .collect::<Vec<(WeaponName, Option<Equipped>)>>(),
            ExaltHands::Both(arr) => arr[0]
                .iter()
                .map(|name| (name, Some(Equipped::MainHand)))
                .chain(arr[1].iter().map(|id| (id, Some(Equipped::OffHand))))
                .collect::<Vec<(WeaponName, Option<Equipped>)>>(),
            ExaltHands::TwoHanded(two) => two
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
        one_handed_equipped: EquippedOneHandedWeapon<'source>,
        hand: EquipHand,
    ) {
        *self = match (&self, hand) {
            (ExaltHands::Empty, EquipHand::MainHand)
            | (ExaltHands::MainHand(_), EquipHand::MainHand)
            | (ExaltHands::TwoHanded(_), EquipHand::MainHand) => {
                ExaltHands::MainHand(one_handed_equipped)
            }
            (ExaltHands::Empty, EquipHand::OffHand)
            | (ExaltHands::OffHand(_), EquipHand::OffHand)
            | (ExaltHands::TwoHanded(_), EquipHand::OffHand) => {
                ExaltHands::OffHand(one_handed_equipped)
            }
            (ExaltHands::OffHand(current_off), EquipHand::MainHand) => {
                ExaltHands::Both([one_handed_equipped, current_off.clone()])
            }
            (ExaltHands::MainHand(current_main), EquipHand::OffHand) => {
                ExaltHands::Both([current_main.clone(), one_handed_equipped])
            }
            (ExaltHands::Both(arr), EquipHand::MainHand) => {
                ExaltHands::Both([one_handed_equipped, arr[1].clone()])
            }
            (ExaltHands::Both(arr), EquipHand::OffHand) => {
                ExaltHands::Both([arr[0].clone(), one_handed_equipped])
            }
        }
    }

    /// WARNING: If any weapons are currently equipped, they will be deleted
    /// forever!
    pub fn set_two_handed(&mut self, two_handed_equipped: EquippedTwoHandedWeapon<'source>) {
        *self = ExaltHands::TwoHanded(two_handed_equipped)
    }

    pub fn free_hand(
        &mut self,
        weapon_name: WeaponName<'_>,
        hand: EquipHand,
    ) -> Option<EquippedOneHandedWeapon<'source>> {
        match (&self, hand) {
            (ExaltHands::Empty, _)
            | (ExaltHands::TwoHanded(_), _)
            | (ExaltHands::MainHand(_), EquipHand::OffHand)
            | (ExaltHands::OffHand(_), EquipHand::MainHand) => None,
            (ExaltHands::MainHand(one_handed_equipped), EquipHand::MainHand) => {
                match (one_handed_equipped, weapon_name) {
                    (
                        EquippedOneHandedWeapon::Mundane(actual_name, _),
                        WeaponName::Mundane(wanted_name),
                    ) => {
                        if *actual_name == wanted_name {
                            let one_handed_equipped = one_handed_equipped.clone();
                            *self = ExaltHands::Empty;
                            Some(one_handed_equipped)
                        } else {
                            None
                        }
                    }
                    (
                        EquippedOneHandedWeapon::Artifact(actual_name, _, _),
                        WeaponName::Artifact(wanted_name),
                    ) => {
                        if *actual_name == wanted_name {
                            let one_handed_equipped = one_handed_equipped.clone();
                            *self = ExaltHands::Empty;
                            Some(one_handed_equipped)
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            }
            (ExaltHands::OffHand(one_handed_equipped), EquipHand::OffHand) => {
                match (one_handed_equipped, weapon_name) {
                    (
                        EquippedOneHandedWeapon::Mundane(actual_name, _),
                        WeaponName::Mundane(wanted_name),
                    ) => {
                        if *actual_name == wanted_name {
                            let one_handed_equipped = one_handed_equipped.clone();
                            *self = ExaltHands::Empty;
                            Some(one_handed_equipped)
                        } else {
                            None
                        }
                    }
                    (
                        EquippedOneHandedWeapon::Artifact(actual_name, _, _),
                        WeaponName::Artifact(wanted_name),
                    ) => {
                        if *actual_name == wanted_name {
                            let one_handed_equipped = one_handed_equipped.clone();
                            *self = ExaltHands::Empty;
                            Some(one_handed_equipped)
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            }
            (ExaltHands::Both(arr), EquipHand::MainHand) => {
                let one_handed_equipped = &arr[0];
                match (one_handed_equipped, weapon_name) {
                    (
                        EquippedOneHandedWeapon::Mundane(actual_name, _),
                        WeaponName::Mundane(wanted_name),
                    ) => {
                        if *actual_name == wanted_name {
                            let one_handed_equipped = one_handed_equipped.clone();
                            *self = ExaltHands::OffHand(arr[1].clone());
                            Some(one_handed_equipped)
                        } else {
                            None
                        }
                    }
                    (
                        EquippedOneHandedWeapon::Artifact(actual_name, _, _),
                        WeaponName::Artifact(wanted_name),
                    ) => {
                        if *actual_name == wanted_name {
                            let one_handed_equipped = one_handed_equipped.clone();
                            *self = ExaltHands::OffHand(arr[1].clone());
                            Some(one_handed_equipped)
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            }
            (ExaltHands::Both(arr), EquipHand::OffHand) => {
                let one_handed_equipped = &arr[1];
                match (one_handed_equipped, weapon_name) {
                    (
                        EquippedOneHandedWeapon::Mundane(actual_name, _),
                        WeaponName::Mundane(wanted_name),
                    ) => {
                        if *actual_name == wanted_name {
                            let one_handed_equipped = one_handed_equipped.clone();
                            *self = ExaltHands::MainHand(arr[0].clone());
                            Some(one_handed_equipped)
                        } else {
                            None
                        }
                    }
                    (
                        EquippedOneHandedWeapon::Artifact(actual_name, _, _),
                        WeaponName::Artifact(wanted_name),
                    ) => {
                        if *actual_name == wanted_name {
                            let one_handed_equipped = one_handed_equipped.clone();
                            *self = ExaltHands::MainHand(arr[0].clone());
                            Some(one_handed_equipped)
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
        weapon_id: WeaponName<'_>,
    ) -> Option<EquippedTwoHandedWeapon<'source>> {
        if let ExaltHands::TwoHanded(two_handed_equipped) = self {
            match (&two_handed_equipped, weapon_id) {
                (EquippedTwoHandedWeapon::Mundane(actual_name, _), WeaponName::Mundane(wanted_name)) => {
                    if *actual_name == wanted_name {
                        let two_handed_equipped = two_handed_equipped.clone();
                        *self = ExaltHands::Empty;
                        Some(two_handed_equipped)
                    } else {
                        None
                    }
                }
                (
                    EquippedTwoHandedWeapon::Artifact(actual_name, _, _),
                    WeaponName::Artifact(wanted_name),
                ) => {
                    if *actual_name == wanted_name {
                        let two_handed_equipped = two_handed_equipped.clone();
                        *self = ExaltHands::Empty;
                        Some(two_handed_equipped)
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
