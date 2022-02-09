use std::iter::Peekable;

pub(crate) fn to_num(bytes: Vec<u8>, radix: u32) -> anyhow::Result<u32> {
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

pub(crate) fn to_vec(num: u32, radix: u32) -> Vec<u8> {
    if radix > 36 {
        panic!("to_vec: radix is too high (maximum 36)");
    }
    let mut vec: Vec<u8> = Vec::new();
    let mut num = num;
    loop {
        let u = (num % radix) as u8;
        let c = if u < 10 { b'0' + u } else { b'a' + u - 10 };
        vec.insert(0, c);
        num = num / radix;
        if num == 0 {
            break;
        }
    }
    return vec;
}

pub fn from_iter<I: Iterator<Item=u8>>(iter: &mut Peekable<I>) -> anyhow::Result<u32> {
    let mut buf = vec![];
    loop {
        match iter.peek() {
            None => break,
            Some(b) => match b {
                b'0'..=b'9' | b'a'..=b'z' => {
                    buf.insert(buf.len(), b.clone())
                }
                _ => break,
            }
        }
        iter.next();
    };
    if buf.len() > 0 {
        return to_num(buf.to_vec(), 36)
    }
    return Ok(0)
}
