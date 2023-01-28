use std::ops::Deref;

pub struct ArtifactWeaponName(String);

impl<T> From<T> for ArtifactWeaponName where T: ToString {
    fn from(name: T) -> Self {
        Self(name.to_string())
    }
}

impl Deref for ArtifactWeaponName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}