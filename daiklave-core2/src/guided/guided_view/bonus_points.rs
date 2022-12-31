use crate::{attributes::AttributeName, guided::ExaltationChoice};

use super::GuidedView;

impl<'source> GuidedView<'source> {
    pub(in crate::guided) fn attributes_buckets(&self) -> (u8, u8, u8) {
        let physical_attributes = self
            .character_view
            .attributes()
            .dots(AttributeName::Strength)
            + self
                .character_view
                .attributes()
                .dots(AttributeName::Dexterity)
            + self
                .character_view
                .attributes()
                .dots(AttributeName::Stamina);
        let mental_attributes = self
            .character_view
            .attributes()
            .dots(AttributeName::Perception)
            + self
                .character_view
                .attributes()
                .dots(AttributeName::Intelligence)
            + self.character_view.attributes().dots(AttributeName::Wits);
        let social_attributes = self
            .character_view
            .attributes()
            .dots(AttributeName::Charisma)
            + self
                .character_view
                .attributes()
                .dots(AttributeName::Manipulation)
            + self
                .character_view
                .attributes()
                .dots(AttributeName::Appearance);

        let primary = physical_attributes
            .max(mental_attributes)
            .max(social_attributes)
            - 3;
        let tertiary = physical_attributes
            .min(mental_attributes)
            .min(social_attributes)
            - 3;
        let secondary =
            physical_attributes + mental_attributes + social_attributes - primary - tertiary - 9;

        (primary, secondary, tertiary)
    }

    fn mortal_attributes_bonus_points_spent(&self) -> i32 {
        let (primary, secondary, tertiary) = self.attributes_buckets();
        ((primary - primary.min(6) + secondary - secondary.min(4)) * 4
            + (tertiary - tertiary.min(3)) * 3)
            .into()
    }

    fn solar_attributes_bonus_points_spent(&self) -> i32 {
        let (primary, secondary, tertiary) = self.attributes_buckets();
        ((primary - primary.min(8) + secondary - secondary.min(6)) * 4
            + (tertiary - tertiary.min(4)) * 3)
            .into()
    }

    fn mortal_merits_bonus_points_spent(&self) -> i32 {
        // Mortals get 7 free merit dots, the rest are 1 BP per dot
        self.merit_dots - self.merit_dots.min(7)
    }

    fn solal_merits_bonus_points_spent(&self) -> i32 {
        // Solars get 10 free merit dots, the rest are 1 BP per dot
        self.merit_dots - self.merit_dots.min(10)
    }

    pub(in crate::guided) fn update_bonus_points(&mut self) {
        if let Some(exaltation_choice) = self.exaltation_choice {
            match exaltation_choice {
                ExaltationChoice::Mortal => {
                    self.bonus_points = 21;
                    self.bonus_points -= self.mortal_attributes_bonus_points_spent();
                    self.bonus_points -= self.mortal_merits_bonus_points_spent();
                }
                ExaltationChoice::Dawn
                | ExaltationChoice::Zenith
                | ExaltationChoice::Twilight
                | ExaltationChoice::Night
                | ExaltationChoice::Eclipse => {
                    self.bonus_points = 15;
                    self.bonus_points -= self.solar_attributes_bonus_points_spent();
                    self.bonus_points -= self.solal_merits_bonus_points_spent();
                }
            }
        } else {
            self.bonus_points = 0;
        }
    }

    /// The number of available Bonus Points to spend.
    pub fn bonus_points_remaining(&self) -> i32 {
        self.bonus_points
    }
}
