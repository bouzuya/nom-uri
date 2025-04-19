use nom::{IResult, Input as _, Offset as _, Parser};

use super::{HasSpan, Span};
use crate::parser::{host, port, userinfo};

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
    let (i, _) = nom::branch::alt((
        (
            userinfo.map(|_| ()),
            nom::character::complete::char('@').map(|_| ()),
            host.map(|_| ()),
            nom::combinator::opt((nom::character::complete::char(':'), port)),
        )
            .map(|_| ()),
        (
            host.map(|_| ()),
            nom::combinator::opt((nom::character::complete::char(':'), port)),
        )
            .map(|_| ()),
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

        ok(authority, "user@example.com", ("", "user@example.com"));
        ok(authority, "username@localhost", ("", "username@localhost"));
        ok(
            authority,
            "john.doe@192.168.0.1",
            ("", "john.doe@192.168.0.1"),
        );
        ok(
            authority,
            "user-name@domain.com",
            ("", "user-name@domain.com"),
        );
        ok(
            authority,
            "user%20name@domain.com",
            ("", "user%20name@domain.com"),
        );
        ok(authority, "abc123@a.b.c", ("", "abc123@a.b.c"));

        ok(
            authority,
            "user.name@example.com",
            ("", "user.name@example.com"),
        );
        ok(
            authority,
            "user-name@example.com",
            ("", "user-name@example.com"),
        );
        ok(
            authority,
            "user+name@example.com",
            ("", "user+name@example.com"),
        );
        ok(
            authority,
            "user%21@example.com",
            ("", "user%21@example.com"),
        );

        ok(
            authority,
            "user:pass@example.com",
            ("", "user:pass@example.com"),
        );
        ok(
            authority,
            "user:p@ss@example.com",
            ("@example.com", "user:p@ss"),
        );
        ok(
            authority,
            "user:pass:word@host",
            ("", "user:pass:word@host"),
        );

        // TODO: userinfo@host:port

        ok(
            authority,
            "user@example.com/path",
            ("/path", "user@example.com"),
        );
        ok(
            authority,
            "user:pass@example.com:80/path",
            ("/path", "user:pass@example.com:80"),
        );
        ok(
            authority,
            "user@example.com?query",
            ("?query", "user@example.com"),
        );
        ok(
            authority,
            "user@example.com:8080#fragment",
            ("#fragment", "user@example.com:8080"),
        );
    }
}
