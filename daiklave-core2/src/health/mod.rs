mod damage_level;
mod health_iter;
mod wound_penalty;

pub use damage_level::DamageLevel;
pub use wound_penalty::WoundPenalty;

use serde::{Deserialize, Serialize};

use crate::CharacterMutationError;

use self::health_iter::HealthIter;

/// Struct for a character's health track.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub struct Health {
    zero_boxes: u8,
    minus_one_boxes: u8,
    minus_two_boxes: u8,
    minus_four_boxes: u8,
    incapacitated_boxes: u8,
    bashing_damage: u8,
    lethal_damage: u8,
    aggravated_damage: u8,
}

impl Default for Health {
    fn default() -> Self {
        Self {
            zero_boxes: 1,
            minus_one_boxes: 2,
            minus_two_boxes: 2,
            minus_four_boxes: 1,
            incapacitated_boxes: 1,
            bashing_damage: 0,
            lethal_damage: 0,
            aggravated_damage: 0,
        }
    }
}

impl Health {
    /// Iterates over a health track as boxes from left to right. Lower wound
    /// penalties (-0, -1) appear before higher wound penalties (-4, INC);
    /// worse damage types (Agg, Lethal) appear before lighter wound penalties
    /// (Bashing, no damage)
    pub fn iter(&self) -> impl Iterator<Item = (WoundPenalty, Option<DamageLevel>)> {
        HealthIter {
            zero_boxes: self.zero_boxes,
            minus_one_boxes: self.minus_one_boxes,
            minus_two_boxes: self.minus_two_boxes,
            minus_four_boxes: self.minus_four_boxes,
            incapacitated_boxes: self.incapacitated_boxes,
            bashing_damage: self.bashing_damage,
            lethal_damage: self.lethal_damage,
            aggravated_damage: self.aggravated_damage,
        }
    }

    /// The character's current wound penalty, given their current damage
    /// amount.
    pub fn current_wound_penalty(&self) -> WoundPenalty {
        let mut damage = self.bashing_damage + self.lethal_damage + self.aggravated_damage;
        if damage <= self.zero_boxes {
            return WoundPenalty::Zero;
        } else {
            damage -= self.zero_boxes;
        }

        if damage <= self.minus_one_boxes {
            return WoundPenalty::MinusOne;
        } else {
            damage -= self.minus_one_boxes;
        }

        if damage <= self.minus_two_boxes {
            return WoundPenalty::MinusTwo;
        } else {
            damage -= self.minus_two_boxes;
        }

        if damage <= self.minus_four_boxes {
            WoundPenalty::MinusFour
        } else {
            WoundPenalty::Incapacitated
        }
    }

    pub(crate) fn set_wound_penalties(
        &mut self,
        new_wound_penalties: &[WoundPenalty],
    ) -> Result<&mut Self, CharacterMutationError> {
        self.bashing_damage = 0;
        self.lethal_damage = 0;
        self.aggravated_damage = 0;
        self.zero_boxes = 0;
        self.minus_one_boxes = 0;
        self.minus_two_boxes = 0;
        self.minus_four_boxes = 0;
        self.incapacitated_boxes = 0;
        for wound_penalty in new_wound_penalties.iter() {
            match wound_penalty {
                WoundPenalty::Zero => {
                    self.zero_boxes += 1;
                }
                WoundPenalty::MinusOne => {
                    self.minus_one_boxes += 1;
                }
                WoundPenalty::MinusTwo => {
                    self.minus_two_boxes += 1;
                }
                WoundPenalty::MinusFour => {
                    self.minus_four_boxes += 1;
                }
                WoundPenalty::Incapacitated => {
                    self.incapacitated_boxes += 1;
                }
            }
        }
        Ok(self)
    }

    pub(crate) fn take_damage(
        &mut self,
        damage_level: DamageLevel,
        amount: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        let total_health = self.zero_boxes
            + self.minus_one_boxes
            + self.minus_two_boxes
            + self.minus_four_boxes
            + self.incapacitated_boxes;

        match damage_level {
            DamageLevel::Bashing => {
                self.bashing_damage += amount;
            }
            DamageLevel::Lethal => {
                self.lethal_damage += amount;
            }
            DamageLevel::Aggravated => {
                self.aggravated_damage += amount;
            }
        }

        while self.bashing_damage + self.lethal_damage + self.aggravated_damage > total_health {
            if self.bashing_damage >= 2 {
                // Bashing damage rolls over into lethal
                self.bashing_damage -= 2;
                self.lethal_damage += 1;
            } else if self.bashing_damage == 1 && self.lethal_damage >= 1 {
                // If full up with lethal and still have 1 bashing damage,
                // upgrades lethal to aggravated
                self.bashing_damage -= 1;
                self.lethal_damage -= 1;
                self.aggravated_damage += 1;
            } else if self.lethal_damage >= 2 {
                // If full up with lethal, starts upgrading to aggravated
                self.lethal_damage -= 2;
                self.aggravated_damage += 1;
            } else {
                // If full up with aggravated, all other damage is irrelevant
                self.bashing_damage = 0;
                self.lethal_damage = 0;
                self.aggravated_damage = total_health;
            }
        }

        Ok(self)
    }

    pub(crate) fn heal_damage(&mut self, amount: u8) -> Result<&mut Self, CharacterMutationError> {
        if amount == 0 {
            return Ok(self);
        }
        let mut amount = amount;
        let bashing_healed = self.bashing_damage.min(amount);
        self.bashing_damage -= bashing_healed;
        amount -= bashing_healed;

        if amount == 0 {
            return Ok(self);
        }
        let lethal_healed = self.lethal_damage.min(amount);
        self.lethal_damage -= lethal_healed;
        amount -= lethal_healed;

        if amount == 0 {
            return Ok(self);
        }
        let aggravated_healed = self.aggravated_damage.min(amount);
        self.aggravated_damage -= aggravated_healed;

        Ok(self)
    }
}
