mod add;
pub mod builder;
mod name;
mod remove;
mod text;

pub use add::AddManse;
pub use name::ManseName;
pub use remove::RemoveManse;
pub(crate) use text::*;