use crate::{attributes::AttributeName, guided::ExaltationChoice, abilities::{AbilityNameVanilla, AbilityName}};

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

    fn solar_merits_bonus_points_spent(&self) -> i32 {
        // Solars get 10 free merit dots, the rest are 1 BP per dot
        self.merit_dots - self.merit_dots.min(10)
    }

    fn mortal_ability_bonus_points_spent(&self) -> i32 {
        // Mortals get 28 free ability dots with a limit of 3 per skill
        // Dots at 4 or 5, or 29+ total, are 2 each
        let mut three_or_less = 0;
        let mut more_than_three = 0;

        for vanilla in AbilityNameVanilla::iter() {
            let dots = self.character_view.abilities().dots(vanilla);
            three_or_less += dots.min(3);
            more_than_three += dots - dots.min(3);
        }

        for style_id in self.character_view.martial_arts().iter() {
            let dots = self.character_view.martial_arts().style(style_id).unwrap().dots();
            three_or_less += dots.min(3);
            more_than_three += dots - dots.min(3);
        }

        for focus in self.character_view.craft().iter() {
            let dots = self.character_view.craft().dots(focus);
            three_or_less += dots.min(3);
            more_than_three += dots - dots.min(3);
        }

        (2 * (three_or_less - 28.min(three_or_less) + more_than_three)) as i32
    }

    fn solar_ability_bonus_points_spent(&self) -> i32 {
        if self.character_view.solar_traits().is_none() {
            // Solar traits are set before abilities
            return 0;
        }

        // Solars get 28 free ability dots with a limit of 3 per skill
        // Dots above 3 in a skill need to be purchases, as do dots 29+ at 3 
        // or less
        // Caste or Favored skills cost 1 BP each, non-Caste non-Favored 
        // abilities cost 2
        // Efficent allocation puts 28 free dots towards non-C/F skills first
        let mut cf_three_or_less = 0;
        let mut cf_more_than_three = 0;
        let mut not_cf_three_or_less = 0;
        let mut not_cf_more_than_three = 0;

        for vanilla in AbilityNameVanilla::iter() {
            let dots = self.character_view.abilities().dots(vanilla);
            if self.character_view.solar_traits().unwrap().has_caste_ability(vanilla.into()) 
                || self.character_view.solar_traits().unwrap().has_favored_ability(vanilla.into()) {
                    cf_three_or_less += dots.min(3);
                    cf_more_than_three += dots - dots.min(3);
            } else {
                not_cf_three_or_less += dots.min(3);
                not_cf_more_than_three += dots - dots.min(3);
            }
        }

        for style_id in self.character_view.martial_arts().iter() {
            let dots = self.character_view.martial_arts().style(style_id).unwrap().dots();
            if self.character_view.solar_traits().unwrap().has_caste_ability(AbilityName::Brawl) 
                || self.character_view.solar_traits().unwrap().has_favored_ability(AbilityName::Brawl) {
                    cf_three_or_less += dots.min(3);
                    cf_more_than_three += dots - dots.min(3);
            } else {
                not_cf_three_or_less += dots.min(3);
                not_cf_more_than_three += dots - dots.min(3);
            }
        }

        for focus in self.character_view.craft().iter() {
            let dots = self.character_view.craft().dots(focus);
            if self.character_view.solar_traits().unwrap().has_caste_ability(AbilityName::Craft) 
            || self.character_view.solar_traits().unwrap().has_favored_ability(AbilityName::Craft) {
                cf_three_or_less += dots.min(3);
                cf_more_than_three += dots - dots.min(3);
        } else {
            not_cf_three_or_less += dots.min(3);
            not_cf_more_than_three += dots - dots.min(3);
        }
        }

        let three_or_less = cf_three_or_less + not_cf_three_or_less;
        let over_28 = three_or_less - 28.min(three_or_less);
        let discount = over_28.min(cf_three_or_less);

        (2 * (over_28 + not_cf_more_than_three) + cf_more_than_three - discount) as i32
    }

    pub(in crate::guided) fn update_bonus_points(&mut self) {
        if let Some(exaltation_choice) = self.exaltation_choice {
            match exaltation_choice {
                ExaltationChoice::Mortal => {
                    self.bonus_points = 21;
                    self.bonus_points -= self.mortal_attributes_bonus_points_spent();
                    self.bonus_points -= self.mortal_ability_bonus_points_spent();
                    self.bonus_points -= self.mortal_merits_bonus_points_spent();
                }
                ExaltationChoice::Dawn
                | ExaltationChoice::Zenith
                | ExaltationChoice::Twilight
                | ExaltationChoice::Night
                | ExaltationChoice::Eclipse => {
                    self.bonus_points = 15;
                    self.bonus_points -= self.solar_attributes_bonus_points_spent();
                    self.bonus_points -= self.solar_ability_bonus_points_spent();
                    self.bonus_points -= self.solar_merits_bonus_points_spent();
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
