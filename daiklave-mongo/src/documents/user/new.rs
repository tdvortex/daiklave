use bson::oid::ObjectId;
use mongodb::results::InsertOneResult;
use serde::{Serialize, Deserialize};
use serenity::all::UserId;

use crate::error::DocumentError;

use super::versions::UserVersion;

/// A document to insert a new User.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "user")]
#[serde(rename_all = "camelCase")]
pub struct NewUser {
    /// The version of the User document to be inserted.
    pub version: UserVersion,
    /// The Discord snowflake for this user.
    pub discord_id: UserId,
}

impl NewUser {
    /// Inserts a new user into the "users" collection with no campaigns. No
    /// session is required here as the update is atomic.
    pub async fn create(&self, database: &mongodb::Database) -> Result<ObjectId, DocumentError> {
        let users = database.collection::<NewUser>("users");

        let InsertOneResult {
            inserted_id,
            ..
        } = users.insert_one(self, None).await?;

        inserted_id.as_object_id().ok_or(DocumentError::DeserializationError)
    }
}