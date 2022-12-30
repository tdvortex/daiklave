use serde::{Deserialize, Serialize};

/// A unique identifier for a character component or referenced item. All other Id
/// subtypes should Deref to this type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub enum UniqueId {
    /// Used as an offline key before saving to the database. Uniqueness is
    /// maintained client-side. Stored as a u32 to prevent accidental
    /// cross-contamination with Id::Database.
    Placeholder(u32),
    /// The Id as stored in the database. i32 is equivalent to Integer in
    /// Postgres and most other SQL engines.
    Database(i32),
}