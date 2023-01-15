mod id;
pub use id::SorceryArchetypeMeritId;
use serde::{Serialize, Deserialize};

use crate::book_reference::BookReference;

/// A merit which is made available to sorcerers following a specific 
/// Archetype.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SorceryArchetypeMerit {
    name: String,
    book_reference: Option<BookReference>,
    dots: u8,
    description: String,
}