mod memo;
pub use memo::ExaltHandsMemo;

use crate::{
    exaltation::mortal::MortalHands,
    weapons::weapon::{
        equipped::{EquipHand, EquippedOneHandedWeapon, EquippedTwoHandedWeapon},
        Weapon, WeaponId, Equipped,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExaltHands<'source> {
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

    pub fn get_weapon(&'view self, weapon_id: WeaponId) -> Option<Weapon<'source>> {
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

    pub fn iter(&self) -> impl Iterator<Item = (WeaponId, Option<Equipped>)> + '_ {
        match self {
            ExaltHands::Empty => vec![],
            ExaltHands::MainHand(one) => one.iter().map(|id| (id, Some(Equipped::MainHand))).collect::<Vec<(WeaponId, Option<Equipped>)>>(),
            ExaltHands::OffHand(one) => one.iter().map(|id| (id, Some(Equipped::OffHand))).collect::<Vec<(WeaponId, Option<Equipped>)>>(),
            ExaltHands::Both(arr) => arr[0]
                .iter()
                .map(|id| (id, Some(Equipped::MainHand)))
                .chain(arr[1].iter().map(|id| (id, Some(Equipped::OffHand))))
                .collect::<Vec<(WeaponId, Option<Equipped>)>>(),
            ExaltHands::TwoHanded(two) => two.iter().map(|id| (id, Some(Equipped::TwoHanded))).collect::<Vec<(WeaponId, Option<Equipped>)>>(),
        }
        .into_iter()
    }
}
