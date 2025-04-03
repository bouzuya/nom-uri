use nom::{IResult, Parser};

use crate::parser::{pct_encoded, sub_delims, unreserved};

/// pchar         = unreserved / pct-encoded / sub-delims / ":" / "@"
///
/// <https://datatracker.ietf.org/doc/html/rfc3986#section-3.3>
pub fn pchar(input: &str) -> IResult<&str, String> {
    nom::branch::alt((
        unreserved.map(|c| c.to_string()),
        pct_encoded,
        sub_delims.map(|c| c.to_string()),
        nom::character::char(':').map(|c| c.to_string()),
        nom::character::char('@').map(|c| c.to_string()),
    ))
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pchar() {
        assert_eq!(pchar("arest"), Ok(("rest", "a".to_string()))); // unreserved
        assert_eq!(pchar("%20rest"), Ok(("rest", "%20".to_string()))); // pct-encoded
        assert_eq!(pchar("!rest"), Ok(("rest", "!".to_string()))); // sub-delims
        assert_eq!(pchar(":rest"), Ok(("rest", ":".to_string()))); // ":"
        assert_eq!(pchar("@rest"), Ok(("rest", "@".to_string()))); // "@"

        assert!(pchar("%G1rest").is_err());
        assert!(pchar("%1Grest").is_err());
        assert!(pchar("%rest").is_err());
        assert!(pchar("").is_err());
        assert!(pchar("#rest").is_err());
        assert!(pchar("/rest").is_err());
    }
}
