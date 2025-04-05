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

/// unreserved  = ALPHA / DIGIT / "-" / "." / "_" / "~"
///
/// <https://datatracker.ietf.org/doc/html/rfc3986#section-2.3>
pub fn unreserved(i: Span) -> IResult<Span, Token> {
    let start = i;
    let (i, _) = nom::character::complete::satisfy(|c| {
        c.is_ascii_alphanumeric() || c == '-' || c == '.' || c == '_' || c == '~'
    })
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
    fn test_unreserved() {
        ok(unreserved, "arest", ("rest", "a"));
        ok(unreserved, "Zrest", ("rest", "Z"));
        ok(unreserved, "9rest", ("rest", "9"));
        ok(unreserved, "-rest", ("rest", "-"));
        ok(unreserved, ".rest", ("rest", "."));
        ok(unreserved, "_rest", ("rest", "_"));
        ok(unreserved, "~rest", ("rest", "~"));

        err(unreserved, "%20rest");
        err(unreserved, "!rest");
        err(unreserved, "");
    }
}
