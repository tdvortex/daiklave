use std::ops::Deref;

pub struct LocalTongueName(String);

impl<T> From<T> for LocalTongueName where T: ToString {
    fn from(name: T) -> Self {
        Self(name.to_string())
    }
}

impl Deref for LocalTongueName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}