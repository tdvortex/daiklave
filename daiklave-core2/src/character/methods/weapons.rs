use crate::{
    attributes::AttributeName,
    weapons::{
        weapon::{
            mundane::AddMundaneWeapon, AttackRange, EquipHand, Equipped, WeaponName,
            WeaponWeightClass,
        },
        WeaponError, Weapons,
    },
    Character, CharacterMutationError,
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
        add_mundane_weapon: &'source AddMundaneWeapon,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation
            .add_mundane_weapon(&add_mundane_weapon.name, &add_mundane_weapon.weapon)?;
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
        name: WeaponName<'_>,
        hand: Option<EquipHand>,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.check_equip_weapon(name, hand)?;
        self.exaltation.equip_weapon(name, hand)?;
        Ok(self)
    }

    fn check_equip_weapon(
        &self,
        name: WeaponName<'_>,
        hand: Option<EquipHand>,
    ) -> Result<(), CharacterMutationError> {
        if let Some(weapon) = self.weapons().get(name, None) {
            if weapon.is_natural() {
                Err(CharacterMutationError::WeaponError(
                    WeaponError::EquipNatural,
                ))
            } else if weapon.is_worn() && self.weapons().get(name, Some(Equipped::Worn)).is_some() {
                Err(CharacterMutationError::WeaponError(
                    WeaponError::DuplicateEquippedWorn,
                ))
            } else if weapon.is_one_handed() && hand.is_none() {
                Err(CharacterMutationError::WeaponError(
                    WeaponError::HandRequired,
                ))
            } else if weapon.weight_class() == WeaponWeightClass::Heavy
                && weapon.damage(AttackRange::Melee).is_some()
                && self.attributes().dots(AttributeName::Strength).get() < 3
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
        weapon_name: WeaponName<'_>,
        equipped: Equipped,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.unequip_weapon(weapon_name, equipped)?;
        Ok(self)
    }

    /// Removes a mundane weapon from the character.
    pub fn remove_mundane_weapon(
        &mut self,
        weapon_name: &'view str,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.remove_mundane_weapon(weapon_name)?;
        Ok(self)
    }
}
