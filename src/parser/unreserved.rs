use nom::{IResult, Parser};

/// unreserved  = ALPHA / DIGIT / "-" / "." / "_" / "~"
///
/// <https://datatracker.ietf.org/doc/html/rfc3986#section-2.3>
pub fn unreserved(input: &str) -> IResult<&str, char> {
    nom::character::satisfy(|c| {
        c.is_ascii_alphanumeric() || c == '-' || c == '.' || c == '_' || c == '~'
    })
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unreserved() {
        assert_eq!(unreserved("arest"), Ok(("rest", 'a')));
        assert_eq!(unreserved("Zrest"), Ok(("rest", 'Z')));
        assert_eq!(unreserved("9rest"), Ok(("rest", '9')));
        assert_eq!(unreserved("-rest"), Ok(("rest", '-')));
        assert_eq!(unreserved(".rest"), Ok(("rest", '.')));
        assert_eq!(unreserved("_rest"), Ok(("rest", '_')));
        assert_eq!(unreserved("~rest"), Ok(("rest", '~')));

        assert!(unreserved("%20rest").is_err());
        assert!(unreserved("!rest").is_err());
        assert!(unreserved("").is_err());
    }
}
