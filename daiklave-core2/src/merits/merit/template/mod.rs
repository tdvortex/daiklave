/// A builder path to construct a new merit.
pub mod builder;
mod nonstackable;
mod stackable;

pub use nonstackable::{NonStackableMeritName, NonStackableMeritTemplateName};
pub use stackable::StackableMeritTemplateName;
