use serde::{Deserialize, Serialize};

use crate::weapons::{
    equipped::{
        EquippedOneHandedWeapon, EquippedOneHandedWeaponNoAttunement,
        EquippedOneHandedWeaponNoAttunementMemo, EquippedTwoHandedWeapon,
        EquippedTwoHandedWeaponNoAttunement, EquippedTwoHandedWeaponNoAttunementMemo,
    },
    exalt::ExaltHands,
    EquipHand, Weapon, WeaponId,
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

    pub fn get_weapon(&'view self, weapon_id: WeaponId) -> Option<Weapon<'view, 'source>> {
        match self {
            MortalHands::Empty => None,
            MortalHands::MainHand(one) => one.get_weapon(weapon_id, EquipHand::MainHand),
            MortalHands::OffHand(one) => one.get_weapon(weapon_id, EquipHand::OffHand),
            MortalHands::Both(arr) => arr[0]
                .get_weapon(weapon_id, EquipHand::MainHand)
                .or_else(|| arr[1].get_weapon(weapon_id, EquipHand::OffHand)),
            MortalHands::TwoHanded(two) => two.get_weapon(weapon_id),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = WeaponId> {
        match self {
            MortalHands::Empty => vec![],
            MortalHands::MainHand(one) => one.iter().collect::<Vec<WeaponId>>(),
            MortalHands::OffHand(one) => one.iter().collect::<Vec<WeaponId>>(),
            MortalHands::Both(arr) => arr[0]
                .iter()
                .chain(arr[1].iter())
                .collect::<Vec<WeaponId>>(),
            MortalHands::TwoHanded(two) => two.iter().collect::<Vec<WeaponId>>(),
        }
        .into_iter()
    }
}

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
