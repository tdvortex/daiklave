use super::{AddIntimacy, IntimacyLevel, IntimacyTypeMemo};

/// A builder to construct a new Intimacy to add to a character.
pub struct IntimacyBuilder;

impl IntimacyBuilder {
    /// Sets the intimacy to be a Tie, and details what it's a tie to.
    pub fn tie(target: impl Into<String>) -> TieBuilder {
        TieBuilder {
            target: target.into(),
        }
    }

    /// Sets the intimacy to be a principle, and describes the principle.
    pub fn principle(description: impl Into<String>) -> IntimacyBuilderWithDescription {
        IntimacyBuilderWithDescription {
            intimacy_type: IntimacyTypeMemo::Principle(description.into()),
        }
    }
}

/// A builder to construct a Tie intimacy.
pub struct TieBuilder {
    target: String,
}

impl TieBuilder {
    /// Provides a description of the emotional quality of the intimacy.
    pub fn description(self, description: impl Into<String>) -> IntimacyBuilderWithDescription {
        IntimacyBuilderWithDescription {
            intimacy_type: IntimacyTypeMemo::Tie(self.target, description.into()),
        }
    }
}

/// An intimacy builder with a description.
pub struct IntimacyBuilderWithDescription {
    intimacy_type: IntimacyTypeMemo,
}

impl IntimacyBuilderWithDescription {
    /// Sets the level of the intimacy and completes the builder.
    pub fn level(self, level: IntimacyLevel) -> AddIntimacy {
        AddIntimacy {
            intimacy_type: self.intimacy_type,
            level,
        }
    }
}
