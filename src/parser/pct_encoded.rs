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

/// pct-encoded = "%" HEXDIG HEXDIG
///
/// <https://datatracker.ietf.org/doc/html/rfc3986#section-2.1>
pub fn pct_encoded(i: Span) -> IResult<Span, Token> {
    let start = i;
    let (i, _) = (
        nom::character::complete::char('%').map(|_| ()),
        nom::character::complete::satisfy(|c| c.is_ascii_hexdigit()).map(|_| ()),
        nom::character::complete::satisfy(|c| c.is_ascii_hexdigit()).map(|_| ()),
    )
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
    fn test_pct_encoded() {
        ok(pct_encoded, "%20rest", ("rest", "%20"));
        ok(pct_encoded, "%7Erest", ("rest", "%7E"));
        ok(pct_encoded, "%41rest", ("rest", "%41"));
        ok(pct_encoded, "%5Arest", ("rest", "%5A"));

        err(pct_encoded, "20rest");
        err(pct_encoded, "%G1rest");
        err(pct_encoded, "%1Grest");
        err(pct_encoded, "%rest");
        err(pct_encoded, "");
    }
}
