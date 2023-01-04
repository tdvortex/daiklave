use serde::{Serialize, Deserialize};

use crate::weapons::{equipped::{EquippedOneHandedWeapon, EquippedTwoHandedWeapon, EquippedOneHandedWeaponMemo, EquippedTwoHandedWeaponMemo}, mortal::MortalHands, WeaponId, Weapon, EquipHand};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::weapons) enum ExaltHands<'source> {
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

impl<'source> ExaltHands<'source> {
    pub fn as_memo(&self) -> ExaltHandsMemo {
        match self {
            ExaltHands::Empty => ExaltHandsMemo::Empty,
            ExaltHands::MainHand(view) => ExaltHandsMemo::MainHand(view.as_memo()),
            ExaltHands::OffHand(view) => ExaltHandsMemo::OffHand(view.as_memo()),
            ExaltHands::Both(arr) => ExaltHandsMemo::Both(arr.map(|el| el.as_memo())),
            ExaltHands::TwoHanded(view) => ExaltHandsMemo::TwoHanded(view.as_memo()),
        }
    }

    pub fn get_weapon(&self, weapon_id: WeaponId) -> Option<Weapon<'source>> {
        match self {
            ExaltHands::Empty => None,
            ExaltHands::MainHand(one) => one.get_weapon(weapon_id, EquipHand::MainHand),
            ExaltHands::OffHand(one) => one.get_weapon(weapon_id, EquipHand::OffHand),
            ExaltHands::Both(arr) => {
                arr[0].get_weapon(weapon_id, EquipHand::MainHand).or_else(|| arr[1].get_weapon(weapon_id, EquipHand::OffHand))
            }
            ExaltHands::TwoHanded(two) => two.get_weapon(weapon_id),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(in crate::weapons::exalt) enum ExaltHandsMemo {
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
            ExaltHandsMemo::Both(arr) => ExaltHands::Both(arr.map(|el| el.as_ref())),
            ExaltHandsMemo::TwoHanded(memo) => ExaltHands::TwoHanded(memo.as_ref()),
        }
    }
}