use std::collections::{HashMap, HashSet, hash_map::Entry};

use crate::{book_reference::BookReference, merits::merit_new::{MeritType, MeritPrerequisite}};

use super::{VariableStackableMeritTemplateBuilder, nonstackable::VariableNonStackableMeritTemplateBuilder};

pub struct VariableMeritTemplateBuilderWithDots {
    pub(crate) name: String,
    pub(crate) merit_type: MeritType,
    pub(crate) description: String,
    pub(crate) min_dots: (u8, String),
    pub(crate) other_dots: HashMap<u8, String>,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) prerequisites: HashSet<MeritPrerequisite>,
}

impl VariableMeritTemplateBuilderWithDots {
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    pub fn prerequisite(mut self, prerequisite: MeritPrerequisite) -> Self {
        self.prerequisites.insert(prerequisite);
        self
    }

    pub fn dot_option(mut self, dots: u8, description: impl Into<String>) -> Self {
        match dots.cmp(&self.min_dots.0) {
            std::cmp::Ordering::Less => {
                let (old_min, old_min_description) = self.min_dots;
                self.min_dots = (dots, description.into());
    
                if let Entry::Vacant(e) = self.other_dots.entry(old_min) {
                    e.insert(old_min_description);
                }
            }
            std::cmp::Ordering::Equal => {
                self.min_dots.1 = description.into();
            }
            std::cmp::Ordering::Greater => {
                if let Entry::Vacant(e) = self.other_dots.entry(dots) {
                    e.insert(description.into());
                }
            }
        }
    
        self
    }

    pub fn nonstackable(self) -> VariableNonStackableMeritTemplateBuilder {
        VariableNonStackableMeritTemplateBuilder(self)
    }

    pub fn stackable(self) -> VariableStackableMeritTemplateBuilder {
        VariableStackableMeritTemplateBuilder(self)
    }
}