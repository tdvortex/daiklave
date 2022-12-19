use crate::{abilities::update::AbilitiesDiff, attributes::AttributesDiff, craft::update::CraftDiff, intimacies::{IntimaciesDiff, compare_intimacies}, weapons::WeaponsDiff, merits::{MeritDiff, compare_merits}, health::HealthDiff, armor::ArmorDiff, martial_arts::update::MartialArtsDiff, Character};

use super::CharacterBaseDiff;

pub struct CharacterDiff {
    pub attributes_diff: AttributesDiff,
    pub abilities_diff: AbilitiesDiff,
    pub craft_diff: CraftDiff,
    pub base_diff: CharacterBaseDiff,
    pub health_diff: HealthDiff,
    pub intimacies_diff: IntimaciesDiff,
    pub weapons_diff: WeaponsDiff,
    pub armor_diff: ArmorDiff,
    pub merits_diff: MeritDiff,
    pub martial_arts_diff: MartialArtsDiff,
}

impl Character {
    pub fn compare_newer(&self, newer: &Character) -> CharacterDiff {
        CharacterDiff { 
            attributes_diff: self.attributes.compare_newer(&newer.attributes), 
            abilities_diff: self.abilities.compare_newer(&newer.abilities), 
            craft_diff: self.craft_abilities.compare_newer(&newer.craft_abilities),
            base_diff: self.compare_newer_base(&newer),
            health_diff: self.health.compare_newer(&newer.health),
            intimacies_diff: compare_intimacies(&self.intimacies, &newer.intimacies),
            weapons_diff: self.weapons.compare_newer(&newer.weapons),
            armor_diff: self.armor.compare_newer(&newer.armor),
            merits_diff: compare_merits(&self.merits, &newer.merits),
            martial_arts_diff: self.martial_arts_styles.compare_newer(&newer.martial_arts_styles),
        }
    }
}