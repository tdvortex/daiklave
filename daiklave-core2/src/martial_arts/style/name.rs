use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MartialArtsStyleName(String);

impl<T> From<T> for MartialArtsStyleName where T: ToString {
    fn from(name: T) -> Self {
        Self(name.to_string())
    }
}

impl Deref for MartialArtsStyleName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}