use std::{
    collections::{HashMap, HashSet},
    num::NonZeroU8,
};

use crate::{
    book_reference::BookReference,
    charms::{
        charm::{
            spirit::{
                inner::SpiritCharmInner, noneclipse::AddNonEclipseCharm, AddEclipseCharm,
                AddSpiritCharm, EclipseCharm, NonEclipseCharm, SpiritCharm,
            },
            SpiritCharmKeyword,
        },
        CharmActionType, CharmCostType,
    },
};

/// A Spirit Charm builder after the description has been provided. Complete
/// the builder using build().
pub struct SpiritCharmBuilderWithDescription {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) summary: Option<String>,
    pub(crate) keywords: HashSet<SpiritCharmKeyword>,
    pub(crate) costs: HashMap<CharmCostType, NonZeroU8>,
    pub(crate) essence_required: NonZeroU8,
    pub(crate) action_type: CharmActionType,
    pub(crate) duration: String,
    pub(crate) description: String,
}

impl SpiritCharmBuilderWithDescription {
    /// Sets the book reference for the Evocation.
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    /// Sets a summary for the evocation.
    pub fn summary(mut self, summary: String) -> Self {
        self.summary = Some(summary);
        self
    }

    /// Adds a keyword to this Charm.
    pub fn keyword(mut self, keyword: SpiritCharmKeyword) -> Self {
        self.keywords.insert(keyword);
        self
    }

    /// Adds a cost to use this Charm.
    pub fn cost(mut self, cost_type: CharmCostType, amount: NonZeroU8) -> Self {
        self.costs
            .entry(cost_type)
            .and_modify(|prior| {
                *prior = (*prior).saturating_add(amount.get());
            })
            .or_insert(amount);

        self
    }

    fn build_inner(self) -> (String, SpiritCharmInner) {
        (
            self.name,
            SpiritCharmInner {
                book_reference: self.book_reference,
                summary: self.summary,
                description: self.description,
                essence_required: self.essence_required,
                keywords: self.keywords,
                costs: self.costs,
                action_type: self.action_type,
                duration: self.duration,
            },
        )
    }

    /// Finishes the builder, returning an Eclipse charm.
    pub fn build_eclipse(self) -> AddEclipseCharm {
        let (name, inner) = self.build_inner();
        (name, EclipseCharm(inner))
    }

    /// Finishes the builder, returning a non-Eclipse Spirit charm.
    pub fn build_non_eclipse(self) -> AddNonEclipseCharm {
        let (name, inner) = self.build_inner();
        (name, NonEclipseCharm(inner))
    }

    /// Finishes the builder, retuning a Spirit charm, either Eclipse or
    /// Non-Eclipse as specified.
    pub fn build(self, eclipse: bool) -> AddSpiritCharm {
        if eclipse {
            let (name, eclipse) = self.build_eclipse();
            (name, SpiritCharm::Eclipse(eclipse))
        } else {
            let (name, noneclipse) = self.build_non_eclipse();
            (name, SpiritCharm::NonEclipse(noneclipse))
        }
    }
}
