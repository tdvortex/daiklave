pub mod abilities;
pub mod attributes;
pub mod merits;
pub mod weapons;
pub mod willpower;

use attributes::{AttributeName, AttributeValue};

pub trait Character {
    fn get_attribute(&self, attribute_name: &AttributeName) -> AttributeValue;
    fn set_attribute(&mut self, attribute_name: &AttributeName, new_value: AttributeValue);
}
