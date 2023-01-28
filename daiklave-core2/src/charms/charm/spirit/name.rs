use std::ops::Deref;

pub struct SpiritCharmName(String);

impl<T> From<T> for SpiritCharmName where T: ToString {
    fn from(name: T) -> Self {
        Self(name.to_string())
    }
}

impl Deref for SpiritCharmName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}