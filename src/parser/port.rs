use nom::{IResult, Input as _, Offset as _, Parser};

use super::{HasSpan, Span};

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub span: Span<'a>,
}

impl<'a> HasSpan<'a> for Token<'a> {
    fn span(&self) -> Span<'a> {
        self.span
    }
}

/// port        = *DIGIT
///
/// <https://datatracker.ietf.org/doc/html/rfc3986#section-3.2.3>
pub fn port(i: Span) -> IResult<Span, Token> {
    let start = i;
    let (i, _) =
        nom::multi::many0(nom::character::complete::satisfy(|c| c.is_ascii_digit())).parse(i)?;
    Ok((
        i,
        Token {
            span: start.take(start.offset(&i)),
        },
    ))
}

#[cfg(test)]
mod tests {
    use crate::parser::tests::ok;

    use super::*;

    #[test]
    fn test_port() {
        ok(port, "80", ("", "80"));
        ok(port, "8080", ("", "8080"));
        ok(port, "443", ("", "443"));
        ok(port, "65535", ("", "65535"));
        ok(port, "0", ("", "0"));

        ok(port, "", ("", ""));

        ok(port, "80/path", ("/path", "80"));
        ok(port, "8080?query", ("?query", "8080"));
        ok(port, "443#fragment", ("#fragment", "443"));

        ok(port, "1a", ("a", "1"));
    }
}
