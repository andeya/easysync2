use std::io::Read;
use std::io::Write;
use std::iter::Peekable;

struct Head {
    old_len: u32,
    new_len: u32,
}

mod digit {
    pub fn to_num(bytes: Vec<u8>, radix: u32) -> anyhow::Result<u32> {
        if radix > 36 {
            panic!("to_num: radix is too high (maximum 36)");
        }
        let mut u: u32 = 0;
        for c in bytes {
            u *= radix;
            match c {
                b'0'..=b'9' => u += (c - b'0') as u32,
                b'a'..=b'z' => u += (c - b'a') as u32,
                _ => return Err(anyhow::Error::msg(format!("to_num: invalid char {}", c as char))),
            }
        }
        return Ok(u);
    }

    #[test]
    fn test_to_num() {
        let res = to_num("vec![]".into(), 36);
        match res {
            Err(e) => assert_eq!("to_num: invalid char !", e.to_string()),
            Ok(_) => panic!("fail"),
        }
        let res = to_num("10".into(), 36);
        match res {
            Err(_) => panic!("fail"),
            Ok(u) => assert_eq!(36, u),
        }
    }

    pub fn to_vec(num: u32, radix: u32) -> Vec<u8> {
        if radix > 36 {
            panic!("to_vec: radix is too high (maximum 36)");
        }
        let mut num = num;
        let mut vec: Vec<u8> = Vec::new();
        loop {
            if num == 0 {
                break;
            }
            let u = (num % radix) as u8;
            let c = if u < 10 { b'0' + u } else { b'a' + u - 10 };
            vec.insert(0, c);
            num = num / radix;
        }
        return vec;
    }

    pub fn is_valid(b: u8) -> bool {
        match b {
            b'0'..=b'9' | b'a'..=b'z' => true,
            _ => false,
        }
    }
}

impl Head {
    fn invalie_payload_error() -> anyhow::Error {
        anyhow::Error::msg("invalid payload head")
    }
    fn from_iter(iter: &mut Peekable<&mut dyn Iterator<Item=u8>>) -> anyhow::Result<Head> {
        let mut buf = vec![];
        type Step = u8;
        let mut step: Step = 0;
        let mut ok = false;
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
                    b'Z' => { if step != 0 { break } else { step += 1 } }
                    // step=1
                    b':' => { if step != 1 { break } else { step += 1 } }
                    // step=2,4
                    b'0'..=b'9' | b'a'..=b'z' => {
                        match step {
                            2 | 4 => buf.insert(buf.len(), b.clone()),
                            _ => break
                        }
                    }
                    // step=3
                    b'>' | b'<' | b'=' => {
                        if step != 3 { break } else {
                            op = b.clone();
                            head.old_len = digit::to_num(buf.to_vec(), 36)?;
                            buf.clear();
                            step += 1
                        }
                    }
                    _ => {
                        break
                    }
                }
            }
            iter.next();
        }
        if ok && step == 4 || buf.len() > 0 {
            head.new_len = digit::to_num(buf.to_vec(), 36)?;
            match op {
                b'>' => {
                    ok = head.new_len > head.old_len
                }
                b'<' => {
                    ok = head.new_len < head.old_len
                }
                b'=' => {
                    ok = head.new_len == head.old_len
                }
                _ => {}
            }
        }
        if !ok {
            return Err(Self::invalie_payload_error());
        }
        return Ok(head)
    }

    fn write_to(&self, writer: &mut dyn Write) -> anyhow::Result<()> {
        writer.write(b"Z:")?;
        writer.write(digit::to_vec(self.old_len, 36).as_slice())?;
        if self.new_len > self.old_len {
            writer.write(b">")?;
        } else if self.new_len < self.old_len {
            writer.write(b"<")?;
        } else {
            writer.write(b"=")?;
        };
        writer.write(digit::to_vec(self.new_len, 36).as_slice())?;
        Ok(())
    }

    fn to_string(&self) -> String {
        let mut w = Vec::new();
        self.write_to(&mut w).unwrap();
        unsafe { String::from_utf8_unchecked(w) }
    }
}

#[test]
fn head() {
    assert_eq!("Z:1>10", (Head { old_len: 1, new_len: 36 }).to_string());
}

struct Operation(String);

impl Operation {
    fn to_string(&self) -> String {
        unimplemented!()
    }
}

struct Changeset {
    head: Head,
    ops: Operation,
    char_bank: String,
}

impl Changeset {
    fn from_reader(reader: &mut dyn Read) -> anyhow::Result<Changeset> {
        let mut iter = reader.bytes().map_while(|item| item.ok());
        Changeset::from_iter(&mut iter)
    }
    fn from_iter(iter: &mut dyn Iterator<Item=u8>) -> anyhow::Result<Changeset> {
        let mut iter = iter.peekable();
        let head = Head::from_iter(&mut iter)?;
        unimplemented!()
    }
    fn follow(&self, next: &Changeset) -> Changeset {
        unimplemented!()
    }
    fn compose(&self, next: &Changeset) -> Changeset {
        unimplemented!()
    }
}
