use std::ops::Deref;

use serde::{Serialize, Deserialize};
use thiserror::Error;

/// An identifier for a character component or referenced item. All other Id
/// subtypes should Deref to this type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub enum Id {  
    /// Used as a key for items which are not unique or are identified uniquely
    /// by other structural elements (such as the name of a MartialArts style.)
    NonUnique,
    /// Used as an offline key before saving to the database. Uniqueness is 
    /// maintained client-side. Stored as a u32 to prevent accidental
    /// cross-contamination with Id::Database.
    Placeholder(u32),
    /// The Id as stored in the database. i32 is equivalent to Integer in 
    /// Postgres and most other SQL engines.
    Database(i32),
}

impl Default for Id {
    fn default() -> Self {
        Self::Placeholder(0)
    }
}

/// An Id identifying a Character. 
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash, Default)]
pub struct CharacterId(pub Id);

impl Deref for CharacterId {
    type Target = Id;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// An error occurring while attempting to set CharacterId.
#[derive(Debug, Error)]
pub enum SetIdError {
    /// DatabaseIds cannot be safely overridden.
    #[error("Cannot override existing database Id")]
    DatabaseIdExists,
    /// Characters must have unique Ids.
    #[error("Cannot make Id non-unique")]
    NonUniqueId,
}