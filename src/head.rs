use std::fmt::{Display, Formatter};
use std::io::Write;
use std::iter::Peekable;

use crate::digit;
use crate::write_to::WriteTo;

#[derive(Clone)]
pub(crate) struct Head {
    old_len: u32,
    new_len: u32,
}

impl Head {
    pub(crate) fn old_len(&self) -> u32 {
        self.old_len
    }
    pub(crate) fn new_len(&self) -> u32 {
        self.new_len
    }
    pub(crate) fn char_delta(&self) -> i64 {
        (self.new_len - self.old_len) as i64
    }
    fn invalie_payload_error() -> anyhow::Error {
        anyhow::Error::msg("invalid payload head")
    }
    pub(crate) fn from_iter<I: Iterator<Item=u8>>(iter: &mut Peekable<I>) -> anyhow::Result<Self> where Self: Sized {
        let mut buf = vec![];
        type Step = u8;
        let mut step: Step = 0;
        let mut ok = true;
        let mut op = b'_';
        let mut head = Head {
            old_len: 0,
            new_len: 0,
        };
        loop {
            match iter.peek() {
                None => break,
                Some(b) => match b {
                    // step=0
                    b'Z' => {
                        if step != 0 {
                            ok = false;
                            break
                        } else { step += 1 }
                    }
                    // step=1
                    b':' => {
                        if step != 1 {
                            ok = false;
                            break
                        } else { step += 1 }
                    }
                    // step=2,4
                    b'0'..=b'9' | b'a'..=b'z' => {
                        match step {
                            2 | 4 => buf.insert(buf.len(), b.clone()),
                            _ => {
                                ok = false;
                                break;
                            }
                        }
                    }
                    // step=3
                    b'>' | b'<' => {
                        if step != 2 || buf.len() == 0 {
                            ok = false;
                            break
                        } else {
                            op = b.clone();
                            head.old_len = digit::to_num(buf.to_vec(), 36)?;
                            buf.clear();
                            step += 2
                        }
                    }
                    _ => {
                        ok = false;
                        break
                    }
                }
            }
            iter.next();
        }
        if ok && step == 4 || buf.len() > 0 {
            let delta = digit::to_num(buf.to_vec(), 36)?;
            match op {
                b'>' => {
                    head.new_len = head.old_len + delta;
                    ok = true
                }
                b'<' => {
                    if head.old_len >= delta {
                        head.new_len = head.old_len - delta;
                        ok = true
                    }
                }
                _ => {}
            }
        }
        if !ok {
            return Err(Self::invalie_payload_error());
        }
        return Ok(head)
    }
}

impl WriteTo for Head {
    fn write_to(&self, writer: &mut dyn Write) -> anyhow::Result<()> {
        writer.write(b"Z:")?;
        writer.write(digit::to_vec(self.old_len, 36).as_slice())?;
        let mut delta = 0;
        if self.new_len >= self.old_len {
            writer.write(b">")?;
            delta = self.new_len - self.old_len
        } else if self.new_len < self.old_len {
            writer.write(b"<")?;
            delta = self.old_len - self.new_len
        };
        writer.write(digit::to_vec(delta, 36).as_slice())?;
        Ok(())
    }
}

impl Display for Head {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <Self as WriteTo>::fmt(self, f)
    }
}

#[test]
fn head() {
    const S: &'static str = "Z:1>0";
    assert_eq!(S, (Head { old_len: 1, new_len: 1 }).to_string());
    let b = S.as_bytes().iter().map(|item| item.clone());
    assert_eq!(S, Head::from_iter(&mut b.peekable()).unwrap().to_string());
}
