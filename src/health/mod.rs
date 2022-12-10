use serde::{Deserialize, Serialize};
pub(crate) mod update;
pub use update::HealthDiff;
pub(crate) mod tables;
use eyre::Result;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Health {
    health_boxes: Vec<HealthBox>,
}

impl Default for Health {
    fn default() -> Self {
        Self {
            health_boxes: vec![
                HealthBox::new(WoundPenalty::Zero),
                HealthBox::new(WoundPenalty::MinusOne),
                HealthBox::new(WoundPenalty::MinusOne),
                HealthBox::new(WoundPenalty::MinusTwo),
                HealthBox::new(WoundPenalty::MinusTwo),
                HealthBox::new(WoundPenalty::MinusFour),
                HealthBox::new(WoundPenalty::Incapacitated),
            ],
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct HealthBox {
    wound_penalty: WoundPenalty,
    damage: DamageLevel,
}

impl HealthBox {
    fn new(wound_penalty: WoundPenalty) -> Self {
        Self {
            wound_penalty,
            damage: DamageLevel::None,
        }
    }

    pub fn wound_penalty(&self) -> WoundPenalty {
        self.wound_penalty
    }

    pub fn damage(&self) -> DamageLevel {
        self.damage
    }
}

impl Health {
    pub(crate) fn empty() -> Self {
        Self {
            health_boxes: Vec::new(),
        }
    }

    pub fn damage(&self) -> (u8, u8, u8) {
        self.health_boxes
            .iter()
            .fold(
                (0, 0, 0),
                |(bashing, lethal, aggravated), health_box| match health_box.damage {
                    DamageLevel::None => (bashing, lethal, aggravated),
                    DamageLevel::Bashing => (bashing + 1, lethal, aggravated),
                    DamageLevel::Lethal => (bashing, lethal + 1, aggravated),
                    DamageLevel::Aggravated => (bashing, lethal, aggravated + 1),
                },
            )
    }

    pub fn heal_all_damage(&mut self) {
        self.health_boxes
            .iter_mut()
            .for_each(|health_box| health_box.damage = DamageLevel::None);
    }

    pub fn set_damage(&mut self, mut bashing: u8, mut lethal: u8, mut aggravated: u8) {
        // Note: excess damage is dropped, not rolled over
        self.heal_all_damage();
        self.health_boxes.iter_mut().for_each(|health_box| {
            match (&mut bashing, &mut lethal, &mut aggravated) {
                (0, 0, 0) => {}
                (bashing, 0, 0) => {
                    health_box.damage = DamageLevel::Bashing;
                    *bashing -= 1;
                }
                (_, lethal, 0) => {
                    health_box.damage = DamageLevel::Lethal;
                    *lethal -= 1;
                }
                (_, _, aggravated) => {
                    health_box.damage = DamageLevel::Aggravated;
                    *aggravated -= 1;
                }
            }
        });
    }

    fn sort_boxes(&mut self) {
        let (bashing, lethal, aggravated) = self.damage();
        self.heal_all_damage();
        self.health_boxes
            .sort_by(|a, b| a.wound_penalty.cmp(&b.wound_penalty));
        self.set_damage(bashing, lethal, aggravated);
    }

    pub fn health_boxes(&self) -> &Vec<HealthBox> {
        &self.health_boxes
    }

    pub fn current_wound_penalty(&self) -> WoundPenalty {
        let (bashing, lethal, aggravated) = self.damage();
        let total_damage: usize = (bashing + lethal + aggravated).into();
        if total_damage == 0 {
            WoundPenalty::Zero
        } else {
            self.health_boxes[total_damage - 1].wound_penalty
        }
    }

    pub fn add_health_box(&mut self, wound_penalty: WoundPenalty) {
        self.health_boxes.push(HealthBox::new(wound_penalty));
        self.sort_boxes();
    }

    pub fn remove_health_box(&mut self, wound_penalty: WoundPenalty) -> Result<()> {
        // Attempt to preserve damage totals
        let (bashing, lethal, aggravated) = self.damage();
        self.heal_all_damage();

        for i in 0..self.health_boxes.len() {
            if self.health_boxes[i].wound_penalty == wound_penalty {
                self.health_boxes.remove(i);
                self.set_damage(bashing, lethal, aggravated);
                return Ok(());
            }
        }
        Err(eyre::eyre!(
            "no health box with wound penalty {:?}",
            wound_penalty
        ))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
pub enum WoundPenalty {
    Zero,
    MinusOne,
    MinusTwo,
    MinusFour,
    Incapacitated,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
pub enum DamageLevel {
    None,
    Bashing,
    Lethal,
    Aggravated,
}
