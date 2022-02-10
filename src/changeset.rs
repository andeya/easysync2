use std::borrow::BorrowMut;
use std::fmt::{Display, Formatter};
use std::io::Read;
use std::io::Write;
use std::iter::Peekable;

use anyhow::anyhow;

use crate::{apool::{self, Apool}, body::Body, head::Head};
use crate::body::Monomial;
use crate::write_to::WriteTo;

#[derive(Clone)]
pub struct Changeset<'a> {
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
        if head.char_delta() != body.char_delta() {
            return Err(anyhow::Error::msg(format!("wrong data: head.char_delta({}) != body.char_delta({})", head.char_delta(), body.char_delta())));
        }
        Ok(Self {
            head,
            body,
        })
    }
    pub fn follow(&self, next: &Changeset) -> anyhow::Result<Changeset> {
        if self.head.old_len() != next.head.old_len() {
            return Err(anyhow!("old lengths of two changsets are not equal"));
        }
        let old_op = self.body.operation();
        let new_op = next.body.operation();
        let mut new_idx = 0;
        for old in old_op {
            let new = new_op.get(new_idx);

        }
        return Err(anyhow!("old lengths of two changsets are not equal"));
    }
    pub fn compose(&mut self, next: &Changeset) {
        unimplemented!()
    }
}

impl<'a> WriteTo for Changeset<'a> {
    fn write_to(&self, writer: &mut dyn Write) -> anyhow::Result<()> {
        self.head.write_to(writer)?;
        self.body.write_to(writer)
    }
}

impl<'a> Display for Changeset<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <Self as WriteTo>::fmt(self, f)
    }
}

#[test]
fn changeset() {
    let mut mem = crate::apool::Mem::new(1);
    mem.set(apool::AttribPair { id: 4, key: "color:red".to_string() });
    mem.set(apool::AttribPair { id: 5, key: "color:black".to_string() });
    const S: &str = "Z:196>1|5=97=31*4*5+1$x";
    let b = S.as_bytes().iter().map(|item| item.clone());
    let cs = Changeset::from_iter(&mem, &mut b.peekable());
    assert_eq!(S, cs.unwrap_or_else(|e| panic!("{}", e)).to_string());
}
