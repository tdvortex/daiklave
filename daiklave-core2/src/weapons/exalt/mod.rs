use serde::{Deserialize, Serialize};

use crate::CharacterMutationError;

use self::{equipped::ExaltEquippedWeaponsMemo, unequipped::ExaltUnequippedWeaponsMemo};

pub(in crate::weapons) use equipped::ExaltEquippedWeapons;
pub(in crate::weapons) use hands::ExaltHands;
pub(in crate::weapons) use unequipped::ExaltUnequippedWeapons;

use super::{mortal::MortalWeapons, Weapon, WeaponId, BaseWeaponId, MundaneWeaponMemo, mundane::{HandlessMundaneWeapon, NaturalMundaneWeapon, NonnaturalMundaneWeapon, WornMundaneWeapon}};

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

    pub fn add_mundane_weapon(&mut self, weapon_id: BaseWeaponId, weapon: &'source MundaneWeaponMemo) -> Result<&mut Self, CharacterMutationError> {
        let nonnatural_mundane = match weapon {
            MundaneWeaponMemo::Natural(weapon) => {
                let handless_mundane = HandlessMundaneWeapon::Natural(NaturalMundaneWeapon(&weapon.0));
                self.equipped.add_natural_mundane_weapon(weapon_id, handless_mundane)?;
                return Ok(self);
            }
            MundaneWeaponMemo::Worn(weapon, _) => NonnaturalMundaneWeapon::Worn(WornMundaneWeapon(&weapon.0)),
            MundaneWeaponMemo::OneHanded(weapon, _) => NonnaturalMundaneWeapon::Worn(WornMundaneWeapon(&weapon.0)),
            MundaneWeaponMemo::TwoHanded(weapon, _) => NonnaturalMundaneWeapon::Worn(WornMundaneWeapon(&weapon.0)),
        };

        self.unequipped.add_mundane_weapon(weapon_id, nonnatural_mundane)?;
        Ok(self)
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
