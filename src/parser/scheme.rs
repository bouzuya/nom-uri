use nom::{IResult, Parser};

/// scheme      = ALPHA *( ALPHA / DIGIT / "+" / "-" / "." )
///
/// <https://datatracker.ietf.org/doc/html/rfc3986#section-3.1>
pub fn scheme(input: &str) -> IResult<&str, String> {
    let (input, first) = nom::character::satisfy(|c| c.is_ascii_alphabetic()).parse(input)?;
    let (input, rest) = nom::multi::many0(nom::character::satisfy(|c| {
        c.is_ascii_alphanumeric() || c == '+' || c == '-' || c == '.'
    }))
    .parse(input)?;
    Ok((
        input,
        std::iter::once(first).chain(rest).collect::<String>(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scheme() {
        assert_eq!(scheme("http://"), Ok(("://", "http".to_owned())));
        assert_eq!(scheme("https://"), Ok(("://", "https".to_owned())));
        assert_eq!(scheme("ftp://"), Ok(("://", "ftp".to_owned())));
        assert_eq!(scheme("a1+.-://"), Ok(("://", "a1+.-".to_owned())));
        assert!(scheme("1http://").is_err());
        assert!(scheme("+http://").is_err());
        assert!(scheme("").is_err());
        assert_eq!(scheme("a://"), Ok(("://", "a".to_owned())));
    }
}
