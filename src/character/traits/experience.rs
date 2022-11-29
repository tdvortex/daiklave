#[derive(Debug, Default)]
pub struct ExperiencePoints {
    pub current: usize,
    pub total: usize,
}

#[derive(Debug, Default)]
pub struct CraftingExperience {
    pub silver: usize,
    pub gold: usize,
    pub white: usize,
    pub major_slots: usize,
}

pub enum CraftingExperienceType {
    Silver,
    Gold,
    White,
}
