use serde::{Serialize, Deserialize};

use self::{equipped::{MortalEquippedWeapons, MortalEquippedWeaponsMemo}, unequipped::{MortalUnequippedWeapons, MortalUnequippedWeaponsMemo}};

mod equipped;
mod hands;
mod unequipped;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct MortalWeapons<'source> {
    equipped: MortalEquippedWeapons<'source>,
    unequipped: MortalUnequippedWeapons<'source>,
}

impl<'source> MortalWeapons<'source> {
    pub fn as_memo(&self) -> MortalWeaponsMemo {
        MortalWeaponsMemo { equipped: self.equipped.as_memo(), unequipped: self.unequipped.as_memo() }
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