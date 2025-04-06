use nom::{IResult, Input as _, Offset as _, Parser};

use super::{HasSpan, Span};
use crate::parser::{ipv4address, reg_name};

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
        // TODO: IP-literal support
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
    use crate::parser::tests::{err, ok};

    use super::*;

    #[test]
    fn test_host() {
        ok(host, "192.168.0.1", ("", "192.168.0.1"));
        ok(host, "127.0.0.1", ("", "127.0.0.1"));
        ok(host, "255.255.255.255", ("", "255.255.255.255"));
        ok(host, "0.0.0.0", ("", "0.0.0.0"));

        ok(host, "192.168.0.1:80", (":80", "192.168.0.1"));
        ok(host, "192.168.0.1/path", ("/path", "192.168.0.1"));

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
