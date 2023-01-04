use serde::{Serialize, Deserialize};

use self::{equipped::{MortalEquippedWeaponsMemo}, unequipped::{MortalUnequippedWeaponsMemo}};
pub(in crate::weapons) use equipped::MortalEquippedWeapons;
pub(in crate::weapons) use unequipped::MortalUnequippedWeapons;
pub(in crate::weapons) use hands::MortalHands;

use super::{exalt::ExaltWeapons, WeaponId, Weapon};

mod equipped;
mod hands;
mod unequipped;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct MortalWeapons<'source> {
    pub equipped: MortalEquippedWeapons<'source>,
    pub unequipped: MortalUnequippedWeapons<'source>,
}

impl<'source> From<ExaltWeapons<'source>> for MortalWeapons<'source> {
    fn from(exalt: ExaltWeapons<'source>) -> Self {
        Self {
            equipped: exalt.equipped.into(),
            unequipped: exalt.unequipped.into(),
        }
    }
}

impl<'source> MortalWeapons<'source> {
    pub fn as_memo(&self) -> MortalWeaponsMemo {
        MortalWeaponsMemo { equipped: self.equipped.as_memo(), unequipped: self.unequipped.as_memo() }
    }

    pub fn get_weapon(&self, weapon_id: WeaponId) -> Option<Weapon<'source>> {
        if matches!(weapon_id, WeaponId::Unarmed) {
            Some(super::unarmed())
        } else {
            self.equipped.get_weapon(weapon_id).or_else(|| self.unequipped.get_weapon(weapon_id))
        }        
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MortalWeaponsMemo {
    equipped: MortalEquippedWeaponsMemo,
    unequipped: MortalUnequippedWeaponsMemo,
}

impl<'source> MortalWeaponsMemo {
    pub fn as_ref(&'source self) -> MortalWeapons<'source> {
        MortalWeapons { equipped: self.equipped.as_ref(), unequipped: self.unequipped.as_ref() }
    }
}