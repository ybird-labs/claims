use super::{AssertedAt, AssertorIri};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct AssertionProvenance {
    assertor: AssertorIri,
    asserted_at: AssertedAt,
}

impl AssertionProvenance {
    pub fn new(assertor: AssertorIri, asserted_at: AssertedAt) -> Self {
        Self {
            assertor,
            asserted_at,
        }
    }

    pub fn assertor(&self) -> &AssertorIri {
        &self.assertor
    }

    pub fn asserted_at(&self) -> AssertedAt {
        self.asserted_at
    }
}
