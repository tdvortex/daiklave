use crate::unique_id::UniqueId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StackableMeritId(pub UniqueId);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StackableMeritTemplateId(pub UniqueId);