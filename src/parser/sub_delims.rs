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

/// sub-delims    = "!" / "$" / "&" / "'" / "(" / ")"
///               / "*" / "+" / "," / ";" / "="
///
/// <https://datatracker.ietf.org/doc/html/rfc3986#section-2.2>
pub fn sub_delims(i: Span) -> IResult<Span, Token> {
    let start = i;
    let (i, _) = nom::character::satisfy(|c| {
        matches!(
            c,
            '!' | '$' | '&' | '\'' | '(' | ')' | '*' | '+' | ',' | ';' | '='
        )
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
    fn test_sub_delims() {
        ok(sub_delims, "!rest", ("rest", "!"));
        ok(sub_delims, "$rest", ("rest", "$"));
        ok(sub_delims, "&rest", ("rest", "&"));
        ok(sub_delims, "'rest", ("rest", "'"));
        ok(sub_delims, "(rest", ("rest", "("));
        ok(sub_delims, ")rest", ("rest", ")"));
        ok(sub_delims, "*rest", ("rest", "*"));
        ok(sub_delims, "+rest", ("rest", "+"));
        ok(sub_delims, ",rest", ("rest", ","));
        ok(sub_delims, ";rest", ("rest", ";"));
        ok(sub_delims, "=rest", ("rest", "="));

        err(sub_delims, "arest");
        err(sub_delims, "");
    }
}
