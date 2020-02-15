use std::io;
use regex::Regex;

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
