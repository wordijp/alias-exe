use std::{io, ops};
use std::io::{Error, ErrorKind};
use regex::Regex;

use crate::lib::term;

pub fn replace_all_func(
    re: &Regex,
    text: &str,
    frep: impl Fn(&regex::Captures) -> io::Result<String>
)
    -> io::Result<String>
{
    let mut s = String::new();

    let mut idx = 0;
    for m in re.captures_iter(text) {
        let elm = m.get(0).unwrap();
        let start = elm.start();
        if idx < start {
            s.push_str(&text[idx..start]);
        }
        let rep = frep(&m);
        if let Err(err) = rep {
            return Err(Error::new(ErrorKind::InvalidData, format!("{}\n{}{}{}", err, &text[..start], term::ewrite(elm.as_str())?, &text[elm.end()..])));
        }

        s.push_str(&rep.unwrap());

        idx = elm.end();
    }
    if idx < text.len() {
        s.push_str(&text[idx..]);
    }

    Ok(s)
}

pub fn replace_all_func_nested(
    re: &Regex,
    text: &str,
    frep: impl Fn(&regex::Captures) -> io::Result<String>
)
    -> io::Result<String>
{
    let mut s = text.to_string();

    while let Some(rng) = nested_match(re, &s) {
        let s1 = &s[..rng.start];
        let s2 = &s[rng.start..rng.end];
        let s3 = &s[rng.end..];

        let rep = frep(&re.captures(s2).unwrap());
        if let Err(err) = rep {
            return Err(Error::new(ErrorKind::InvalidData, format!("{}\n{}{}{}", err, s1, term::ewrite(s2)?, s3)));
        }

        s = format!("{}{}{}", s1, &rep.unwrap(), s3);
    }
    Ok(s)
}

fn nested_match(re: &Regex, text: &str) -> Option<ops::Range<usize>>
{
    let m = re.find(text);
    if m.is_none() {
        return None;
    }

    let mut rng = m.unwrap().range();

    while let Some(m) = re.find_at(text, rng.start + 1) {
        if m.end() != rng.end {
            break;
        }
        rng = m.range();
    }
    Some(rng)
}

pub fn partition_re<'a>(re: &Regex, text: &'a str) -> Option<(&'a str, &'a str, &'a str)> {
    if let Some(caps) = re.captures(text) {
        if let Some(elm) = caps.get(0) {
            return Some((
                &text[..elm.start()],
                &text[elm.start()..elm.end()],
                &text[elm.end()..]
            ));
        }
    }
    None
}
