use serde::{Deserialize, Serialize};

use crate::weapons::{
    equipped::{
        EquippedOneHandedWeapon, EquippedOneHandedWeaponMemo, EquippedTwoHandedWeapon,
        EquippedTwoHandedWeaponMemo,
    },
    mortal::MortalHands,
    EquipHand, Weapon, WeaponId,
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
            ExaltHands::Both(arr) => ExaltHandsMemo::Both(
                arr.iter()
                    .map(|el| el.as_memo())
                    .enumerate()
                    .fold([None, None], |mut opt_arr, (i, memo)| {
                        opt_arr[i] = Some(memo);
                        opt_arr
                    })
                    .map(|opt| opt.unwrap()),
            ),
            ExaltHands::TwoHanded(view) => ExaltHandsMemo::TwoHanded(view.as_memo()),
        }
    }

    pub fn get_weapon(&'view self, weapon_id: WeaponId) -> Option<Weapon<'view, 'source>> {
        match self {
            ExaltHands::Empty => None,
            ExaltHands::MainHand(one) => one.get_weapon(weapon_id, EquipHand::MainHand),
            ExaltHands::OffHand(one) => one.get_weapon(weapon_id, EquipHand::OffHand),
            ExaltHands::Both(arr) => arr[0]
                .get_weapon(weapon_id, EquipHand::MainHand)
                .or_else(|| arr[1].get_weapon(weapon_id, EquipHand::OffHand)),
            ExaltHands::TwoHanded(two) => two.get_weapon(weapon_id),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = WeaponId> + '_ {
        match self {
            ExaltHands::Empty => vec![],
            ExaltHands::MainHand(one) => one.iter().collect::<Vec<WeaponId>>(),
            ExaltHands::OffHand(one) => one.iter().collect::<Vec<WeaponId>>(),
            ExaltHands::Both(arr) => arr[0]
                .iter()
                .chain(arr[1].iter())
                .collect::<Vec<WeaponId>>(),
            ExaltHands::TwoHanded(two) => two.iter().collect::<Vec<WeaponId>>(),
        }
        .into_iter()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum ExaltHandsMemo {
    Empty,
    MainHand(EquippedOneHandedWeaponMemo),
    OffHand(EquippedOneHandedWeaponMemo),
    Both([EquippedOneHandedWeaponMemo; 2]),
    TwoHanded(EquippedTwoHandedWeaponMemo),
}

impl<'source> ExaltHandsMemo {
    pub fn as_ref(&'source self) -> ExaltHands<'source> {
        match self {
            ExaltHandsMemo::Empty => ExaltHands::Empty,
            ExaltHandsMemo::MainHand(memo) => ExaltHands::MainHand(memo.as_ref()),
            ExaltHandsMemo::OffHand(memo) => ExaltHands::OffHand(memo.as_ref()),
            ExaltHandsMemo::Both(arr) => ExaltHands::Both(
                arr.iter()
                    .map(|el| el.as_ref())
                    .enumerate()
                    .fold([None, None], |mut opt_arr, (i, reffed)| {
                        opt_arr[i] = Some(reffed);
                        opt_arr
                    })
                    .map(|opt| opt.unwrap()),
            ),
            ExaltHandsMemo::TwoHanded(memo) => ExaltHands::TwoHanded(memo.as_ref()),
        }
    }
}
