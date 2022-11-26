pub mod abilities;
pub mod attributes;
pub mod merits;
pub mod weapons;
pub mod willpower;

use abilities::{AbilitiesIter, Ability, AbilityName, AbilityValue};
use attributes::{AttributeName, AttributeValue, AttributesIter};
use eyre::Result;

pub trait Character {
    fn attributes_iter(&self) -> AttributesIter;
    fn get_attribute(&self, attribute_name: &AttributeName) -> AttributeValue;
    fn set_attribute(&mut self, attribute_name: &AttributeName, new_value: AttributeValue);
    fn abilities_iter(&self) -> AbilitiesIter;
    fn get_ability(&self, ability_name: &AbilityName) -> Option<&Ability>;
    fn set_ability_value(&mut self, ability_name: &AbilityName, new_value: AbilityValue);
    fn add_specialty_to_ability(&mut self, ability_name: &AbilityName, specialty: String) -> Result<()>;
    fn remove_specialty_from_ability(
        &mut self,
        ability_name: &AbilityName,
        specialty: String,
    ) -> Result<()>;
}
