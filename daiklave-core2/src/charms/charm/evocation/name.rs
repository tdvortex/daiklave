use std::ops::Deref;

pub struct EvocationName(String);

impl<T> From<T> for EvocationName where T: ToString {
    fn from(name: T) -> Self {
        Self(name.to_string())
    }
}

impl Deref for EvocationName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}