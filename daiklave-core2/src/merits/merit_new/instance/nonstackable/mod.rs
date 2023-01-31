mod add;
mod remove;
pub use add::AddNonStackableMerit;
pub use remove::RemoveNonStackableMerit;

use super::inner::MeritInstanceInner;

pub(crate) struct NonStackableMeritInstance(pub MeritInstanceInner);