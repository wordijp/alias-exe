use encoding_rs::SHIFT_JIS;

pub fn to_utf8_string(v: &Vec<u8>) -> String {
    let decoded = SHIFT_JIS.decode(v);
    if !decoded.2 {
        return decoded.0.to_string();
    }

    String::from_utf8(v.to_owned()).unwrap()
}
