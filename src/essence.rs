#[derive(Debug, Clone, Copy)]
pub struct Essence {
    pub rating: u8,
    pub personal: MotePool,
    pub peripheral: MotePool,
    pub committed: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct MotePool {
    pub current: u8,
    pub maximum: u8,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Anima {
    Dim,
    Glowing,
    Burning,
    Bonfire,
}

impl Anima {
    pub fn increase(&mut self) {
        match &self {
            Anima::Dim => {*self = Anima::Glowing;}
            Anima::Glowing => {*self = Anima::Burning;}
            Anima::Burning | Anima::Bonfire=> {*self = Anima::Bonfire;}
        };
    }

    pub fn decrease(&mut self) {
        match &self {
            Anima::Dim | Anima::Glowing=> {*self = Anima::Dim;}
            Anima::Burning => {*self = Anima::Glowing;}
            Anima::Bonfire => {*self = Anima::Burning;}
        };
    }
}