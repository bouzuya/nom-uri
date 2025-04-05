use nom::{IResult, Input as _, Offset as _, Parser};

use crate::parser::pchar;

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

/// segment-nz    = 1*pchar
///
/// <https://datatracker.ietf.org/doc/html/rfc3986#section-3.3>
pub fn segment_nz(i: Span) -> IResult<Span, Token> {
    let start = i;
    let (i, _) = nom::multi::many1(pchar).parse(i)?;
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
    fn test_segment_nz() {
        ok(segment_nz, "abc", ("", "abc"));
        ok(segment_nz, "123", ("", "123"));
        ok(segment_nz, "a-b.c_d~", ("", "a-b.c_d~"));
        ok(segment_nz, "%2f", ("", "%2f"));
        ok(segment_nz, "!$&'()*+,;=", ("", "!$&'()*+,;="));
        ok(segment_nz, "@", ("", "@"));
        ok(segment_nz, ":", ("", ":"));
        ok(segment_nz, "abc/def", ("/def", "abc"));
        ok(segment_nz, "abc?query", ("?query", "abc"));
        ok(segment_nz, "abc#fragment", ("#fragment", "abc"));

        err(segment_nz, "");
        err(segment_nz, "%g0"); // Invalid percent encoding
    }
}
