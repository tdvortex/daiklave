use serde::{Serialize, Deserialize};

use self::{equipped::{ExaltEquippedWeapons, ExaltEquippedWeaponsMemo}, unequipped::{ExaltUnequippedWeapons, ExaltUnequippedWeaponsMemo}};

use super::mortal::MortalWeapons;

mod equipped;
mod hands;
mod unequipped;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct ExaltWeapons<'source> {
    pub equipped: ExaltEquippedWeapons<'source>,
    pub unequipped: ExaltUnequippedWeapons<'source>,
}

impl<'source> ExaltWeapons<'source> {
    pub fn as_memo(&'source self) -> ExaltWeaponsMemo {
        ExaltWeaponsMemo { 
            equipped: self.equipped.as_memo(),
            unequipped: self.unequipped.as_memo(),
        }
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