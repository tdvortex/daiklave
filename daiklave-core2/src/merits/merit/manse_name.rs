use std::ops::Deref;

pub struct ManseName(String);

impl<T> From<T> for ManseName where T: ToString {
    fn from(name: T) -> Self {
        Self(name.to_string())
    }
}

impl Deref for ManseName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}