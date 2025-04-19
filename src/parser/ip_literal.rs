use nom::{IResult, Input as _, Offset as _, Parser};

use super::{HasSpan, Span};
use crate::parser::{ipv6address, ipvfuture};

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub span: Span<'a>,
}

impl<'a> HasSpan<'a> for Token<'a> {
    fn span(&self) -> Span<'a> {
        self.span
    }
}

/// IP-literal = "[" ( IPv6address / IPvFuture  ) "]"
///
/// <https://datatracker.ietf.org/doc/html/rfc3986#section-3.2.2>
pub fn ip_literal(i: Span) -> IResult<Span, Token> {
    let start = i;
    let (i, _) = nom::character::complete::char('[').parse(i)?;
    let (i, _) = nom::branch::alt((ipvfuture.map(|_| ()), ipv6address.map(|_| ()))).parse(i)?;
    let (i, _) = nom::character::complete::char(']').parse(i)?;
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
    fn test_ip_literal() {
        ok(
            ip_literal,
            "[1111:2222:3333:4444:5555:6666:7777:8888]",
            ("", "[1111:2222:3333:4444:5555:6666:7777:8888]"),
        );
        ok(
            ip_literal,
            "[1111:2222:3333:4444:5555:6666:127.0.0.1]",
            ("", "[1111:2222:3333:4444:5555:6666:127.0.0.1]"),
        );

        ok(ip_literal, "[::1]", ("", "[::1]"));
        ok(ip_literal, "[2001:db8::1]", ("", "[2001:db8::1]"));
        ok(
            ip_literal,
            "[::ffff:192.0.2.128]",
            ("", "[::ffff:192.0.2.128]"),
        );

        ok(ip_literal, "[v1.12345]", ("", "[v1.12345]"));
        ok(
            ip_literal,
            "[vF.fe80000000000000022501fffef00003]",
            ("", "[vF.fe80000000000000022501fffef00003]"),
        );
        ok(ip_literal, "[vA.!$&'()*+,;=:]", ("", "[vA.!$&'()*+,;=:]"));

        ok(ip_literal, "[2001:db8::1]:8080", (":8080", "[2001:db8::1]"));
        ok(ip_literal, "[v1.12345]/path", ("/path", "[v1.12345]"));

        err(ip_literal, "2001:db8::1");
        err(ip_literal, "[2001:db8::1");
        err(ip_literal, "2001:db8::1]");
        err(ip_literal, "[2001::db8::1]");
        err(ip_literal, "[v1]");
        err(ip_literal, "[v1.]");
        err(ip_literal, "[.1]");
        err(ip_literal, "[]");
    }
}
