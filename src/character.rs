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
pub struct Character {
    attributes: Attributes,
    abilities: Abilities,
    merits: Merits,
    owned_weapons: Weapons,
    willpower: Willpower,
}
