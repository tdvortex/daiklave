/// Provides a CharacterBuilder struct, which can be used to scaffold a character quickly.
/// Calling the .build() method will attempt to compile the builder into a true Character struct.
/// This requires that, at a minimum, the character has been given a player and a name.
/// All values are otherwise set to either character defaults, or an empty list.
pub mod builder;

/// Contains all of the individual traits that describe a character.
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

/// The basic Character object, representing a full player character.
/// This represents the state of a valid character at a given instant of a game.
/// It is also the serialization format to be moved back and forth between client and server.
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
