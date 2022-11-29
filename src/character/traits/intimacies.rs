pub type Intimacies = Vec<Intimacy>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum IntimacyLevel {
    Minor,
    Major,
    Defining,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum IntimacyType {
    Tie,
    Principle,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Intimacy {
    pub intimacy_level: IntimacyLevel,
    pub intimacy_type: IntimacyType,
    pub description: String,
}
