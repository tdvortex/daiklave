use std::collections::HashMap;

use crate::{
    health::{DamageLevel, Health, WoundPenalty},
    Character, CharacterMutationError,
};

impl<'source> Character<'source> {
    /// Gets the character's health state (read-only).
    pub fn health(&self) -> &Health {
        &self.health
    }

    /// Sets the character to have the specified number of each type of health 
    /// box.
    pub fn set_health_track(
        &mut self,
        new_wound_penalties: &HashMap<WoundPenalty, u8>
    ) -> Result<&mut Self, CharacterMutationError> {
        todo!()
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

    /// Heals a character for the specified amount of damage (capped at the
    /// amount of damage they actually have). Bashing heals before lethal which
    /// heals before aggravated.
    pub fn heal_damage(&mut self, amount: u8) -> Result<&mut Self, CharacterMutationError> {
        self.health.heal_damage(amount)?;
        Ok(self)
    }
}
