use crate::{Character, player::Player, attributes::AttributeName};

use super::CharacterBuilder;

fn _begin_guided_builder(player: Player) -> _ChooseNameAndConcept {
    _ChooseNameAndConcept { 
        counter: 2,
        character_builder: Character::builder(1, player)
    }
}

struct _ChooseNameAndConcept {
    counter: i32,
    character_builder: CharacterBuilder,
}

impl _ChooseNameAndConcept {
    fn _name_and_concept(self, name: String, concept: Option<String>) -> _ChooseExaltType {
        if let Some(concept) = concept {
            _ChooseExaltType {
                counter: self.counter,
                character_builder: self.character_builder.with_name(name).with_concept(concept)
            }
        } else {
            _ChooseExaltType {
                counter: self.counter,
                character_builder: self.character_builder.with_name(name)
            } 
        }
    }
}

struct _ChooseExaltType {
    counter: i32,
    character_builder: CharacterBuilder,
}

enum _ExaltTypeChoice {
    Mortal,
    Solar,
}

impl _ChooseExaltType {
    pub fn _mortal(self) -> _MortalAttributesBuilder {
        _MortalAttributesBuilder {
            counter: self.counter,
            character_builder: self.character_builder,
            strength: 1,
            dexterity: 1,
            stamina: 1,
            charisma: 1,
            manipulation: 1,
            appearance: 1,
            perception: 1,
            intelligence: 1,
            wits: 1,
        }
    }
}

struct _MortalAttributesBuilder {
    counter: i32,
    character_builder: CharacterBuilder,
    strength: u8,
    dexterity: u8,
    stamina: u8,
    charisma: u8,
    manipulation: u8,
    appearance: u8,
    perception: u8,
    intelligence: u8,
    wits: u8,
}

enum _AttributeGroup {
    Mental,
    Social,
    Physical,
}

// Mortals get 8/6/4 attributes
impl _MortalAttributesBuilder {
    fn _group_dots_allocated(&self, group: _AttributeGroup) -> u8 {
        (match group {
            _AttributeGroup::Physical => self.strength + self.dexterity + self.stamina,
            _AttributeGroup::Social => self.charisma + self.manipulation + self.appearance,
            _AttributeGroup::Mental => self.perception + self.intelligence + self.wits,
        }) - 3
    }

    fn _can_increase(&self, attribute: AttributeName) -> bool {
        
        // Can't increase any attribute group above 8

        // Can't increase an attribute group above 4 if both of the other two 
        // are at 5+
        let attribute_value = match attribute {
            AttributeName::Strength => self.strength,
            AttributeName::Dexterity => self.dexterity,
            AttributeName::Stamina => self.stamina,
            AttributeName::Charisma => self.charisma,
            AttributeName::Manipulation => self.manipulation,
            AttributeName::Appearance => self.appearance,
            AttributeName::Perception => self.perception,
            AttributeName::Intelligence => self.intelligence,
            AttributeName::Wits => self.wits,
        };

        // Can't increase any attribute above five
        if attribute_value >= 5 {
            return false;
        }

        let mental = self._group_dots_allocated(_AttributeGroup::Mental);
        let social = self._group_dots_allocated(_AttributeGroup::Social);
        let physical = self._group_dots_allocated(_AttributeGroup::Physical);

        // Can't increase any attribute if 18 dots have been allocated
        if mental + social + physical >= 18 {
            return false;
        }

        let (target_group, alternate_one, alternate_two) = match attribute {
            AttributeName::Strength
            | AttributeName::Dexterity
            | AttributeName::Stamina => (physical, mental, social),
            AttributeName::Charisma
            | AttributeName::Manipulation
            | AttributeName::Appearance => (social, mental, physical),
            AttributeName::Perception
            | AttributeName::Intelligence
            | AttributeName::Wits => (mental, social, physical)
        };

        // Can't increase any attribute group above 8
        if target_group >= 8 {
            return false;
        }

        // Can't increase an attribute group above 6 if either of the other two
        // is at 7+ 
        if target_group >= 6 && (alternate_one > 6 || alternate_two > 6) {
            return false;
        }

        // Can't increase an attribute group above 4 if both of the other two 
        // are at 5+
        if target_group >= 4 && alternate_one > 4 && alternate_two > 4 {
            return false;
        }

        true
    }

    fn _can_decrease(&self, attribute: AttributeName) -> bool {
        // Can decrease to a minimum of 1 for any attribute
        match attribute {
            AttributeName::Strength => self.strength > 1,
            AttributeName::Dexterity => self.dexterity > 1,
            AttributeName::Stamina => self.stamina > 1,
            AttributeName::Charisma => self.charisma > 1,
            AttributeName::Manipulation => self.manipulation > 1,
            AttributeName::Appearance => self.appearance > 1,
            AttributeName::Perception => self.perception > 1,
            AttributeName::Intelligence => self.intelligence > 1,
            AttributeName::Wits => self.wits > 1,
        }
    }

    fn _increase(&mut self, attribute: AttributeName) -> bool {
        if !self._can_increase(attribute) {
            return false;
        }

        match attribute {
            AttributeName::Strength => {self.strength += 1;}
            AttributeName::Dexterity => {self.dexterity += 1;}
            AttributeName::Stamina => {self.stamina += 1;}
            AttributeName::Charisma => {self.charisma += 1;}
            AttributeName::Manipulation => {self.manipulation += 1;}
            AttributeName::Appearance => {self.appearance += 1;}
            AttributeName::Perception => {self.perception += 1;}
            AttributeName::Intelligence => {self.intelligence += 1;}
            AttributeName::Wits => {self.wits += 1;}
        }

        true
    }

    fn _decrease(&mut self, attribute: AttributeName) -> bool {
        if !self._can_decrease(attribute) {
            return false;
        }

        match attribute {
            AttributeName::Strength => {self.strength -= 1;}
            AttributeName::Dexterity => {self.dexterity -= 1;}
            AttributeName::Stamina => {self.stamina -= 1;}
            AttributeName::Charisma => {self.charisma -= 1;}
            AttributeName::Manipulation => {self.manipulation -= 1;}
            AttributeName::Appearance => {self.appearance -= 1;}
            AttributeName::Perception => {self.perception -= 1;}
            AttributeName::Intelligence => {self.intelligence -= 1;}
            AttributeName::Wits => {self.wits -= 1;}
        }

        true
    }

    fn _ready_to_progress(&self) -> bool {
        let mental = self._group_dots_allocated(_AttributeGroup::Mental);
        let social = self._group_dots_allocated(_AttributeGroup::Social);
        let physical = self._group_dots_allocated(_AttributeGroup::Physical);

        let max_group = mental.max(social).max(physical);
        let min_group = mental.min(social).min(physical);
        let third_group = mental + social + physical - max_group - min_group;

        max_group == 8 && min_group == 4 && third_group == 6
    }
}