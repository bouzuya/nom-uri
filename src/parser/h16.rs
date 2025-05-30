use nom::{IResult, Input as _, Offset as _, Parser};

use super::{HasSpan, Span};
use crate::parser::hexdig;

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub span: Span<'a>,
}

impl<'a> HasSpan<'a> for Token<'a> {
    fn span(&self) -> Span<'a> {
        self.span
    }
}

/// h16         = 1*4HEXDIG
///             ; 16 bits of address represented in hexadecimal
///
/// <https://datatracker.ietf.org/doc/html/rfc3986#section-3.2.2>
pub fn h16(i: Span) -> IResult<Span, Token> {
    let start = i;
    let (i, _) = nom::multi::many_m_n(1, 4, hexdig).parse(i)?;
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
    fn test_h16() {
        ok(h16, "0123", ("", "0123"));
        ok(h16, "4567", ("", "4567"));
        ok(h16, "89AB", ("", "89AB"));
        ok(h16, "CDEF", ("", "CDEF"));

        ok(h16, "A", ("", "A"));
        ok(h16, "1F", ("", "1F"));

        ok(h16, "ABCDEF", ("EF", "ABCD"));
        ok(h16, "12345678", ("5678", "1234"));

        ok(h16, "ffff", ("", "ffff"));
        ok(h16, "12aF", ("", "12aF"));

        ok(h16, "ABCD:", (":", "ABCD"));
        ok(h16, "123Frest", ("rest", "123F"));
        ok(h16, "aBcD/path", ("/path", "aBcD"));
        ok(h16, "0123?query", ("?query", "0123"));

        err(h16, "");
        err(h16, "G123");
        ok(h16, "12G4", ("G4", "12"));
        err(h16, "xyz1");
        err(h16, "!@#$");
    }
}
