use serde::{Deserialize, Serialize};

use crate::CharacterMutationError;

use self::{equipped::MortalEquippedWeaponsMemo, unequipped::MortalUnequippedWeaponsMemo};
pub(in crate::weapons) use equipped::MortalEquippedWeapons;
pub(in crate::weapons) use hands::MortalHands;
pub(in crate::weapons) use unequipped::MortalUnequippedWeapons;

use super::{exalt::ExaltWeapons, Weapon, WeaponId, mundane::{MundaneWeapon, HandlessMundaneWeapon, NonnaturalMundaneWeapon, WornMundaneWeapon, OneHandedMundaneWeapon, TwoHandedMundaneWeapon}, BaseWeaponId};

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

impl<'view, 'source> MortalWeapons<'source> {
    pub fn as_memo(&self) -> MortalWeaponsMemo {
        MortalWeaponsMemo {
            equipped: self.equipped.as_memo(),
            unequipped: self.unequipped.as_memo(),
        }
    }

    pub fn get_weapon(&'view self, weapon_id: WeaponId) -> Option<Weapon<'source>> {
        if matches!(weapon_id, WeaponId::Unarmed) {
            Some(super::unarmed())
        } else {
            self.equipped
                .get_weapon(weapon_id)
                .or_else(|| self.unequipped.get_weapon(weapon_id))
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = WeaponId> + '_ {
        self.equipped.iter().chain(self.unequipped.iter())
    }

    pub fn add_mundane_weapon(&mut self, weapon_id: BaseWeaponId, weapon: &'source MundaneWeapon) -> Result<&mut Self, CharacterMutationError> {
        match weapon {
            MundaneWeapon::Natural(natural_mundane_weapon) => {
                self.equipped.add_natural_mundane_weapon(weapon_id, natural_mundane_weapon)?;
            }
            MundaneWeapon::Worn(worn_mundane_weapon, _) => {
                let nonnatural = NonnaturalMundaneWeapon::Worn(WornMundaneWeapon(&**worn_mundane_weapon));
                self.unequipped.add_mundane_weapon(weapon_id, nonnatural)?;
            }
            MundaneWeapon::OneHanded(one_handed_mundane_weapon, _) => {
                let nonnatural = NonnaturalMundaneWeapon::OneHanded(OneHandedMundaneWeapon(&**one_handed_mundane_weapon));
                self.unequipped.add_mundane_weapon(weapon_id, nonnatural)?;
            }
            MundaneWeapon::TwoHanded(two_handed_mundane_weapon, _) => {
                let nonnatural = NonnaturalMundaneWeapon::TwoHanded(TwoHandedMundaneWeapon(&**two_handed_mundane_weapon));
                self.unequipped.add_mundane_weapon(weapon_id, nonnatural)?;
            }
        }
        
        self.unequipped.add_mundane_weapon(weapon_id, weapon)?;
        Ok(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MortalWeaponsMemo {
    equipped: MortalEquippedWeaponsMemo,
    unequipped: MortalUnequippedWeaponsMemo,
}

impl<'source> MortalWeaponsMemo {
    pub fn as_ref(&'source self) -> MortalWeapons<'source> {
        MortalWeapons {
            equipped: self.equipped.as_ref(),
            unequipped: self.unequipped.as_ref(),
        }
    }
}
