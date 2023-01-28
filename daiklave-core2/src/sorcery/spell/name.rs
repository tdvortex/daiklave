use std::ops::Deref;

pub struct SpellName(String);

impl<T> From<T> for SpellName where T: ToString {
    fn from(name: T) -> Self {
        Self(name.to_string())
    }
}

impl Deref for SpellName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}