use nom::{IResult, Input as _, Offset as _, Parser};

use super::{HasSpan, Span};
use crate::parser::{ip_literal, ipv4address, reg_name};

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub span: Span<'a>,
}

impl<'a> HasSpan<'a> for Token<'a> {
    fn span(&self) -> Span<'a> {
        self.span
    }
}

/// host        = IP-literal / IPv4address / reg-name
///
/// <https://datatracker.ietf.org/doc/html/rfc3986#section-3.2.2>
pub fn host(i: Span) -> IResult<Span, Token> {
    let start = i;
    let (i, _) = nom::branch::alt((
        ip_literal.map(|_| ()),
        ipv4address.map(|_| ()),
        reg_name.map(|_| ()),
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
    use crate::parser::tests::ok;

    use super::*;

    #[test]
    fn test_host() {
        ok(host, "192.168.0.1", ("", "192.168.0.1"));
        ok(host, "127.0.0.1", ("", "127.0.0.1"));
        ok(host, "255.255.255.255", ("", "255.255.255.255"));
        ok(host, "0.0.0.0", ("", "0.0.0.0"));

        ok(host, "192.168.0.1:80", (":80", "192.168.0.1"));
        ok(host, "192.168.0.1/path", ("/path", "192.168.0.1"));

        ok(
            host,
            "[1111:2222:3333:4444:5555:6666:7777:8888]",
            ("", "[1111:2222:3333:4444:5555:6666:7777:8888]"),
        );
        ok(
            host,
            "[1111:2222:3333:4444:5555:6666:127.0.0.1]",
            ("", "[1111:2222:3333:4444:5555:6666:127.0.0.1]"),
        );
        ok(host, "[::1]", ("", "[::1]"));
        ok(host, "[2001:db8::1]", ("", "[2001:db8::1]"));
        ok(host, "[::ffff:192.0.2.128]", ("", "[::ffff:192.0.2.128]"));

        ok(host, "[2001:db8::1]:8080", (":8080", "[2001:db8::1]"));
        ok(host, "[2001:db8::1]/path", ("/path", "[2001:db8::1]"));

        ok(host, "[v1.12345]", ("", "[v1.12345]"));
        ok(
            host,
            "[vF.fe80000000000000022501fffef00003]",
            ("", "[vF.fe80000000000000022501fffef00003]"),
        );
        ok(host, "[vA.!$&'()*+,;=:]", ("", "[vA.!$&'()*+,;=:]"));
        ok(host, "[v1.12345]/path", ("/path", "[v1.12345]"));

        ok(host, "example.com", ("", "example.com"));
        ok(host, "sub.example.com", ("", "sub.example.com"));
        ok(host, "example", ("", "example"));
        ok(host, "localhost", ("", "localhost"));

        ok(host, "example.com:80", (":80", "example.com"));
        ok(host, "example.com/path", ("/path", "example.com"));

        ok(host, "%20:80", (":80", "%20"));
        ok(host, "!$&'()*+,;=:80", (":80", "!$&'()*+,;="));

        ok(host, "", ("", ""));

        ok(host, "example.com#f", ("#f", "example.com"));
        ok(host, "example.com/p", ("/p", "example.com"));
        ok(host, "example.com?q", ("?q", "example.com"));

        // 256.0.0.1 is parsed as reg-name, not IPv4address
        ok(host, "256.0.0.1", ("", "256.0.0.1"));
        ok(host, "1.2.3.4.5", (".5", "1.2.3.4"));
        // 1.2.3 is parsed as reg-name, not IPv4address
        ok(host, "1.2.3", ("", "1.2.3"));
    }
}
