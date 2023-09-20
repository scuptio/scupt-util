use std::string::String;

pub fn escape_string(s: &String) -> String {
    if s.len() < 2 {
        return s.clone();
    }
    let byte = s.as_bytes();
    let mut ret = String::from("");

    let mut pos = 0;
    while pos < byte.len() {
        let b1 = byte[pos];
        if b1 == b'\\' {
            if pos + 1 < byte.len() {
                let b2 = byte[pos + 1];
                match b2 {
                    b'a' => ret.push('\x07'),
                    b'b' => ret.push('\x08'),
                    b't' => ret.push('\t'),
                    b'n' => ret.push('\n'),
                    b'v' => ret.push('\x0b'),
                    b'f' => ret.push('\x0c'),
                    b'r' => ret.push('\r'),
                    b' ' => ret.push(' '),
                    b'\\' => ret.push('\\'),
                    b'"' => ret.push('"'),
                    _ => {
                        ret.push(b1 as char);
                        ret.push(b2 as char)
                    }
                }
                pos += 2;
            } else {
                ret.push(b1 as char);
                pos += 1;
            }
        } else {
            ret.push(b1 as char);
            pos += 1;
        }
    }

    return ret;
}