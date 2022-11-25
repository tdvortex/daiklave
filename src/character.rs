mod abilities;
mod attributes;
mod merits;
mod weapons;
mod willpower;

use abilities::Abilities;
use attributes::Attributes;
use merits::Merits;
use weapons::Weapons;
use willpower::Willpower;

#[derive(Default, Debug)]
pub struct MortalCharacter {
    attributes: Attributes,
    abilities: Abilities,
    merits: Merits,
    weapons: Weapons,
    willpower: Willpower,
}

#[derive(Debug)]
pub enum Character {
    MortalCharacter(MortalCharacter),
}

impl Default for Character {
    fn default() -> Self {
        Self::MortalCharacter(MortalCharacter::default())
    }
}
