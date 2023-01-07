mod memo;

pub(crate) use memo::MortalHandsMemo;

use crate::{
    exaltation::exalt::ExaltHands,
    weapons::weapon::{
        equipped::{
            EquipHand, EquippedOneHandedWeapon, EquippedOneHandedWeaponNoAttunement,
            EquippedTwoHandedWeapon, EquippedTwoHandedWeaponNoAttunement,
        },
        Weapon, WeaponId, Equipped,
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

    pub fn get_weapon(&'view self, weapon_id: WeaponId, equipped: Equipped) -> Option<Weapon<'source>> {
        match (self, equipped) {
            (MortalHands::Empty, _) | (_, Equipped::Natural) | (_, Equipped::Worn) => None,
            (MortalHands::MainHand(one), Equipped::MainHand) => one.get_weapon(weapon_id, EquipHand::MainHand),
            (MortalHands::OffHand(one), Equipped::OffHand) => one.get_weapon(weapon_id, EquipHand::OffHand),
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
            MortalHands::MainHand(one) => one.iter().map(|id| (id, Some(Equipped::MainHand))).collect::<Vec<(WeaponId, Option<Equipped>)>>(),
            MortalHands::OffHand(one) => one.iter().map(|id| (id, Some(Equipped::OffHand))).collect::<Vec<(WeaponId, Option<Equipped>)>>(),
            MortalHands::Both(arr) => arr[0]
                .iter()
                .map(|id| (id, Some(Equipped::MainHand)))
                .chain(arr[1].iter().map(|id| (id, Some(Equipped::OffHand))))
                .collect::<Vec<(WeaponId, Option<Equipped>)>>(),
            MortalHands::TwoHanded(two) => two.iter().map(|id| (id, Some(Equipped::TwoHanded))).collect::<Vec<(WeaponId, Option<Equipped>)>>(),
        }
        .into_iter()
    }
}
