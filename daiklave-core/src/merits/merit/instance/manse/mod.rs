mod add;

/// A builder path to construct a new merit.
pub mod builder;
mod name;
mod remove;
mod text;

pub use add::AddManse;
pub use name::ManseName;
pub use remove::RemoveManse;
pub(crate) use text::*;
