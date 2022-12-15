use crate::{charms::MartialArtsCharm, data_source::DataSource, id::Id};
use eyre::{eyre, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, Serialize, Deserialize)]
pub struct MartialArtsStyle {
    id: Id,
    name: String,
    description: Option<String>,
    data_source: DataSource,
}

impl PartialEq for MartialArtsStyle {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MartialArtistDetails {
    style: MartialArtsStyle,
    dots: u8,
    charms: Vec<MartialArtsCharm>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MartialArtistTraits(Vec<MartialArtistDetails>);

impl MartialArtistTraits {
    pub fn add_style(&mut self, style: MartialArtsStyle, dots: u8) -> Result<()> {
        if self.0.iter().any(|details| details.style == style) {
            Err(eyre!("Style {} already known", style.name))
        } else {
            self.0.push(MartialArtistDetails {
                style,
                dots,
                charms: Default::default(),
            });
            self.0
                .sort_by(|a, b| a.style.name.as_str().cmp(b.style.name.as_str()));
            Ok(())
        }
    }
}
