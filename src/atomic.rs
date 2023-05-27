use regex::Regex;
use std::borrow::Cow;

pub fn detect(x: Option<&str>, pat: &Regex, negate: bool) -> Option<bool> {
    let x = x?;
    let a = pat.is_match(x);
    negate.then(|| !a).or(Some(a))
}

pub fn to_upper<'a>(x: Option<&'a str>, seperator: &str) -> Option<Cow<'a, str>> {
    let x = x?;
    let a = x.to_string();
    let rs = a
        .split(seperator)
        .map(|x| {
            let mut k = x.to_string();
            let first_letter = k.remove(0);
            let first_letter = first_letter.to_uppercase().to_string();
            format!("{}{}", first_letter, k)
        })
        .collect::<Vec<String>>()
        .join(seperator.to_string().as_str());
    Some(Cow::Owned(rs))
}
