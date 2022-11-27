use crate::abilities::Abilities;
use crate::armor::Armor;
use crate::attributes::Attributes;
use crate::experience::ExperiencePoints;
use crate::health::Health;
use crate::intimacies::Intimacies;
use crate::merits::Merits;
use crate::weapons::Weapons;
use crate::willpower::Willpower;

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
