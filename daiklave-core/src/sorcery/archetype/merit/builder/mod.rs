use crate::{sorcery::SorceryArchetypeName, book_reference::BookReference, CharacterMutation};

use super::{SorceryArchetypeMeritName, AddSorceryArchetypeMerit, SorceryArchetypeMeritDetails};

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

    pub fn name(self, name: impl Into<SorceryArchetypeMeritName>) -> SorceryArchetypeMeritBuilderWithMeritName {
        SorceryArchetypeMeritBuilderWithMeritName {
            archetype_name: self.archetype_name,
            merit_name: name.into(),
            book_reference: self.book_reference,
        }
    }
}

pub struct SorceryArchetypeMeritBuilderWithMeritName {
    archetype_name: SorceryArchetypeName,
    merit_name: SorceryArchetypeMeritName,
    book_reference: Option<BookReference>,
}

impl SorceryArchetypeMeritBuilderWithMeritName {
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    pub fn description(self, description: impl Into<String>) -> SorceryArchetypeMeritBuilderWithDescription {
        SorceryArchetypeMeritBuilderWithDescription {
            archetype_name: self.archetype_name,
            merit_name: self.merit_name,
            description: description.into(),
            book_reference: self.book_reference,
        }
    }
}

pub struct SorceryArchetypeMeritBuilderWithDescription {
    archetype_name: SorceryArchetypeName,
    merit_name: SorceryArchetypeMeritName,
    description: String,
    book_reference: Option<BookReference>,
}

impl SorceryArchetypeMeritBuilderWithDescription {
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    pub fn dots(self, dots: u8) -> SorceryArchetypeMeritBuilderWithDots {
        SorceryArchetypeMeritBuilderWithDots {
            archetype_name: self.archetype_name,
            merit_name: self.merit_name,
            description: self.description,
            dots,
            book_reference: self.book_reference,
        }
    }
}

pub struct SorceryArchetypeMeritBuilderWithDots {
    archetype_name: SorceryArchetypeName,
    merit_name: SorceryArchetypeMeritName,
    description: String,
    dots: u8,
    book_reference: Option<BookReference>,
}

impl SorceryArchetypeMeritBuilderWithDots {
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    pub fn build(self) -> AddSorceryArchetypeMerit {
        AddSorceryArchetypeMerit {
            archetype_name: self.archetype_name,
            merit_name: self.merit_name,
            merit: SorceryArchetypeMeritDetails { 
                book_reference: self.book_reference, 
                dots: self.dots, 
                description: self.description, 
            }
        }
    }
}

impl From<SorceryArchetypeMeritBuilderWithDots> for CharacterMutation {
    fn from(builder: SorceryArchetypeMeritBuilderWithDots) -> Self {
        AddSorceryArchetypeMerit::from(builder).into()
    }
}