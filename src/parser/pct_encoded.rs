use nom::{IResult, Parser};

/// pct-encoded = "%" HEXDIG HEXDIG
///
/// <https://datatracker.ietf.org/doc/html/rfc3986#section-2.1>
pub fn pct_encoded(input: &str) -> IResult<&str, String> {
    (
        nom::character::char('%'),
        nom::character::satisfy(|c| c.is_ascii_hexdigit()),
        nom::character::satisfy(|c| c.is_ascii_hexdigit()),
    )
        .map(|(_, first, second)| format!("%{}{}", first, second))
        .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pct_encoded() {
        assert_eq!(pct_encoded("%20rest"), Ok(("rest", "%20".to_string())));
        assert_eq!(pct_encoded("%7Erest"), Ok(("rest", "%7E".to_string())));
        assert_eq!(pct_encoded("%41rest"), Ok(("rest", "%41".to_string())));
        assert_eq!(pct_encoded("%5Arest"), Ok(("rest", "%5A".to_string())));

        assert!(pct_encoded("20rest").is_err());
        assert!(pct_encoded("%G1rest").is_err());
        assert!(pct_encoded("%1Grest").is_err());
        assert!(pct_encoded("%rest").is_err());
        assert!(pct_encoded("").is_err());
    }
}
