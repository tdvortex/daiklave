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
        new_wound_penalties: &HashMap<WoundPenalty, u8>,
    ) -> Result<&mut Self, CharacterMutationError> {
        // Preserve existing damage if possible
        let (bashing, lethal, aggravated) = (
            self.health.bashing_damage,
            self.health.lethal_damage,
            self.health.aggravated_damage,
        );

        self.health = Health {
            zero_boxes: new_wound_penalties
                .get(&WoundPenalty::Zero)
                .cloned()
                .unwrap_or_default(),
            minus_one_boxes: new_wound_penalties
                .get(&WoundPenalty::MinusOne)
                .cloned()
                .unwrap_or_default(),
            minus_two_boxes: new_wound_penalties
                .get(&WoundPenalty::MinusTwo)
                .cloned()
                .unwrap_or_default(),
            minus_four_boxes: new_wound_penalties
                .get(&WoundPenalty::MinusFour)
                .cloned()
                .unwrap_or_default(),
            incapacitated_boxes: new_wound_penalties
                .get(&WoundPenalty::Incapacitated)
                .cloned()
                .unwrap_or_default(),
            bashing_damage: 0,
            lethal_damage: 0,
            aggravated_damage: 0,
        };

        self.take_damage(DamageLevel::Bashing, bashing)
            .and_then(|character| character.take_damage(DamageLevel::Lethal, lethal))
            .and_then(|character| character.take_damage(DamageLevel::Aggravated, aggravated))
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
