use serde::{Deserialize, Serialize};

use super::ExaltHands;

use crate::weapons::weapon::equipped::{EquippedOneHandedWeaponMemo, EquippedTwoHandedWeaponMemo};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExaltHandsMemo {
    Empty,
    MainHand(EquippedOneHandedWeaponMemo),
    OffHand(EquippedOneHandedWeaponMemo),
    Both(Box<[EquippedOneHandedWeaponMemo; 2]>),
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
