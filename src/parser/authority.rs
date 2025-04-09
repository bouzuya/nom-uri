use nom::{IResult, Input as _, Offset as _, Parser};

use super::{HasSpan, Span};
use crate::parser::{host, port};

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub span: Span<'a>,
}

impl<'a> HasSpan<'a> for Token<'a> {
    fn span(&self) -> Span<'a> {
        self.span
    }
}

/// authority   = [ userinfo "@" ] host [ ":" port ]
///
/// <https://datatracker.ietf.org/doc/html/rfc3986#section-3.2>
pub fn authority(i: Span) -> IResult<Span, Token> {
    let start = i;
    let (i, _) = (
        // TODO: userinfo support
        host.map(|_| ()),
        nom::combinator::opt((nom::character::complete::char(':'), port)),
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
    use crate::parser::tests::ok;

    use super::*;

    #[test]
    fn test_authority() {
        ok(authority, "example.com", ("", "example.com"));
        ok(authority, "localhost", ("", "localhost"));
        ok(authority, "a.b.c", ("", "a.b.c"));
        ok(authority, "192.168.0.1", ("", "192.168.0.1"));

        ok(authority, "example.com:80", ("", "example.com:80"));
        ok(authority, "localhost:8080", ("", "localhost:8080"));
        ok(authority, "192.168.0.1:443", ("", "192.168.0.1:443"));
        ok(authority, "example.com:65535", ("", "example.com:65535"));

        ok(authority, "example.com:", ("", "example.com:"));
        ok(authority, "example.com:0", ("", "example.com:0"));
        ok(
            authority,
            "example.com:123/path",
            ("/path", "example.com:123"),
        );
        ok(
            authority,
            "example.com:456?query",
            ("?query", "example.com:456"),
        );
        ok(
            authority,
            "example.com:789#fragment",
            ("#fragment", "example.com:789"),
        );

        ok(authority, "example.com:8080a", ("a", "example.com:8080"));

        ok(authority, "", ("", ""));

        // TODO: Add tests for userinfo@host once implemented
        // TODO: Add tests for userinfo@host:port once implemented
    }
}
