use std::fmt::{Display, Formatter};
use std::io::Write;
use std::iter::Peekable;
use std::string::String;

use crate::{apool::AttribPair, digit, element::Element};

enum Monomial {
    ADD(Vec<AttribPair>, u32),
    EQ(Vec<AttribPair>, u32),
    MINUS(u32),
    Newline(u32),
}

pub(crate) struct Operation {
    polynomial: Vec<Monomial>,
    add_num: u32,
    minus_num: u32,
}

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
        let mut s = Vec::new();
        for x in &self.polynomial {
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
        unsafe { write!(f, "{}", String::from_utf8_unchecked(s)) }
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
