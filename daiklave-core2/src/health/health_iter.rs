use super::{wound_penalty::WoundPenalty, damage_level::DamageLevel};

pub(in crate::health) struct HealthIter {
    zero_boxes: u8,
    minus_one_boxes: u8,
    minus_two_boxes: u8,
    minus_four_boxes: u8,
    incapacitated_boxes: u8,
    bashing_damage: u8,
    lethal_damage: u8,
    aggravated_damage: u8,
}

impl HealthIter {
    pub fn new(
        zero_boxes: u8,
        minus_one_boxes: u8,
        minus_two_boxes: u8,
        minus_four_boxes: u8,
        incapacitated_boxes: u8,
        bashing_damage: u8,
        lethal_damage: u8,
        aggravated_damage: u8,
    ) -> Self {
        Self {
            zero_boxes,
            minus_one_boxes,
            minus_two_boxes,
            minus_four_boxes,
            incapacitated_boxes,
            bashing_damage,
            lethal_damage,
            aggravated_damage,
        }
    }
}

impl Iterator for HealthIter {
    type Item = (WoundPenalty, Option<DamageLevel>);

    fn next(&mut self) -> Option<Self::Item> {
        let wound_penalty = if self.zero_boxes > 0 {
            self.zero_boxes -= 1;
            WoundPenalty::Zero
        } else if self.minus_one_boxes > 0 {
            self.minus_one_boxes -= 1;
            WoundPenalty::MinusOne
        } else if self.minus_two_boxes > 0 {
            self.minus_two_boxes -= 1;
            WoundPenalty::MinusTwo
        } else if self.minus_four_boxes > 0 {
            self.minus_four_boxes -= 1;
            WoundPenalty::MinusFour
        } else if self.incapacitated_boxes > 0 {
            self.incapacitated_boxes -= 1;
            WoundPenalty::Incapacitated
        } else {
            return None;
        };

        let maybe_damage = if self.aggravated_damage > 0 {
            self.aggravated_damage -= 1;
            Some(DamageLevel::Aggravated)
        } else if self.lethal_damage > 0 {
            self.lethal_damage -= 1;
            Some(DamageLevel::Lethal)
        } else if self.bashing_damage > 0 {
            self.bashing_damage -= 1;
            Some(DamageLevel::Bashing)
        } else {
            None
        };

        Some((wound_penalty, maybe_damage))
    }
}