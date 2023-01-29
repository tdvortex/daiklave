use crate::{sorcery::SorceryArchetypeName, book_reference::BookReference};

pub struct SorceryArchetypeMeritBuilder {
    archetype_name: SorceryArchetypeName,
    book_reference: Option<BookReference>,
}

impl SorceryArchetypeMeritBuilder {
    pub fn archetype_name(archetype_name: impl Into<SorceryArchetypeName>) -> Self {
        Self {
            archetype_name: archetype_name.into(),
            book_reference: None,
        }
    }

    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    pub fn summary(self, summary: impl ToString) -> SorceryArchetypeMeritBuilderWithSummary {
        SorceryArchetypeMeritBuilderWithSummary {
            archetype_name: self.archetype_name,
            summary: summary.to_string(),
            book_reference: self.book_reference,
        }
    }
}

pub struct SorceryArchetypeMeritBuilderWithSummary {
    archetype_name: SorceryArchetypeName,
    summary: String,
    book_reference: Option<BookReference>,
}

impl SorceryArchetypeMeritBuilderWithSummary {
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    pub fn description(self, description: impl ToString) -> SorceryArchetypeMeritBuilderWithDescription {
        SorceryArchetypeMeritBuilderWithDescription {
            archetype_name: self.archetype_name,
            summary: self.summary,
            description: description.to_string(),
            book_reference: self.book_reference,
        }
    }
}