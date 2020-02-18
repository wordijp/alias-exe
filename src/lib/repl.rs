use std::{io, ops};
use regex::Regex;

#[allow(dead_code)]
pub fn replace_all_func(
    re: &Regex,
    text: &str,
    frep: impl Fn(&str) -> io::Result<String>
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
        s.push_str(&frep(&elm.as_str())?);
        
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
        s = format!("{}{}{}", &s[..rng.start], &frep(&re.captures(&s[rng.start..rng.end]).unwrap())?, &s[rng.end..]);
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
