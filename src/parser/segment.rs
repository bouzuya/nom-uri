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

/// segment       = *pchar
///
/// <https://datatracker.ietf.org/doc/html/rfc3986#section-3.3>
pub fn segment(i: Span) -> IResult<Span, Token> {
    let start = i;
    let (i, _) = nom::multi::many0(pchar).parse(i)?;
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
    fn test_segment() {
        ok(segment, "", ("", ""));
        ok(segment, "/", ("/", ""));
        ok(segment, "abc", ("", "abc"));
        ok(segment, "123", ("", "123"));
        ok(segment, "a-b.c_d~", ("", "a-b.c_d~"));
        ok(segment, "%2F", ("", "%2F"));
        ok(segment, "!$&'()*+,;=", ("", "!$&'()*+,;="));
        ok(segment, "@", ("", "@"));
        ok(segment, ":", ("", ":"));
        ok(segment, "abc/def", ("/def", "abc"));
        ok(segment, "abc?query", ("?query", "abc"));
        ok(segment, "abc#fragment", ("#fragment", "abc"));
        ok(segment, "%g0", ("%g0", ""));
    }
}
