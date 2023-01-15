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

impl<'source> SorceryArchetypeMerit {
    /// The name of the merit.
    pub fn name(&'source self) -> &'source str {
        self.name.as_str()
    }
    
    /// The book reference for the merit.
    pub fn book_reference(&self) -> Option<BookReference> {
        self.book_reference
    }

    /// The cost of the merit in dots.
    pub fn dots(&self) -> u8 {
        self.dots
    }

    /// The merit's description.
    pub fn description(&'source self) -> &'source str {
        self.description.as_str()
    }
}