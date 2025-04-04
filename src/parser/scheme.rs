use nom::{Input, Offset};

use super::{IResult, Parser, Span};

#[derive(Debug, PartialEq)]
struct Token<'a> {
    pub span: Span<'a>,
}

/// scheme      = ALPHA *( ALPHA / DIGIT / "+" / "-" / "." )
///
/// <https://datatracker.ietf.org/doc/html/rfc3986#section-3.1>
pub fn scheme(i: Span) -> IResult<Span, Token> {
    let start = i;
    let (i, _) = nom::character::satisfy(|c| c.is_ascii_alphabetic()).parse(i)?;
    let (i, _) = nom::multi::many0(nom::character::satisfy(|c| {
        c.is_ascii_alphanumeric() || c == '+' || c == '-' || c == '.'
    }))
    .parse(i)?;
    Ok((
        i,
        Token {
            span: start.take(start.offset(&i)),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scheme() -> Result<(), ()> {
        fn err(s: &str) {
            use nom_locate::LocatedSpan;

            let f = scheme;
            let s = LocatedSpan::new(s);
            assert!(f(s).is_err());
        }
        fn ok(s: &str, (i, o): (&str, &str)) {
            use nom::Input;
            use nom_locate::LocatedSpan;

            let f = scheme;
            let s = LocatedSpan::new(s);
            assert_eq!(
                f(s).map(|(i, o)| (i, o.span)).expect("f"),
                (
                    s.take_from(s.rfind(i).expect("i")),
                    s.take_from(s.find(o).expect("o")).take(o.len())
                )
            );
        }

        ok("http://", ("://", "http"));
        ok("ftp://", ("://", "ftp"));
        ok("a1+.-://", ("://", "a1+.-"));
        ok("a://", ("://", "a"));
        err("1http://");
        err("+http://");
        err("");

        Ok(())
    }
}
