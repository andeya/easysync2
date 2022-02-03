use std::borrow::Borrow;
use std::fmt::{Display, Formatter};
use std::io::Write;
use std::iter::Peekable;
use std::string::String;

use crate::{apool::AttribPair, digit};
use crate::apool::Apool;
use crate::write_to::WriteTo;

#[derive(Clone)]
enum Monomial {
    ADD(Vec<AttribPair>, u32),
    EQ(Vec<AttribPair>, u32),
    MINUS(u32),
    NEWLINE(u32),
}

#[derive(Clone)]
pub(crate) struct Body<'a> {
    operation: Vec<Monomial>,
    add_num: u32,
    minus_num: u32,
    char_bank: Vec<u8>,
    apool: &'a Box<dyn Apool>,
}

impl<'a> Body<'a> {
    pub(crate) fn char_delta(&self) -> i64 {
        (self.add_num - self.minus_num) as i64
    }
    pub(crate) fn from_iter<I: Iterator<Item=u8>>(apool: &'a Box<dyn Apool>, iter: &mut Peekable<I>) -> anyhow::Result<Self> where Self: Sized {
        todo!()
    }
}

impl<'a> WriteTo for Body<'a> {
    fn write_to(&self, writer: &mut dyn Write) -> anyhow::Result<()> {
        for x in &self.operation {
            match x {
                Monomial::EQ(attrs, count) => {
                    for x in attrs {
                        writer.write_all(b"*")?;
                        writer.write_all(digit::to_vec(x.attrib_num, 36).as_mut())?;
                    }
                    writer.write_all(b"=")?;
                    writer.write_all(unsafe { digit::to_vec(count.clone(), 36).as_mut() })?;
                },
                Monomial::MINUS(count) => {
                    writer.write_all(b"-")?;
                    writer.write_all(unsafe { digit::to_vec(count.clone(), 36).as_mut() })?;
                },
                Monomial::ADD(attrs, count) => {
                    for x in attrs {
                        writer.write_all(b"*")?;
                        writer.write_all(digit::to_vec(x.attrib_num, 36).as_mut())?;
                    }
                    writer.write_all(b"+")?;
                    writer.write_all(unsafe { digit::to_vec(count.clone(), 36).as_mut() })?;
                },
                Monomial::NEWLINE(count) => {
                    writer.write_all(b"|")?;
                    writer.write_all(unsafe { digit::to_vec(count.clone(), 36).as_mut() })?;
                },
            }
        }
        if self.char_bank.len() > 0 {
            writer.write_all(b"$")?;
            writer.write_all(self.char_bank.as_slice())?;
        }
        Ok(())
    }
}

impl<'a> Display for Body<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <Self as WriteTo>::fmt(self, f)
    }
}
