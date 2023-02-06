mod new;
mod versions;
pub use new::NewCharacter;
pub use versions::{CharacterCurrent, CharacterV0};

/// A versioned Character document.
#[derive(Debug)]
pub enum CharacterDocument {
    /// Version 0
    V0(CharacterV0),
}

