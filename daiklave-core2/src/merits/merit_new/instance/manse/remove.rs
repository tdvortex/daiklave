use super::ManseName;

pub struct RemoveManse(pub ManseName);

impl RemoveManse {
    pub fn name(name: impl Into<ManseName>) -> Self {
        Self(name.into())
    }
}