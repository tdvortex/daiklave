#![warn(missing_docs)]
//! **Daiklave** is a Rust character sheet application, designed to be as
//! flexible as a paper sheet, as easy to use as a virtual tabletop (VTT),
//! with full Discord integration for over-the-internet play.

use serde::{Serialize, Deserialize};

/// An owned instance of a full (player) character. This is the format used in
/// serialization and deserialization.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Character;

/// A read-only instance of a Character which references a CharacterEventSource
/// object, using &str instead of String. 
pub struct CharacterView;

/// The API for the character, expressed as an owned struct. Each mutation has
/// an associated pub method on Character and CharacterEventSource which 
/// returns Result<&mut Self, CharacterMutationError>. All API events also has
///  a "check_" variant which returns Result<(), CharacterMutationError>. 
pub enum CharacterMutation {}

/// An error representing something that could go wrong with a 
/// CharacterMutation.
pub enum CharacterMutationError {}

/// A container to hold a successfully applied sequence of mutations, with
/// capability to undo/redo mutations. 
pub struct CharacterEventSource{}