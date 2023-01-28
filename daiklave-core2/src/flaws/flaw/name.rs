use std::ops::Deref;

pub struct FlawName(String);

impl<T> From<T> for FlawName where T: ToString {
    fn from(name: T) -> Self {
        Self(name.to_string())
    }
}

impl Deref for FlawName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}