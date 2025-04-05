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

/// segment-nz-nc = 1*( unreserved / pct-encoded / sub-delims / "@" )
///               ; non-zero-length segment without any colon ":"
///
/// <https://datatracker.ietf.org/doc/html/rfc3986#section-3.3>
pub fn segment_nz_nc(i: Span) -> IResult<Span, Token> {
    let start = i;
    let (i, _) = nom::multi::many1(nom::branch::alt((
        unreserved.map(|t| Token { span: t.span() }),
        pct_encoded.map(|t| Token { span: t.span() }),
        sub_delims.map(|t| Token { span: t.span() }),
        nom::character::complete::char('@').map(|_| Token {
            span: start.take(1),
        }),
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
    fn test_segment_nz_nc() {
        ok(segment_nz_nc, "abc", ("", "abc"));
        ok(segment_nz_nc, "123", ("", "123"));
        ok(segment_nz_nc, "a-b.c_d~", ("", "a-b.c_d~"));
        ok(segment_nz_nc, "%2f", ("", "%2f"));
        ok(segment_nz_nc, "!$&'()*+,;=", ("", "!$&'()*+,;="));
        ok(segment_nz_nc, "@", ("", "@"));
        ok(segment_nz_nc, "abc/def", ("/def", "abc"));
        ok(segment_nz_nc, "abc?query", ("?query", "abc"));
        ok(segment_nz_nc, "abc#fragment", ("#fragment", "abc"));

        err(segment_nz_nc, "");
        err(segment_nz_nc, "%g0");
        err(segment_nz_nc, ":");
    }
}
