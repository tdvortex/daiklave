use crate::{
    weapons::{
        weapon::{
            mundane::MundaneWeapon, BaseWeaponId, EquipHand, Equipped, WeaponId, WeaponWeightClass, AttackRange,
        },
        Weapons, WeaponError,
    },
    Character, CharacterMutationError, attributes::AttributeName,
};

impl<'view, 'source> Character<'source> {
    /// The character's Weapons.
    pub fn weapons(&'view self) -> Weapons<'view, 'source> {
        Weapons(&self.exaltation)
    }

    /// Adds a new mundane weapon to the character's arsenal. The weapon is
    /// initially unequipped, unless it is Natural, in which case it's
    /// immediately readied and available.
    pub fn add_mundane_weapon(
        &mut self,
        weapon_id: BaseWeaponId,
        weapon: &'source MundaneWeapon,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.add_mundane_weapon(weapon_id, weapon)?;
        Ok(self)
    }

    /// Equips a weapon. For mundane weapons, there must be at least 1
    /// unequipped copy of the weapon. For artifact weapons, the weapon must
    /// not be equipped. \n For a OneHanded weapon, the hand parameter is
    /// required and will unequip the weapon already in that hand. \n
    /// For Worn weapons, the hand parameter is ignored and will not unequip
    /// any weapons. \n For TwoHanded weapons, the hand parameter is ignored
    /// and all one- or two-handed weapons will be unequipped. \n
    /// For Natural weapons, will return an Err.
    pub fn equip_weapon(
        &mut self,
        weapon_id: WeaponId,
        hand: Option<EquipHand>,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.check_equip_weapon(weapon_id, hand)?;
        self.exaltation.equip_weapon(weapon_id, hand)?;
        Ok(self)
    }

    fn check_equip_weapon(
        &self,
        weapon_id: WeaponId,
        hand: Option<EquipHand>,
    ) -> Result<(), CharacterMutationError> {
        if let Some(weapon) = self.weapons().get(weapon_id, None) {
            if weapon.is_natural() {
                Err(CharacterMutationError::WeaponError(
                    WeaponError::EquipNatural,
                ))
            } else if weapon.is_worn()
                && self
                    .weapons()
                    .get(weapon_id, Some(Equipped::Worn))
                    .is_some()
            {
                Err(CharacterMutationError::WeaponError(
                    WeaponError::DuplicateEquippedWorn,
                ))
            } else if weapon.is_one_handed() && hand.is_none() {
                Err(CharacterMutationError::WeaponError(
                    WeaponError::HandRequired,
                ))
            } else if weapon.weight_class() == WeaponWeightClass::Heavy
                && weapon.damage(AttackRange::Melee).is_some()
                && self.attributes().dots(AttributeName::Strength) < 3
            {
                Err(CharacterMutationError::WeaponError(
                    WeaponError::HeavyMeleeStrengthRequirement,
                ))
            } else {
                Ok(())
            }
        } else {
            Err(CharacterMutationError::WeaponError(WeaponError::NotFound))
        }
    }

    /// Unequips a weapon. The equip location of the weapon must be
    /// specified to avoid ambiguity (in case of dual-wielding identical
    /// mundane weapons). Always Errs if Equipped is Natural, or if the
    /// requested weapon is not equipped at that location.
    pub fn unequip_weapon(
        &mut self,
        weapon_id: WeaponId,
        equipped: Equipped,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.unequip_weapon(weapon_id, equipped)?;
        Ok(self)
    }

    /// Removes a mundane weapon from the character.
    pub fn remove_mundane_weapon(
        &mut self,
        weapon_id: BaseWeaponId,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.remove_mundane_weapon(weapon_id)?;
        Ok(self)
    }
}
