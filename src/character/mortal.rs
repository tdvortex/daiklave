use crate::character::traits::abilities::Abilities;
use crate::character::traits::armor::Armor;
use crate::character::traits::attributes::Attributes;
use crate::character::traits::experience::ExperiencePoints;
use crate::character::traits::health::Health;
use crate::character::traits::intimacies::Intimacies;
use crate::character::traits::merits::Merits;
use crate::character::traits::weapons::Weapons;
use crate::character::traits::willpower::Willpower;

#[derive(Default, Debug)]
pub struct MortalCharacter {
    pub abilities: Abilities,
    pub armor: Armor,
    pub attributes: Attributes,
    pub experience: ExperiencePoints,
    pub health: Health,
    pub intimacies: Intimacies,
    pub merits: Merits,
    pub weapons: Weapons,
    pub willpower: Willpower,
}
