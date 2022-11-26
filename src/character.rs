pub mod abilities;
pub mod attributes;
pub mod merits;
pub mod weapons;
pub mod willpower;

use abilities::{AbilitiesIter, Ability, AbilityName, AbilityValue};
use attributes::{AttributeName, AttributeValue, AttributesIter};
use eyre::Result;
use merits::{Merit, MeritType};
use weapons::{Hand, WeaponDetails, WeaponPosition, WeaponsIter};

pub trait Character {
    fn attributes_iter(&self) -> AttributesIter;
    fn get_attribute(&self, attribute_name: &AttributeName) -> AttributeValue;
    fn set_attribute(&mut self, attribute_name: &AttributeName, new_value: AttributeValue);
    fn abilities_iter(&self) -> AbilitiesIter;
    fn get_ability(&self, ability_name: &AbilityName) -> Option<&Ability>;
    fn set_ability_value(&mut self, ability_name: &AbilityName, new_value: AbilityValue);
    fn add_specialty_to_ability(
        &mut self,
        ability_name: &AbilityName,
        specialty: String,
    ) -> Result<()>;
    fn remove_specialty_from_ability(
        &mut self,
        ability_name: &AbilityName,
        specialty: String,
    ) -> Result<()>;
    fn merits_iter(&self) -> std::collections::hash_set::Iter<'_, Merit>;
    fn add_merit(
        &mut self,
        name: String,
        maybe_detail: Option<String>,
        dots: u8,
        merit_type: MeritType,
        description: String,
    );
    fn remove_merit(&mut self, name: String, maybe_detail: Option<String>, dots: u8) -> Result<()>;
    fn recover_one_willpower(&mut self);
    fn recover_all_willpower(&mut self);
    fn gain_one_willpower(&mut self);
    fn spend_one_willpower(&mut self) -> Result<()>;
    fn weapons_iter(&self) -> WeaponsIter;
    fn get_weapon_at_position(&self, position: WeaponPosition) -> Result<&WeaponDetails>;
    fn add_weapon(&mut self, weapon: WeaponDetails, two_handed: bool);
    fn remove_weapon(&mut self, position: WeaponPosition) -> Result<()>;
    fn equip_weapon(&mut self, hand: Hand, position: WeaponPosition) -> Result<()>;
    fn unequip_weapon(&mut self, hand: Hand) -> Result<()>;
}
