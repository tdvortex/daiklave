use serde::{Deserialize, Serialize};

use crate::weapons::weapon::equipped::{
    EquippedOneHandedWeaponNoAttunementMemo, EquippedTwoHandedWeaponNoAttunementMemo,
};

use super::MortalHands;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum MortalHandsMemo {
    Empty,
    MainHand(EquippedOneHandedWeaponNoAttunementMemo),
    OffHand(EquippedOneHandedWeaponNoAttunementMemo),
    Both(Box<[EquippedOneHandedWeaponNoAttunementMemo; 2]>),
    TwoHanded(EquippedTwoHandedWeaponNoAttunementMemo),
}

impl<'source> MortalHandsMemo {
    pub fn as_ref(&'source self) -> MortalHands<'source> {
        match self {
            MortalHandsMemo::Empty => MortalHands::Empty,
            MortalHandsMemo::MainHand(memo) => MortalHands::MainHand(memo.as_ref()),
            MortalHandsMemo::OffHand(memo) => MortalHands::OffHand(memo.as_ref()),
            MortalHandsMemo::Both(arr) => MortalHands::Both(
                arr.iter()
                    .map(|el| el.as_ref())
                    .enumerate()
                    .fold([None, None], |mut opt_arr, (i, memo)| {
                        opt_arr[i] = Some(memo);
                        opt_arr
                    })
                    .map(|opt| opt.unwrap()),
            ),
            MortalHandsMemo::TwoHanded(memo) => MortalHands::TwoHanded(memo.as_ref()),
        }
    }
}
