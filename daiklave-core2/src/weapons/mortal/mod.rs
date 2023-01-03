use serde::{Serialize, Deserialize};

use self::{equipped::{MortalEquippedWeaponsMemo}, unequipped::{MortalUnequippedWeaponsMemo}};
pub(in crate::weapons) use equipped::MortalEquippedWeapons;
pub(in crate::weapons) use unequipped::MortalUnequippedWeapons;
pub(in crate::weapons) use hands::MortalHands;

mod equipped;
mod hands;
mod unequipped;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct MortalWeapons<'source> {
    pub equipped: MortalEquippedWeapons<'source>,
    pub unequipped: MortalUnequippedWeapons<'source>,
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