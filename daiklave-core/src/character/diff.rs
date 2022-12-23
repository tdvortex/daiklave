use crate::{
    abilities::diff::AbilitiesDiff,
    armor::ArmorDiff,
    attributes::AttributesDiff,
    craft::diff::CraftDiff,
    health::HealthDiff,
    intimacies::IntimaciesDiff,
    martial_arts::diff::MartialArtsDiff,
    merits::{compare_merits, MeritDiff},
    weapons::WeaponsDiff,
    Character,
};

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
            base_diff: self.compare_newer_base(newer),
            health_diff: self.health.compare_newer(&newer.health),
            intimacies_diff: IntimaciesDiff::default(), // TODO: Fix this
            weapons_diff: self.weapons.compare_newer(&newer.weapons),
            armor_diff: self.armor.compare_newer(&newer.armor),
            merits_diff: compare_merits(&self.merits, &newer.merits),
            martial_arts_diff: self
                .martial_arts_styles
                .compare_newer(&newer.martial_arts_styles),
        }
    }
}

#[derive(Debug, Default)]
pub struct CharacterBaseDiff(pub Option<(String, Option<String>, i16, i16, i16, i16, Option<i32>)>);

impl Character {
    pub fn compare_newer_base(&self, newer: &Character) -> CharacterBaseDiff {
        let mut diff = CharacterBaseDiff::default();

        let eq_condition = (self.name.as_str() == newer.name.as_str())
            && (self.concept.as_deref() == newer.concept.as_deref())
            && (self.willpower.current == newer.willpower.current)
            && (self.willpower.maximum == newer.willpower.maximum)
            && (self.experience.current.min(i16::MAX as u16)
                != newer.experience.current.max(i16::MAX as u16))
            && (self.initiative == newer.initiative)
            && (self.experience.total.min(i16::MAX as u16)
                != newer.experience.total.max(i16::MAX as u16));

        if !eq_condition {
            diff = CharacterBaseDiff(Some((
                newer.name.clone(),
                newer.concept.clone(),
                newer.willpower.current as i16,
                newer.willpower.maximum as i16,
                newer
                    .experience
                    .current
                    .min(i16::MAX as u16)
                    .try_into()
                    .unwrap(),
                newer
                    .experience
                    .total
                    .min(i16::MAX as u16)
                    .try_into()
                    .unwrap(),
                newer.initiative.current(),
            )));
        }

        diff
    }
}
