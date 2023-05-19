use regex::Regex;

pub fn detect(x: Option<&str>, pat: &Regex, negate: bool) -> Option<bool> {
        let x = x?;
        let a = pat.is_match(x);
        negate.then(|| !a).or(Some(a))
        
    }
