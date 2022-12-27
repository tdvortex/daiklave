use std::ops::Deref;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{Character, CharacterMutationError, CharacterView};

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
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash, Default,
)]
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

impl Character {
    /// Returns the character's id.
    pub fn id(&self) -> CharacterId {
        self.id
    }

    /// Checks if the character's Id can be set.
    pub fn check_set_id(&self, id: CharacterId) -> Result<(), CharacterMutationError> {
        if let Id::Database(_) = *self.id {
            Err(CharacterMutationError::SetIdError(
                SetIdError::DatabaseIdExists,
            ))
        } else if let Id::NonUnique = *id {
            Err(CharacterMutationError::SetIdError(SetIdError::NonUniqueId))
        } else {
            Ok(())
        }
    }

    /// Sets the character's Id.
    pub fn set_id(&mut self, id: CharacterId) -> Result<&mut Self, CharacterMutationError> {
        self.check_set_id(id)?;
        self.id = id;
        Ok(self)
    }
}

impl CharacterView {
    /// Returns the character's id.
    pub fn id(&self) -> CharacterId {
        self.id
    }

    /// Checks if the character's Id can be set.
    pub fn check_set_id(&self, id: CharacterId) -> Result<(), CharacterMutationError> {
        if let Id::Database(_) = *self.id {
            Err(CharacterMutationError::SetIdError(
                SetIdError::DatabaseIdExists,
            ))
        } else if let Id::NonUnique = *id {
            Err(CharacterMutationError::SetIdError(SetIdError::NonUniqueId))
        } else {
            Ok(())
        }
    }

    /// Sets the character's Id.
    pub fn set_id(&mut self, id: CharacterId) -> Result<&mut Self, CharacterMutationError> {
        self.check_set_id(id)?;
        self.id = id;
        Ok(self)
    }
}
