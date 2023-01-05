use serde::{Deserialize, Serialize};

use self::{equipped::ExaltEquippedWeaponsMemo, unequipped::ExaltUnequippedWeaponsMemo};

pub(in crate::weapons) use equipped::ExaltEquippedWeapons;
pub(in crate::weapons) use hands::ExaltHands;
pub(in crate::weapons) use unequipped::ExaltUnequippedWeapons;

use super::{mortal::MortalWeapons, Weapon, WeaponId};

mod equipped;
mod hands;
mod unequipped;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct ExaltWeapons<'source> {
    pub equipped: ExaltEquippedWeapons<'source>,
    pub unequipped: ExaltUnequippedWeapons<'source>,
}

impl<'view, 'source> ExaltWeapons<'source> {
    pub fn as_memo(&'source self) -> ExaltWeaponsMemo {
        ExaltWeaponsMemo {
            equipped: self.equipped.as_memo(),
            unequipped: self.unequipped.as_memo(),
        }
    }

    pub fn get_weapon(&'view self, weapon_id: WeaponId) -> Option<Weapon<'source>> {
        if matches!(weapon_id, WeaponId::Unarmed) {
            Some(crate::weapons::unarmed())
        } else {
            self.equipped
                .get_weapon(weapon_id)
                .or_else(|| self.unequipped.get_weapon(weapon_id))
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = WeaponId> + '_ {
        self.equipped.iter().chain(self.unequipped.iter())
    }
}

impl<'source> From<MortalWeapons<'source>> for ExaltWeapons<'source> {
    fn from(mortal: MortalWeapons<'source>) -> Self {
        Self {
            equipped: mortal.equipped.into(),
            unequipped: mortal.unequipped.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ExaltWeaponsMemo {
    equipped: ExaltEquippedWeaponsMemo,
    unequipped: ExaltUnequippedWeaponsMemo,
}

impl<'source> ExaltWeaponsMemo {
    pub fn as_ref(&'source self) -> ExaltWeapons<'source> {
        ExaltWeapons {
            equipped: self.equipped.as_ref(),
            unequipped: self.unequipped.as_ref(),
        }
    }
}
