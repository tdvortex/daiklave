use core::fmt::Debug;
use mongodb::bson::Bson;
use serde::Serialize;

use super::error::DatabaseError;

/// Helper function to serialize a value as a Bson object, or return
/// [DatabaseError::SerializationError] with the debug string.
pub fn to_bson<T>(value: &T) -> Result<Bson, DatabaseError>
where
    T: Debug + Serialize,
{
    mongodb::bson::to_bson(value)
        .map_err(|_| DatabaseError::SerializationError(format!("{:?}", value)))
}
