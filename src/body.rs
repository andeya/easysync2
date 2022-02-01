use std::fmt::{Display, Formatter};
use std::io::Write;
use std::iter::Peekable;
use std::string::String;

use crate::{apool::AttribPair, digit};
use crate::apool::Apool;

#[derive(Clone)]
enum Monomial {
    ADD(Vec<AttribPair>, u32),
    EQ(Vec<AttribPair>, u32),
    MINUS(u32),
    Newline(u32),
}

#[derive(Clone)]
pub(crate) struct Body<'a> {
    operation: Vec<Monomial>,
    add_num: u32,
    minus_num: u32,
    char_bank: Vec<char>,
    apool: &'a Box<dyn Apool>,
}

impl<'a> Body<'a> {
    pub(crate) fn from_iter<I: Iterator<Item=u8>>(apool: &'a Box<dyn Apool>, iter: &mut Peekable<I>) -> anyhow::Result<Self> where Self: Sized {
        todo!()
    }
    fn write_to(&self, writer: &mut dyn Write) -> anyhow::Result<()> {
        todo!()
    }
    pub(crate) fn char_added(&self) -> i64 {
        todo!()
    }
    pub(crate) fn char_deleted(&self) -> i64 {
        todo!()
    }
}

impl<'a> Display for Body<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = Vec::new();
        for x in &self.operation {
            match x {
                Monomial::EQ(attrs, count) => {
                    for x in attrs {
                        s.push(b'*');
                        s.append(digit::to_vec(x.attrib_num, 36).as_mut())
                    }
                    s.push(b'=');
                    s.append(unsafe { digit::to_vec(count.clone(), 36).as_mut() });
                },
                Monomial::MINUS(count) => {
                    s.push(b'-');
                    s.append(unsafe { digit::to_vec(count.clone(), 36).as_mut() });
                },
                Monomial::ADD(attrs, count) => {
                    for x in attrs {
                        s.push(b'*');
                        s.append(digit::to_vec(x.attrib_num, 36).as_mut())
                    }
                    s.push(b'+');
                    s.append(unsafe { digit::to_vec(count.clone(), 36).as_mut() });
                },
                Monomial::Newline(count) => {
                    s.push(b'|');
                    s.append(unsafe { digit::to_vec(count.clone(), 36).as_mut() });
                },
            }
        }
        if self.char_bank.len() > 0 {
            s.push(b'$');
            let mut buf = vec![0; 4];
            for c in &self.char_bank {
                for x in c.encode_utf8(&mut buf).as_bytes() {
                    s.push(x.clone());
                }
            }
        }
        unsafe { write!(f, "{}", String::from_utf8_unchecked(s)) }
    }
}
