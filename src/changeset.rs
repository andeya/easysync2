use std::array::IntoIter;
use std::borrow::BorrowMut;
use std::fmt::{Display, Formatter};
use std::io::Read;
use std::io::Write;
use std::iter::Peekable;
use std::str::Bytes;

use crate::{char_bank::CharBank, element::Element, head::Head, operation::Operation};

struct Changeset {
    head: Head,
    ops: Operation,
    char_bank: CharBank,
}

impl Changeset {
    fn from_reader(reader: &mut dyn Read) -> anyhow::Result<Changeset> {
        return Changeset::from_iter(reader
            .bytes()
            .map_while(|item| item.ok()).peekable().borrow_mut());
    }
    fn follow(&self, next: &Changeset) -> Changeset {
        unimplemented!()
    }
    fn compose(&self, next: &Changeset) -> Changeset {
        unimplemented!()
    }
}

impl Display for Changeset {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.head, self.ops, self.char_bank)
    }
}

impl Element for Changeset {
    fn from_iter<I: Iterator<Item=u8>>(iter: &mut Peekable<I>) -> anyhow::Result<Self> where Self: Sized {
        let head = Head::from_iter(iter)?;
        let ops: Operation = Operation::from_iter(iter)?;
        let delta = head.char_delta();
        if delta != ops.char_added() - ops.char_deleted() {
            return Err(anyhow::Error::msg("wrong data"))
        }
        let char_bank = CharBank::from_iter(iter)?;
        if char_bank.len() as i64 != ops.char_added() {
            return Err(anyhow::Error::msg("wrong data"))
        }
        Ok(Self {
            head,
            ops,
            char_bank,
        })
    }

    fn write_to(&self, writer: &mut dyn Write) -> anyhow::Result<()> {
        todo!()
    }
}


#[test]
fn changeset() {
    let mut b = "Z:1>0".as_bytes().iter().map(|item| item.clone());
    assert_eq!("Z:1>0", Changeset::from_iter(&mut b.peekable()).unwrap().to_string());
}
