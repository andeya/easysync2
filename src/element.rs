use std::fmt::Display;
use std::io::Write;
use std::iter::Peekable;

pub(crate) trait Element: Display {
    fn from_iter<I: Iterator<Item=u8>>(iter: &mut Peekable<I>) -> anyhow::Result<Self> where Self: Sized;
    fn write_to(&self, writer: &mut dyn Write) -> anyhow::Result<()>;
}
