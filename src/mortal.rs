use crate::character::abilities::{Abilities, AbilitiesIter, Ability, AbilityName, AbilityValue};
use crate::character::attributes::{AttributeName, AttributeValue, Attributes, AttributesIter};
use crate::character::merits::Merits;
use crate::character::weapons::Weapons;
use crate::character::willpower::Willpower;
use crate::character::Character;
use eyre::{eyre, Result};

#[derive(Default, Debug)]
pub struct MortalCharacter {
    attributes: Attributes,
    abilities: Abilities,
    merits: Merits,
    weapons: Weapons,
    willpower: Willpower,
}

impl Character for MortalCharacter {
    fn attributes_iter(&self) -> AttributesIter {
        self.attributes.iter()
    }
    
    fn get_attribute(&self, attribute_name: &AttributeName) -> AttributeValue {
        self.attributes.get(attribute_name)
    }

    fn set_attribute(&mut self, attribute_name: &AttributeName, new_value: AttributeValue) {
        self.attributes.set(attribute_name, new_value);
    }
    
    fn abilities_iter(&self) -> AbilitiesIter {
        self.abilities.iter()
    }

    fn get_ability(&self, ability_name: &AbilityName) -> Option<&Ability> {
        self.abilities.get(ability_name)
    }

    fn set_ability_value(&mut self, ability_name: &AbilityName, new_value: AbilityValue) {
        self.abilities.set_value(ability_name, new_value)
    }

    fn add_specialty_to_ability(&mut self, ability_name: &AbilityName, specialty: String) -> Result<()> {
        if let Some(ability) = self.abilities.get_mut(ability_name) {
            ability.add_specialty(specialty)
        } else {
            Err(eyre!("could not find ability {}", ability_name))
        }
    }

    fn remove_specialty_from_ability(
        &mut self,
        ability_name: &AbilityName,
        specialty: String,
    ) -> Result<()> {
        if let Some(ability) = self.abilities.get_mut(ability_name) {
            ability.remove_specialty(specialty)
        } else {
            Err(eyre!("could not find ability {}", ability_name))
        }
    }
}
