use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LimitTrigger(String);

impl<T> From<T> for LimitTrigger where T: ToString {
    fn from(name: T) -> Self {
        Self(name.to_string())
    }
}

impl Deref for LimitTrigger {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}