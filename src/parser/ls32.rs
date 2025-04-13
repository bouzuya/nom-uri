use nom::{IResult, Input as _, Offset as _, Parser};

use super::{HasSpan, Span};
use crate::parser::{h16, ipv4address};

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
    let (i, _) = nom::branch::alt((
        (
            h16.map(|_| ()),
            nom::character::complete::char(':').map(|_| ()),
            h16.map(|_| ()),
        )
            .map(|_| ()),
        ipv4address.map(|_| ()),
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
    fn test_ls32() {
        ok(ls32, "ABCD:1234", ("", "ABCD:1234"));
        ok(ls32, "0123:4567", ("", "0123:4567"));
        ok(ls32, "89AB:CDEF", ("", "89AB:CDEF"));
        ok(ls32, "ffff:eeee", ("", "ffff:eeee"));
        ok(ls32, "abc:def", ("", "abc:def"));
        ok(ls32, "1:2", ("", "1:2"));

        ok(ls32, "ABCD:1234/path", ("/path", "ABCD:1234"));
        ok(ls32, "0123:4567?query", ("?query", "0123:4567"));
        ok(ls32, "89AB:CDEF#fragment", ("#fragment", "89AB:CDEF"));

        ok(ls32, "192.168.0.1", ("", "192.168.0.1"));
        ok(ls32, "127.0.0.1", ("", "127.0.0.1"));
        ok(ls32, "255.255.255.255", ("", "255.255.255.255"));
        ok(ls32, "0.0.0.0", ("", "0.0.0.0"));

        ok(ls32, "192.168.0.1/path", ("/path", "192.168.0.1"));
        ok(ls32, "127.0.0.1?query", ("?query", "127.0.0.1"));
        ok(ls32, "10.0.0.1#fragment", ("#fragment", "10.0.0.1"));

        ok(ls32, "AbCd:EfFf", ("", "AbCd:EfFf"));

        ok(ls32, "ABC:1234", ("", "ABC:1234"));
        ok(ls32, "A:B", ("", "A:B"));
        ok(ls32, "ABCD:123", ("", "ABCD:123"));

        ok(ls32, "ABCD:123G", ("G", "ABCD:123"));
        ok(ls32, "ABCD:12345", ("5", "ABCD:1234"));
        ok(ls32, "192.168.0.1extra", ("extra", "192.168.0.1"));

        err(ls32, "");
        err(ls32, "ABCD");
        err(ls32, "ABCD:");
        err(ls32, ":1234");
        err(ls32, "ABCDE:1234");
        ok(ls32, "ABCD:12345G", ("5G", "ABCD:1234"));
        err(ls32, "ABCG:1234");
        err(ls32, "ABCD;1234");
        err(ls32, "ABCD1234");

        err(ls32, "256.0.0.1");
        err(ls32, "192.168.0");
        err(ls32, "192.168..1");
        err(ls32, "192.168.0.");
        err(ls32, "a.b.c.d");
    }
}
