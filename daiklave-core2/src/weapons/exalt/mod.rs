use serde::{Serialize, Deserialize};

use self::{equipped::{ExaltEquippedWeapons, ExaltEquippedWeaponsMemo}, unequipped::{ExaltUnequippedWeapons, ExaltUnequippedWeaponsMemo}};

mod equipped;
mod hands;
mod unequipped;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct ExaltWeapons<'source> {
    equipped: ExaltEquippedWeapons<'source>,
    unequipped: ExaltUnequippedWeapons<'source>,
}

impl<'source> ExaltWeapons<'source> {
    pub fn as_memo(&'source self) -> ExaltWeaponsMemo {
        ExaltWeaponsMemo { 
            equipped: self.equipped.as_memo(),
            unequipped: self.unequipped.as_memo(),
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