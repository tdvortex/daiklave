use serde::{Serialize, Deserialize};

use crate::weapons::equipped::{EquippedOneHandedWeaponNoAttunement, EquippedTwoHandedWeaponNoAttunement, EquippedOneHandedWeaponNoAttunementMemo, EquippedTwoHandedWeaponNoAttunementMemo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::weapons) enum MortalHands<'source> {
    Empty,
    MainHand(EquippedOneHandedWeaponNoAttunement<'source>),
    OffHand(EquippedOneHandedWeaponNoAttunement<'source>),
    Both([EquippedOneHandedWeaponNoAttunement<'source>; 2]),
    TwoHanded(EquippedTwoHandedWeaponNoAttunement<'source>),
}

impl<'source> Default for MortalHands<'source> {
    fn default() -> Self {
        Self::Empty
    }
}

impl<'source> MortalHands<'source> {
    pub fn as_memo(&self) -> MortalHandsMemo {
        match self {
            MortalHands::Empty => MortalHandsMemo::Empty,
            MortalHands::MainHand(view) => MortalHandsMemo::MainHand(view.as_memo()),
            MortalHands::OffHand(view) => MortalHandsMemo::OffHand(view.as_memo()),
            MortalHands::Both(arr) => MortalHandsMemo::Both(arr.map(|view| view.as_memo())),
            MortalHands::TwoHanded(view) => MortalHandsMemo::TwoHanded(view.as_memo()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(in crate::weapons::mortal) enum MortalHandsMemo {
    Empty,
    MainHand(EquippedOneHandedWeaponNoAttunementMemo),
    OffHand(EquippedOneHandedWeaponNoAttunementMemo),
    Both([EquippedOneHandedWeaponNoAttunementMemo; 2]),
    TwoHanded(EquippedTwoHandedWeaponNoAttunementMemo),
}

impl<'source> MortalHandsMemo {
    pub fn as_ref(&'source self) -> MortalHands<'source> {
        match self {
            MortalHandsMemo::Empty => MortalHands::Empty,
            MortalHandsMemo::MainHand(memo) => MortalHands::MainHand(memo.as_ref()),
            MortalHandsMemo::OffHand(memo) => MortalHands::OffHand(memo.as_ref()),
            MortalHandsMemo::Both(arr) => MortalHands::Both(arr.map(|memo| memo.as_ref())),
            MortalHandsMemo::TwoHanded(memo) => MortalHands::TwoHanded(memo.as_ref()),
        }
    }
}