use serde::{Serialize, Deserialize};

use crate::weapons::{equipped::{EquippedOneHandedWeaponNoAttunement, EquippedTwoHandedWeaponNoAttunement, EquippedOneHandedWeaponNoAttunementMemo, EquippedTwoHandedWeaponNoAttunementMemo, EquippedOneHandedWeapon, EquippedTwoHandedWeapon}, exalt::ExaltHands, WeaponId, Weapon, EquipHand};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::weapons) enum MortalHands<'source> {
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
            ExaltHands::MainHand(attuned) => MortalHands::MainHand(
                match attuned {
                    EquippedOneHandedWeapon::Mundane(id, mundane) => EquippedOneHandedWeaponNoAttunement::Mundane(id, mundane),
                    EquippedOneHandedWeapon::Artifact(id, artifact, _) => EquippedOneHandedWeaponNoAttunement::Artifact(id, artifact),
                }
            ),
            ExaltHands::OffHand(attuned) => MortalHands::OffHand(
                match attuned {
                    EquippedOneHandedWeapon::Mundane(id, mundane) => EquippedOneHandedWeaponNoAttunement::Mundane(id, mundane),
                    EquippedOneHandedWeapon::Artifact(id, artifact, _) => EquippedOneHandedWeaponNoAttunement::Artifact(id, artifact),
                }
            ),
            ExaltHands::Both(arr) => MortalHands::Both(arr.map(|attuned| {
                match attuned {
                    EquippedOneHandedWeapon::Mundane(id, mundane) => EquippedOneHandedWeaponNoAttunement::Mundane(id, mundane),
                    EquippedOneHandedWeapon::Artifact(id, artifact, _) => EquippedOneHandedWeaponNoAttunement::Artifact(id, artifact),
                }
            })),
            ExaltHands::TwoHanded(attuned) => MortalHands::TwoHanded(
                match attuned {
                    EquippedTwoHandedWeapon::Mundane(id, mundane) => EquippedTwoHandedWeaponNoAttunement::Mundane(id, mundane),
                    EquippedTwoHandedWeapon::Artifact(id, artifact, _) => EquippedTwoHandedWeaponNoAttunement::Artifact(id, artifact),
                }
            ),
        }
    }
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

    pub fn get_weapon(&self, weapon_id: WeaponId) -> Option<Weapon<'source>> {
        match self {
            MortalHands::Empty => None,
            MortalHands::MainHand(one) => one.get_weapon(weapon_id, EquipHand::MainHand),
            MortalHands::OffHand(one) => one.get_weapon(weapon_id, EquipHand::OffHand),
            MortalHands::Both(arr) => {
                arr[0].get_weapon(weapon_id, EquipHand::MainHand).or_else(|| arr[1].get_weapon(weapon_id, EquipHand::OffHand))
            }
            MortalHands::TwoHanded(two) => two.get_weapon(weapon_id),
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