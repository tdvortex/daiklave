pub mod builder;
mod mortal;
pub mod traits;

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