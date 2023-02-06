#![warn(missing_docs)]
//! daiklave-mongo abstracts the data layer, including both MongoDb and the
//! cache-aside Redis instance.

mod documents;
pub use documents::*;