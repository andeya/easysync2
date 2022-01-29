use std::fmt::{Display, Formatter};
use std::io::Write;
use std::iter::Peekable;

use crate::element::Element;

pub(crate) struct Operation(String);

impl Operation {
    pub(crate) fn char_added(&self) -> i64 {
        todo!()
    }
    pub(crate) fn char_deleted(&self) -> i64 {
        todo!()
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Element for Operation {
    fn from_iter<I: Iterator<Item=u8>>(iter: &mut Peekable<I>) -> anyhow::Result<Self> where Self: Sized {
        todo!()
    }

    fn write_to(&self, writer: &mut dyn Write) -> anyhow::Result<()> {
        todo!()
    }
}
