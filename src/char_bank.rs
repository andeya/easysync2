use std::fmt::{Display, Formatter};
use std::io::Write;
use std::iter::Peekable;

use crate::element::Element;

pub(crate) struct CharBank(String);

impl CharBank {
    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }
}

impl Display for CharBank {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Element for CharBank {
    fn from_iter<I: Iterator<Item=u8>>(iter: &mut Peekable<I>) -> anyhow::Result<Self> where Self: Sized {
        todo!()
    }

    fn write_to(&self, writer: &mut dyn Write) -> anyhow::Result<()> {
        todo!()
    }
}
