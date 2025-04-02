use nom::{IResult, Parser};

/// sub-delims    = "!" / "$" / "&" / "'" / "(" / ")"
///               / "*" / "+" / "," / ";" / "="
///
/// <https://datatracker.ietf.org/doc/html/rfc3986#section-2.2>
pub fn sub_delims(input: &str) -> IResult<&str, char> {
    nom::character::satisfy(|c| {
        matches!(
            c,
            '!' | '$' | '&' | '\'' | '(' | ')' | '*' | '+' | ',' | ';' | '='
        )
    })
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub_delims() {
        assert_eq!(sub_delims("!rest"), Ok(("rest", '!')));
        assert_eq!(sub_delims("$rest"), Ok(("rest", '$')));
        assert_eq!(sub_delims("&rest"), Ok(("rest", '&')));
        assert_eq!(sub_delims("'rest"), Ok(("rest", '\'')));
        assert_eq!(sub_delims("(rest"), Ok(("rest", '(')));
        assert_eq!(sub_delims(")rest"), Ok(("rest", ')')));
        assert_eq!(sub_delims("*rest"), Ok(("rest", '*')));
        assert_eq!(sub_delims("+rest"), Ok(("rest", '+')));
        assert_eq!(sub_delims(",rest"), Ok(("rest", ',')));
        assert_eq!(sub_delims(";rest"), Ok(("rest", ';')));
        assert_eq!(sub_delims("=rest"), Ok(("rest", '=')));
        assert!(sub_delims("arest").is_err());
        assert!(sub_delims("").is_err());
    }
}
