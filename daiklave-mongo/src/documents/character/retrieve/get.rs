use bson::{oid::ObjectId, doc};

use crate::{character::CharacterCurrent, error::DocumentError};

/// A command to retrieve an individual character by its Id.
pub struct GetCharacter {
    /// The MongoDb database Id for this character.
    pub _id: ObjectId,
}

impl GetCharacter {
    /// Gets the characters. Returns Ok(Some) if found, Ok(None) if the command 
    /// executed by no error was found, or Err if the request could not be 
    /// completed.
    pub async fn execute(&self, database: &mongodb::Database) -> Result<Option<CharacterCurrent>, DocumentError> {
        let characters = database.collection::<CharacterCurrent>("characters");
        let filter = doc! {
            "_id": self._id,
        };
        Ok(characters.find_one(filter, None).await?)
    }
}