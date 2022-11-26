use crate::abilities::{Abilities, AbilitiesIter, Ability, AbilityName, AbilityValue, HasAbilities};
use crate::attributes::{HasAttributes, AttributeName, AttributeValue, Attributes, AttributesIter};
use crate::merits::{Merit, MeritType, Merits, HasMerits};
use crate::weapons::{Hand, WeaponDetails, WeaponPosition, Weapons, WeaponsIter, HasWeapons};
use crate::willpower::{Willpower, HasWillpower};
use eyre::{eyre, Result};

#[derive(Default, Debug)]
pub struct MortalCharacter {
    attributes: Attributes,
    abilities: Abilities,
    merits: Merits,
    weapons: Weapons,
    willpower: Willpower,
}

impl HasAttributes for MortalCharacter {
    fn attributes_iter(&self) -> AttributesIter {
        self.attributes.iter()
    }

    fn get_attribute(&self, attribute_name: &AttributeName) -> AttributeValue {
        self.attributes.get(attribute_name)
    }

    fn set_attribute(&mut self, attribute_name: &AttributeName, new_value: AttributeValue) {
        self.attributes.set(attribute_name, new_value);
    }
}

impl HasAbilities for MortalCharacter {
    fn abilities_iter(&self) -> AbilitiesIter {
        self.abilities.iter()
    }

    fn get_ability(&self, ability_name: &AbilityName) -> Option<&Ability> {
        self.abilities.get(ability_name)
    }

    fn set_ability_value(&mut self, ability_name: &AbilityName, new_value: AbilityValue) {
        self.abilities.set_value(ability_name, new_value)
    }

    fn add_specialty_to_ability(
        &mut self,
        ability_name: &AbilityName,
        specialty: String,
    ) -> Result<()> {
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

impl HasMerits for MortalCharacter {
    fn merits_iter(&self) -> std::collections::hash_set::Iter<'_, Merit> {
        self.merits.iter()
    }

    fn add_merit(
        &mut self,
        name: String,
        maybe_detail: Option<String>,
        dots: u8,
        merit_type: MeritType,
        description: String,
    ) {
        let merit_to_add = Merit::new(name, dots, merit_type, description, maybe_detail);
        self.merits.insert(merit_to_add);
    }

    fn remove_merit(&mut self, name: String, maybe_detail: Option<String>, dots: u8) -> Result<()> {
        let merit_to_remove = Merit::new(
            name,
            dots,
            MeritType::Purchased,
            "".to_owned(),
            maybe_detail,
        );
        let removed = self.merits.remove(&merit_to_remove);
        if removed {
            Ok(())
        } else {
            Err(eyre!("merit {} not found", merit_to_remove))
        }
    }
}

impl HasWillpower for MortalCharacter {
    fn recover_one_willpower(&mut self) {
        self.willpower.recover_one()
    }

    fn recover_all_willpower(&mut self) {
        self.willpower.recover_all()
    }

    fn gain_one_willpower(&mut self) {
        self.willpower.gain_one()
    }

    fn spend_one_willpower(&mut self) -> Result<()> {
        self.willpower.spend_one()
    }
}

impl HasWeapons for MortalCharacter {
    fn weapons_iter(&self) -> WeaponsIter {
        self.weapons.iter()
    }

    fn get_weapon_at_position(&self, position: WeaponPosition) -> Result<&WeaponDetails> {
        self.weapons.get_at_position(position)
    }

    fn add_weapon(&mut self, weapon: WeaponDetails, two_handed: bool) {
        self.weapons.add_weapon(weapon, two_handed)
    }

    fn remove_weapon(&mut self, position: WeaponPosition) -> Result<()> {
        self.weapons.remove_weapon(position)
    }

    fn equip_weapon(&mut self, hand: Hand, position: WeaponPosition) -> Result<()> {
        self.weapons.equip_weapon(hand, position)
    }

    fn unequip_weapon(&mut self, hand: Hand) -> Result<()> {
        self.weapons.unequip_weapon(hand)
    }
}
