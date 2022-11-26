use crate::character::abilities::{Abilities, AbilitiesIter};
use crate::character::attributes::{AttributeName, AttributeValue, Attributes, AttributesIter};
use crate::character::merits::Merits;
use crate::character::weapons::Weapons;
use crate::character::willpower::Willpower;
use crate::character::Character;

#[derive(Default, Debug)]
pub struct MortalCharacter {
    attributes: Attributes,
    abilities: Abilities,
    merits: Merits,
    weapons: Weapons,
    willpower: Willpower,
}

impl Character for MortalCharacter {
    fn get_attribute(&self, attribute_name: &AttributeName) -> AttributeValue {
        self.attributes.get(attribute_name)
    }

    fn set_attribute(&mut self, attribute_name: &AttributeName, new_value: AttributeValue) {
        self.attributes.set(attribute_name, new_value);
    }

    fn attributes_iter(&self) -> AttributesIter {
        self.attributes.iter()
    }

    fn abilities_iter(&self) -> AbilitiesIter {
        self.abilities.iter()
    }
}