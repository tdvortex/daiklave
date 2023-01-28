use std::ops::Deref;

pub struct OtherMoteCommitmentName(String);

impl<T> From<T> for OtherMoteCommitmentName where T: ToString {
    fn from(name: T) -> Self {
        Self(name.to_string())
    }
}

impl Deref for OtherMoteCommitmentName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}