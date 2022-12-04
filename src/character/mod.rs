pub mod builder;
mod mortal;
pub mod traits;

use std::ops::Deref;

use traits::campaign::Campaign;
use traits::experience::ExperiencePoints;
use traits::player::Player;
use traits::willpower::Willpower;

use self::traits::abilities::Abilities;
use self::traits::armor::Armor;
use self::traits::attributes::Attributes;
use self::traits::health::Health;
use self::traits::intimacies::Intimacies;
use self::traits::merits::Merits;
use self::traits::prerequisite::{Prerequisite, PrerequisiteSet, PrerequisiteType};
use self::traits::weapons::Weapons;

#[derive(Debug)]
pub struct Character {
    pub id: Option<i32>,
    pub player: Player,
    pub campaign: Option<Campaign>,
    pub name: String,
    pub concept: Option<String>,
    pub willpower: Willpower,
    pub experience: ExperiencePoints,
    pub attributes: Attributes,
    pub abilities: Abilities,
    pub intimacies: Intimacies,
    pub health: Health,
    pub weapons: Weapons,
    pub armor: Armor,
    pub merits: Merits,
}

impl Character {
    fn meets_prerequisite(&self, prerequisite: &Prerequisite) -> bool {
        match prerequisite.deref() {
            PrerequisiteType::Ability(ability_prerequisite) => {
                self.abilities.meets_prerequisite(ability_prerequisite)
            }
            PrerequisiteType::Attribute(attribute_prerequisite) => {
                self.attributes.meets_prerequisite(attribute_prerequisite)
            }
            PrerequisiteType::Essence(_) => false,
            PrerequisiteType::Charm(_) => false,
            PrerequisiteType::ExaltType(exalt_type) => match exalt_type {
                traits::prerequisite::ExaltTypePrerequisite::Solar => false,
                traits::prerequisite::ExaltTypePrerequisite::Lunar => false,
                traits::prerequisite::ExaltTypePrerequisite::DragonBlooded => false,
                traits::prerequisite::ExaltTypePrerequisite::Spirit => false,
                traits::prerequisite::ExaltTypePrerequisite::SpiritOrEclipse => false,
            },
        }
    }

    fn meets_prerequisite_set(&self, prerequisite_set: &PrerequisiteSet) -> bool {
        prerequisite_set
            .iter()
            .all(|prerequisite| self.meets_prerequisite(prerequisite))
    }

    pub fn meets_any_prerequisite_set(&self, prerequisite_sets: &Vec<PrerequisiteSet>) -> bool {
        prerequisite_sets
            .iter()
            .any(|prerequisite_set| self.meets_prerequisite_set(prerequisite_set))
    }
}
