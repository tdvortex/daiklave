use std::ops::Deref;

pub struct MartialArtsCharmName(String);

impl<T> From<T> for MartialArtsCharmName where T: ToString {
    fn from(name: T) -> Self {
        Self(name.to_string())
    }
}

impl Deref for MartialArtsCharmName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}