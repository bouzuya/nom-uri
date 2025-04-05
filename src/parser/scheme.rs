use nom::{IResult, Input, Offset, Parser};

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
    use crate::parser::tests::{err, ok};

    use super::*;

    #[test]
    fn test_scheme() {
        ok(scheme, "http://", ("://", "http"));
        ok(scheme, "ftp://", ("://", "ftp"));
        ok(scheme, "a1+.-://", ("://", "a1+.-"));
        ok(scheme, "a://", ("://", "a"));

        err(scheme, "1http://");
        err(scheme, "+http://");
        err(scheme, "");
    }
}
