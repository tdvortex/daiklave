use eyre::{eyre, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MotePoolType {
    Peripheral,
    Personal,
}

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
            personal,
        }
    }

    pub fn solar(rating: u8) -> Result<Self> {
        if (1..=5).contains(&rating) {
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

    pub fn recover(&mut self, motes: u8) {
        let peripheral_recovered = self.peripheral.spent.min(motes);
        let personal_recovered = self.personal.spent.min(motes - peripheral_recovered);
        self.peripheral.spent -= peripheral_recovered;
        self.peripheral.available += peripheral_recovered;
        self.personal.spent -= personal_recovered;
        self.personal.available += personal_recovered;
    }

    pub fn spend(&mut self, motes: u8, first: MotePoolType) -> Result<(u8, u8)> {
        if self.peripheral.available + self.personal.available < motes {
            Err(eyre!("Cannot spend {} motes, only have {}", motes, self.peripheral.available + self.personal.available))
        } else {
            let (peripheral_spent, personal_spent) = if first == MotePoolType::Peripheral {
                let peripheral_spent = self.peripheral.available.min(motes);
                (peripheral_spent, self.personal.available.min(motes - peripheral_spent))
            } else {
                let personal_spent = self.personal.available.min(motes);
                (self.peripheral.available.min(motes- personal_spent), personal_spent)
            };
            self.peripheral.spend(peripheral_spent)?;
            self.personal.spend(personal_spent)?;
            Ok((peripheral_spent, personal_spent))
        }
    }

    pub fn commit(&mut self, motes: u8, first: MotePoolType) -> Result<(u8, u8)> {
        if self.peripheral.available + self.personal.available < motes {
            Err(eyre!("Cannot commit {} motes, only have {}", motes, self.peripheral.available + self.personal.available))
        } else {
            let (peripheral_committed, personal_committed) = if first == MotePoolType::Peripheral {
                let peripheral_committed = self.peripheral.available.min(motes);
                (peripheral_committed, self.personal.available.min(motes - peripheral_committed))
            } else {
                let personal_committed = self.personal.available.min(motes);
                (self.peripheral.available.min(motes- personal_committed), personal_committed)
            };
            self.peripheral.commit(peripheral_committed)?;
            self.personal.commit(personal_committed)?;
            Ok((peripheral_committed, personal_committed))
        }
    }

    pub fn uncommit(&mut self, peripheral_uncommit: u8, personal_uncommit: u8) -> Result<()> {
        if self.peripheral.committed < peripheral_uncommit || self.personal.committed < personal_uncommit {
            Err(eyre!("Cannot uncommit {}/{}, current commitment only {}/{}", peripheral_uncommit, personal_uncommit, self.peripheral.committed, self.personal.committed))
        } else {
            self.peripheral.uncommit(peripheral_uncommit)?;
            self.personal.uncommit(personal_uncommit)?;
            Ok(())
        }
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
        self.available += self.spent;
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
            Err(eyre!(
                "Cannot spend {} motes, only {} available",
                motes,
                self.available
            ))
        } else {
            self.available -= motes;
            self.spent += motes;
            Ok(())
        }
    }

    pub fn commit(&mut self, motes: u8) -> Result<()> {
        if motes > self.available {
            Err(eyre!(
                "Cannot commit {} motes, only {} available",
                motes,
                self.available
            ))
        } else {
            self.available -= motes;
            self.committed += motes;
            Ok(())
        }
    }

    pub fn uncommit(&mut self, motes: u8) -> Result<()> {
        if motes > self.committed {
            Err(eyre!(
                "Cannot uncommit {} motes, only {} committed",
                motes,
                self.committed
            ))
        } else {
            self.spent += motes;
            self.committed -= motes;
            Ok(())
        }
    }
}
