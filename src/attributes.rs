// Attributes are nonnegative integers
// Usually rated 1 to 5, but may be 6+ in some cases
type AttributeValue = u8;

enum AttributeName {
    Strength,
    Dexterity,
    Stamina,
    Charisma,
    Manipulation,
    Appearance,
    Perception,
    Intelligence,
    Wits,
}

struct Attributes {
    strength: AttributeValue,
    dexterity: AttributeValue,
    stamina: AttributeValue,
    charisma: AttributeValue,
    manipulation: AttributeValue,
    appearance: AttributeValue,
    perception: AttributeValue,
    intelligence: AttributeValue,
    wits: AttributeValue,
}

// Attributes default to 1, not 0
impl Default for Attributes {
    fn default() -> Self {
        Self {
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

impl Attributes {
    fn get(&self, attribute: &AttributeName) -> AttributeValue {
        match attribute {
            AttributeName::Strength => self.strength,
            AttributeName::Dexterity => self.dexterity,
            AttributeName::Stamina => self.stamina,
            AttributeName::Charisma => self.charisma,
            AttributeName::Manipulation => self.manipulation,
            AttributeName::Appearance => self.appearance,
            AttributeName::Perception => self.perception,
            AttributeName::Intelligence => self.intelligence,
            AttributeName::Wits => self.wits,
        }
    }

    fn set(&mut self, attribute: &AttributeName, new_value: AttributeValue) {
        match attribute {
            AttributeName::Strength => self.strength = new_value,
            AttributeName::Dexterity => self.dexterity = new_value,
            AttributeName::Stamina => self.stamina = new_value,
            AttributeName::Charisma => self.charisma = new_value,
            AttributeName::Manipulation => self.manipulation = new_value,
            AttributeName::Appearance => self.appearance = new_value,
            AttributeName::Perception => self.perception = new_value,
            AttributeName::Intelligence => self.intelligence = new_value,
            AttributeName::Wits => self.wits = new_value,
        }
    }
}
