use super::inner::MeritInstanceInner;

mod add;
mod remove;

pub use add::AddStackableMerit;
pub use remove::RemoveStackableMerit;

pub(crate) struct StackableMeritInstance(pub MeritInstanceInner);