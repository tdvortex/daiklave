use eyre::{eyre, Result};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, Serialize, Deserialize)]
pub struct Essence {
    rating: u8,
    pub peripheral: MotePool,
    pub personal: MotePool,
}

impl Essence {
    pub fn custom(rating: u8, peripheral: MotePool, personal: MotePool) -> Self {
        Self {
            rating, 
            peripheral,
            personal
        }
    }

    pub fn solar(rating: u8) -> Result<Self> {
        if 1 <= rating && rating <= 5 {
            Ok(Self {
                rating,
                peripheral: MotePool::new(rating * 7 + 26),
                personal: MotePool::new(rating * 3 + 10),
            })
        } else if rating < 1 {
            Err(eyre!("Solars must have at least essence 1"))
        } else {
            Err(eyre!("Solars cannot have ratings above 5"))
        }
    }

    pub fn rating(&self) -> u8 {
        self.rating
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, Serialize, Deserialize)]
pub struct MotePool {
    pub available: u8,
    pub committed: u8,
    pub spent: u8,
}

impl MotePool {
    pub fn new(size: u8) -> Self {
        Self {
            available: size,
            committed: 0,
            spent: 0,
        }
    }

    pub fn maximum(&self) -> u8 {
        self.available + self.committed + self.spent
    }

    pub fn recover_all(&mut self) {
        self.available = self.available + self.spent;
        self.spent = 0;
    }

    pub fn recover_one(&mut self) {
        if self.spent > 0 {
            self.spent -= 1;
            self.available += 1;
        }
    }

    pub fn spend(&mut self, motes: u8) -> Result<()> {
        if motes > self.available {
            Err(eyre!("Cannot spend {} motes, only {} available", motes, self.available))
        } else {
            self.available -= motes;
            self.spent += motes;
            Ok(())
        }
    }

    pub fn commit(&mut self, motes: u8) -> Result<()> {
        if motes > self.available {
            Err(eyre!("Cannot commit {} motes, only {} available", motes, self.available))
        } else {
            self.available -= motes;
            self.committed += motes;
            Ok(())
        }
    }

    pub fn uncommit(&mut self, motes: u8) -> Result<()> {
        if motes > self.committed {
            Err(eyre!("Cannot uncommit {} motes, only {} committed", motes, self.committed))
        } else {
            self.available += motes;
            self.committed -= motes;
            Ok(())
        }
    }
}