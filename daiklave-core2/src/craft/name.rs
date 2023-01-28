use std::ops::Deref;

pub struct CraftName(String);

impl<T> From<T> for CraftName where T: ToString {
    fn from(name: T) -> Self {
        Self(name.to_string())
    }
}

impl Deref for CraftName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}