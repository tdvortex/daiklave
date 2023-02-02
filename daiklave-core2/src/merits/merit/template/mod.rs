pub mod builder;
mod nonstackable;
mod stackable;

pub use nonstackable::{NonStackableMeritTemplateName, NonStackableMeritName};
pub use stackable::StackableMeritTemplateName;