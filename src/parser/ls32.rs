use nom::{IResult, Input as _, Offset as _, Parser};

use super::{HasSpan, Span};
use crate::parser::h16;

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub span: Span<'a>,
}

impl<'a> HasSpan<'a> for Token<'a> {
    fn span(&self) -> Span<'a> {
        self.span
    }
}

/// ls32        = ( h16 ":" h16 ) / IPv4address
///             ; least-significant 32 bits of address
///
/// <https://datatracker.ietf.org/doc/html/rfc3986#section-3.2.2>
pub fn ls32(i: Span) -> IResult<Span, Token> {
    let start = i;
    let (i, _) = (
        h16.map(|_| ()),
        nom::character::complete::char(':').map(|_| ()),
        h16.map(|_| ()),
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
    fn test_ls32() {
        ok(ls32, "ABCD:1234", ("", "ABCD:1234"));
        ok(ls32, "0123:4567", ("", "0123:4567"));
        ok(ls32, "89AB:CDEF", ("", "89AB:CDEF"));
        err(ls32, "ffff:eeee");

        ok(ls32, "ABCD:1234/path", ("/path", "ABCD:1234"));
        ok(ls32, "0123:4567?query", ("?query", "0123:4567"));
        ok(ls32, "89AB:CDEF#fragment", ("#fragment", "89AB:CDEF"));

        err(ls32, "");
        err(ls32, "ABCD");
        err(ls32, "ABCD:");
        err(ls32, ":1234");
        err(ls32, "ABC:1234");
        err(ls32, "ABCD:123");
        err(ls32, "ABCG:1234");
        err(ls32, "ABCD:123G");
        err(ls32, "ABCDE:1234");
        ok(ls32, "ABCD:12345", ("5", "ABCD:1234"));
        err(ls32, "ABCD;1234");
        err(ls32, "ABCD1234");
    }
}
