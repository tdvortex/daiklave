use crate::unique_id::UniqueId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NonStackableMeritId(pub UniqueId);