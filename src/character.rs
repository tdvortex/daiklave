pub mod abilities;
pub mod attributes;
pub mod merits;
pub mod weapons;
pub mod willpower;

use abilities::{AbilitiesIter};
use attributes::{AttributeName, AttributeValue, AttributesIter};

pub trait Character {
    fn get_attribute(&self, attribute_name: &AttributeName) -> AttributeValue;
    fn set_attribute(&mut self, attribute_name: &AttributeName, new_value: AttributeValue);
    fn attributes_iter(&self) -> AttributesIter;
    fn abilities_iter(&self) -> AbilitiesIter;
}
