use crate::{CharacterMutationError, CharacterView, DamageLevel, Health, WoundPenalty};

impl<'source> CharacterView<'source> {
    /// Gets the character's health state (read-only).
    pub fn health(&self) -> &Health {
        &self.health
    }

    /// Checks if wound penalties can be set to a specific level.
    pub fn check_set_wound_penalties(
        &self,
        _new_wound_penalties: &[WoundPenalty],
    ) -> Result<(), CharacterMutationError> {
        Ok(())
    }

    /// Sets a character's health track to be the specified set of wound
    /// penalies. Additionally heals all damage.
    pub fn set_wound_penalties(
        &mut self,
        new_wound_penalties: &[WoundPenalty],
    ) -> Result<&mut Self, CharacterMutationError> {
        self.health.set_wound_penalties(new_wound_penalties)?;
        Ok(self)
    }

    /// Checks if character can be assigned an amount and type of damage.
    pub fn check_take_damage(
        &self,
        _damage_level: DamageLevel,
        _6amount: u8,
    ) -> Result<(), CharacterMutationError> {
        Ok(())
    }

    /// Adds damage to character (including overflow rollovers). Caps out at
    /// being full up with aggravated.
    pub fn take_damage(
        &mut self,
        damage_level: DamageLevel,
        amount: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.health.take_damage(damage_level, amount)?;
        Ok(self)
    }

    /// Checks if the character can heal the specified amount of damage.
    pub fn check_heal_damage(&self, _amount: u8) -> Result<(), CharacterMutationError> {
        Ok(())
    }

    /// Heals a character for the specified amount of damage (capped at the
    /// amount of damage they actually have). Bashing heals before lethal which
    /// heals before aggravated.
    pub fn heal_damage(&mut self, amount: u8) -> Result<&mut Self, CharacterMutationError> {
        self.health.heal_damage(amount)?;
        Ok(self)
    }
}
