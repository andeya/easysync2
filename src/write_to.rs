use std::fmt::Formatter;
use std::io::Write;

pub(crate) trait WriteTo {
    fn write_to(&self, writer: &mut dyn Write) -> anyhow::Result<()>;
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut w = Vec::new();
        self.write_to(&mut w).unwrap();
        write!(f, "{}", unsafe { String::from_utf8_unchecked(w) })
    }
}
