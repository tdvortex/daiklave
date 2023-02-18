use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

/// A subdocument of just the OID and name of a character. For use in dropdown
/// selects where deserializing every character is not required.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterStub {
    /// The Id of the character.
    pub character_id: ObjectId,
    /// The name of the character.
    pub name: String,
}
