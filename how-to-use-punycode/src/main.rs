use base36;
use regex::Regex;

fn main() {}

fn ascii_to_alphanumeric(chars: &str) -> Option<String> {
    let re = Regex::new(r"^[\x00-\x7F]+$").unwrap();
    if !re.is_match(&chars) {
        return None;
    }
    let re = Regex::new(r"[^a-z0-9]").unwrap();
    let str = re.replace_all(&chars, "-");
    let str = str.trim_matches('-');
    if str.is_empty() {
        return None;
    }
    Some(str.to_string())
}

fn to_base36(chars: &str) -> String {
    base36::encode(chars.as_bytes()).to_string()
}

// fn to_punycode(chars: &str) -> Option<String> {
//     // replace ascii but not in [a-z0-9]
// }

#[cfg(test)]
mod test {
    use punycode;

    use super::*;

    // use idna::punycode;

    #[test]
    fn test() {
        println!("{:?}", punycode::encode("issues/3"));
        println!("{:?}", punycode::encode("にほんご"));
    }

    #[test]
    fn test2() {
        assert_eq!(ascii_to_alphanumeric("にほんご"), None);
        assert_eq!(ascii_to_alphanumeric("-"), None);
        assert_eq!(ascii_to_alphanumeric("---"), None);
        assert_eq!(
            ascii_to_alphanumeric("issues/3").unwrap().as_str(),
            "issues-3"
        );
        assert_eq!(
            ascii_to_alphanumeric("new_feature").unwrap().as_str(),
            "new-feature"
        );
        assert_eq!(ascii_to_alphanumeric("-a-").unwrap().as_str(), "a");
    }

    #[test]
    fn test3() {
        assert_eq!(to_base36("にほんご"), "38j2b6b6e");
        assert_eq!(to_base36("issues/3"), "6tpy4ibqqswgwndgadw");
    }
}
