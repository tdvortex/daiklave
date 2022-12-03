#[derive(Debug, Default)]
pub struct ExperiencePoints {
    pub current: u16,
    pub total: u16,
}

#[derive(Debug, Default)]
pub struct CraftingExperience {
    pub silver: u16,
    pub gold: u16,
    pub white: u16,
    pub major_slots: u16,
}

pub enum CraftingExperienceType {
    Silver,
    Gold,
    White,
}
