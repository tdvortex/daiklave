use crate::{
    exaltation::exalt::ExaltWeapons,
    weapons::weapon::{
        mundane::{
            HandlessMundaneWeapon, MundaneWeaponMemo, NaturalMundaneWeapon,
            NonnaturalMundaneWeapon, OneHandedMundaneWeapon, TwoHandedMundaneWeapon,
            WornMundaneWeapon,
        },
        BaseWeaponId, Weapon, WeaponId, Equipped,
    },
    CharacterMutationError,
};

mod equipped;
pub(crate) use equipped::{MortalEquippedWeapons, MortalHands};
mod unequipped;
pub(crate) use unequipped::MortalUnequippedWeapons;
mod memo;
pub(crate) use memo::MortalWeaponsMemo;

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

    pub fn get_weapon(&'view self, weapon_id: WeaponId, equipped: Option<Equipped>) -> Option<Weapon<'source>> {
        if matches!(weapon_id, WeaponId::Unarmed) {
            if matches!(equipped, Some(Equipped::Natural)) {
                Some(crate::weapons::weapon::mundane::unarmed())
            } else {
                None
            }
        } else {
            if let Some(equipped) = equipped {
                self.equipped.get_weapon(weapon_id, equipped)
            } else {
                self.unequipped.get_weapon(weapon_id)
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (WeaponId, Option<Equipped>)> + '_ {
        self.equipped.iter().chain(self.unequipped.iter())
    }

    pub fn add_mundane_weapon(
        &mut self,
        weapon_id: BaseWeaponId,
        weapon: &'source MundaneWeaponMemo,
    ) -> Result<&mut Self, CharacterMutationError> {
        let nonnatural_mundane = match weapon {
            MundaneWeaponMemo::Natural(weapon) => {
                let handless_mundane =
                    HandlessMundaneWeapon::Natural(NaturalMundaneWeapon(&weapon.0));
                self.equipped
                    .add_natural_mundane_weapon(weapon_id, handless_mundane)?;
                return Ok(self);
            }
            MundaneWeaponMemo::Worn(weapon, _) => {
                NonnaturalMundaneWeapon::Worn(WornMundaneWeapon(&weapon.0))
            }
            MundaneWeaponMemo::OneHanded(weapon, _) => {
                NonnaturalMundaneWeapon::OneHanded(OneHandedMundaneWeapon(&weapon.0))
            }
            MundaneWeaponMemo::TwoHanded(weapon, _) => {
                NonnaturalMundaneWeapon::TwoHanded(TwoHandedMundaneWeapon(&weapon.0))
            }
        };

        self.unequipped
            .add_mundane_weapon(weapon_id, nonnatural_mundane)?;
        Ok(self)
    }
}
