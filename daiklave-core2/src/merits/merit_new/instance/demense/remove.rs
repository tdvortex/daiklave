use super::DemenseName;

pub struct RemoveDemense(pub DemenseName);

impl RemoveDemense {
    pub fn name(name: impl Into<DemenseName>) -> Self {
        Self(name.into())
    }
}

impl From<DemenseName> for RemoveDemense {
    fn from(name: DemenseName) -> Self {
        todo!()
    }
}