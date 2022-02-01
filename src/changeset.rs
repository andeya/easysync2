use std::borrow::BorrowMut;
use std::fmt::{Display, Formatter};
use std::io::Read;
use std::io::Write;
use std::iter::Peekable;

use crate::{body::Body, head::Head};
use crate::apool::Apool;

#[derive(Clone)]
pub(crate) struct Changeset<'a> {
    head: Head,
    body: Body<'a>,
}

impl<'a> Changeset<'a> {
    pub(crate) fn from_reader(apool: &'a Box<dyn Apool>, reader: &mut dyn Read) -> anyhow::Result<Self> {
        return Changeset::from_iter(apool, reader
            .bytes()
            .map_while(|item| item.ok()).peekable().borrow_mut());
    }
    fn from_iter<I: Iterator<Item=u8>>(apool: &'a Box<dyn Apool>, iter: &mut Peekable<I>) -> anyhow::Result<Self> {
        let head = Head::from_iter(iter)?;
        let body = Body::from_iter(apool, iter)?;
        if head.char_delta() != body.char_added() - body.char_deleted() {
            return Err(anyhow::Error::msg("wrong data"))
        }
        Ok(Self {
            head,
            body,
        })
    }
    fn write_to(&self, writer: &mut dyn Write) -> anyhow::Result<()> {
        todo!()
    }
    fn follow(&self, next: &Changeset) -> Changeset {
        unimplemented!()
    }
    pub(crate) fn compose(&mut self, next: &Changeset) {
        unimplemented!()
    }
}

impl<'a> Display for Changeset<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.head, self.body)
    }
}


#[test]
fn changeset() {
    let mem = crate::apool::Mem::new(1);
    let mut b = "Z:1>0".as_bytes().iter().map(|item| item.clone());
    assert_eq!("Z:1>0", Changeset::from_iter(&mem, &mut b.peekable()).unwrap().to_string());
}
