use std::fmt::{Display, Formatter};
use std::io::Write;
use std::iter::Peekable;

use crate::{apool::AttribPair, digit};
use crate::apool::Apool;
use crate::write_to::WriteTo;

#[derive(Clone)]
pub(crate) enum Monomial {
    ADD { attrib_pair: Vec<AttribPair>, contains_newline_num: u32, add_num: u32 },
    EQ { attrib_pair: Vec<AttribPair>, contains_newline_num: u32, eq_num: u32 },
    MINUS { contains_newline_num: u32, minus_num: u32 },
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
        let mut body = Body {
            operation: vec![],
            add_num: 0,
            minus_num: 0,
            char_bank: vec![],
            apool,
        };
        'L: loop {
            let mut step: u8 = 0;
            let mut attrib_pair: Vec<AttribPair> = vec![];
            let mut contains_newline_num: u32 = 0;
            loop {
                match iter.peek() {
                    None => return Ok(body),
                    Some(x) => {
                        match x {
                            // attributes
                            b'*' => {
                                if step > 0 {
                                    break
                                }
                                let _ = iter.next();
                                let num = digit::from_iter(iter)?;
                                if let Some(attr) = body.apool.get_attrib(num) {
                                    attrib_pair.push(attr.clone())
                                }
                            }
                            // contains newlines
                            b'|' => {
                                if contains_newline_num > 0 || step > 1 {
                                    break
                                }
                                let _ = iter.next();
                                step = 1;
                                contains_newline_num = digit::from_iter(iter)?;
                            }
                            // delete
                            b'-' => {
                                if step > 2 {
                                    break
                                }
                                let _ = iter.next();
                                let minus_num = digit::from_iter(iter)?;
                                if minus_num > 0 {
                                    body.operation.push(Monomial::MINUS { contains_newline_num, minus_num: 0 });
                                    body.minus_num += minus_num;
                                }
                                break
                            }
                            // add
                            b'+' => {
                                if step > 2 {
                                    break
                                }
                                let _ = iter.next();
                                let add_num = digit::from_iter(iter)?;
                                if add_num > 0 {
                                    body.operation.push(Monomial::ADD { attrib_pair, contains_newline_num, add_num });
                                    body.add_num += add_num;
                                }
                                break
                            }
                            // eq
                            b'=' => {
                                if step > 2 {
                                    break
                                }
                                let _ = iter.next();
                                let eq_num = digit::from_iter(iter)?;
                                if eq_num > 0 {
                                    body.operation.push(Monomial::EQ { attrib_pair, contains_newline_num, eq_num });
                                }
                                break
                            }
                            _ => break 'L,
                        }
                    }
                }
            }
        }
        if let Some(x) = iter.peek() {
            if x == &b'$' {
                let _ = iter.next();
                body.char_bank = iter.collect();
            }
        };
        return Ok(body)
    }
    pub(crate) fn operation(&self) -> &Vec<Monomial> {
        &self.operation
    }
}

impl<'a> WriteTo for Body<'a> {
    fn write_to(&self, writer: &mut dyn Write) -> anyhow::Result<()> {
        for x in &self.operation {
            match x {
                Monomial::MINUS { contains_newline_num, minus_num, } => {
                    if contains_newline_num > &0_u32 {
                        writer.write_all(b"|")?;
                        writer.write_all(digit::to_vec(contains_newline_num.clone(), 36).as_mut())?;
                    }
                    writer.write_all(b"-")?;
                    writer.write_all(digit::to_vec(minus_num.clone(), 36).as_mut())?;
                },
                Monomial::EQ { attrib_pair, eq_num, contains_newline_num } => {
                    for x in attrib_pair {
                        writer.write_all(b"*")?;
                        writer.write_all(digit::to_vec(x.id, 36).as_mut())?;
                    }
                    if contains_newline_num > &0_u32 {
                        writer.write_all(b"|")?;
                        writer.write_all(digit::to_vec(contains_newline_num.clone(), 36).as_mut())?;
                    }
                    writer.write_all(b"=")?;
                    writer.write_all(digit::to_vec(eq_num.clone(), 36).as_mut())?;
                },
                Monomial::ADD { attrib_pair, add_num, contains_newline_num } => {
                    for x in attrib_pair {
                        writer.write_all(b"*")?;
                        writer.write_all(digit::to_vec(x.id, 36).as_mut())?;
                    }
                    if contains_newline_num > &0_u32 {
                        writer.write_all(b"|")?;
                        writer.write_all(digit::to_vec(contains_newline_num.clone(), 36).as_mut())?;
                    }
                    writer.write_all(b"+")?;
                    writer.write_all(digit::to_vec(add_num.clone(), 36).as_mut())?;
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
