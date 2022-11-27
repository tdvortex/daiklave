use crate::abilities::Abilities;
use crate::attributes::Attributes;
use crate::merits::Merits;
use crate::weapons::Weapons;
use crate::willpower::Willpower;

#[derive(Default, Debug)]
pub struct MortalCharacter {
    pub attributes: Attributes,
    pub abilities: Abilities,
    pub merits: Merits,
    pub weapons: Weapons,
    pub willpower: Willpower,
}