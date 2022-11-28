use crate::traits::abilities::Abilities;
use crate::traits::armor::Armor;
use crate::traits::attributes::Attributes;
use crate::traits::experience::ExperiencePoints;
use crate::traits::health::Health;
use crate::traits::intimacies::Intimacies;
use crate::traits::merits::Merits;
use crate::traits::weapons::Weapons;
use crate::traits::willpower::Willpower;

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
