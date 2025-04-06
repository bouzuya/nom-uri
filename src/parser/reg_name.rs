use nom::{IResult, Input as _, Offset as _, Parser};

use super::{HasSpan, Span};
use crate::parser::{pct_encoded, sub_delims, unreserved};

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub span: Span<'a>,
}

impl<'a> HasSpan<'a> for Token<'a> {
    fn span(&self) -> Span<'a> {
        self.span
    }
}

/// reg-name    = *( unreserved / pct-encoded / sub-delims )
///
/// <https://datatracker.ietf.org/doc/html/rfc3986#section-3.2.2>
pub fn reg_name(i: Span) -> IResult<Span, Token> {
    let start = i;
    let (i, _) = nom::multi::many0(nom::branch::alt((
        unreserved.map(|_| ()),
        pct_encoded.map(|_| ()),
        sub_delims.map(|_| ()),
    )))
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
    fn test_reg_name() {
        ok(reg_name, "", ("", ""));

        // unreserved
        ok(reg_name, "a1-._~", ("", "a1-._~"));
        ok(reg_name, "example.com", ("", "example.com"));

        // pct-encoded
        ok(reg_name, "%20", ("", "%20"));
        ok(reg_name, "%20%21", ("", "%20%21"));

        // sub-delims
        ok(reg_name, "!$&'()*+,;=", ("", "!$&'()*+,;="));

        ok(reg_name, "example.com:80", (":80", "example.com"));
        ok(reg_name, "example.com/path", ("/path", "example.com"));
        ok(reg_name, "example.com%g0", ("%g0", "example.com"));
        ok(reg_name, "example.com?q", ("?q", "example.com"));
        ok(reg_name, "example.com#f", ("#f", "example.com"));
    }
}
