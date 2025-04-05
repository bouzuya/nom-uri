use nom::{IResult, Input as _, Offset as _, Parser};

use crate::parser::{pct_encoded, sub_delims, unreserved};

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

/// pchar         = unreserved / pct-encoded / sub-delims / ":" / "@"
///
/// <https://datatracker.ietf.org/doc/html/rfc3986#section-3.3>
pub fn pchar(i: Span) -> IResult<Span, Token> {
    let start = i;
    let (i, _) = nom::branch::alt((
        unreserved.map(|t| Token { span: t.span() }),
        pct_encoded.map(|t| Token { span: t.span() }),
        sub_delims.map(|t| Token { span: t.span() }),
        nom::character::char(':').map(|_| Token {
            span: start.take(1),
        }),
        nom::character::char('@').map(|_| Token {
            span: start.take(1),
        }),
    ))
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
    fn test_pchar() {
        ok(pchar, "arest", ("rest", "a"));
        ok(pchar, "%20rest", ("rest", "%20"));
        ok(pchar, "!rest", ("rest", "!"));
        ok(pchar, ":rest", ("rest", ":"));
        ok(pchar, "@rest", ("rest", "@"));

        err(pchar, "%G1rest");
        err(pchar, "%1Grest");
        err(pchar, "%rest");
        err(pchar, "");
        err(pchar, "#rest");
        err(pchar, "/rest");
    }
}
