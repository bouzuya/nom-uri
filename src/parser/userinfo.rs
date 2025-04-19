use nom::{IResult, Input as _, Offset as _, Parser};

use super::{HasSpan, Span};
use crate::parser::{pct_encoded, sub_delims, unreserved};

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub span: Span<'a>,
}

impl<'a> HasSpan<'a> for Token<'a> {
    fn span(&self) -> Span<'a> {
        self.span
    }
}

/// userinfo    = *( unreserved / pct-encoded / sub-delims / ":" )
///
/// <https://datatracker.ietf.org/doc/html/rfc3986#section-3.2.1>
pub fn userinfo(i: Span) -> IResult<Span, Token> {
    let start = i;
    let (i, _) = nom::multi::many0(nom::branch::alt((
        unreserved.map(|_| ()),
        pct_encoded.map(|_| ()),
        sub_delims.map(|_| ()),
        nom::character::complete::char(':').map(|_| ()),
    )))
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
    fn test_userinfo() {
        ok(userinfo, "user:password", ("", "user:password"));
        ok(userinfo, "username", ("", "username"));
        ok(userinfo, "user:pass:word", ("", "user:pass:word"));
        ok(userinfo, "", ("", ""));

        ok(
            userinfo,
            "abcdefghijklmnopqrstuvwxyz",
            ("", "abcdefghijklmnopqrstuvwxyz"),
        );
        ok(
            userinfo,
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
            ("", "ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
        );
        ok(userinfo, "0123456789", ("", "0123456789"));
        ok(userinfo, "-._~", ("", "-._~"));

        ok(userinfo, "%20%21", ("", "%20%21"));
        ok(userinfo, "user%40example", ("", "user%40example"));
        ok(userinfo, "user%3Apass", ("", "user%3Apass"));

        ok(userinfo, "!$&'()*+,;=", ("", "!$&'()*+,;="));

        ok(userinfo, ":", ("", ":"));

        ok(userinfo, "user:pass@host.com", ("@host.com", "user:pass"));
        ok(userinfo, "user:pass/path", ("/path", "user:pass"));
        ok(userinfo, "user:pass?query", ("?query", "user:pass"));
        ok(userinfo, "user:pass#fragment", ("#fragment", "user:pass"));

        ok(userinfo, "john.doe", ("", "john.doe"));
        ok(userinfo, "user-name.123", ("", "user-name.123"));
        ok(userinfo, "user+name", ("", "user+name"));
        ok(
            userinfo,
            "first.last%40domain.com",
            ("", "first.last%40domain.com"),
        );

        ok(userinfo, "192.168.0.1", ("", "192.168.0.1"));
        ok(userinfo, "127.0.0.1", ("", "127.0.0.1"));

        ok(
            userinfo,
            "a1-._~%20!$&'()*+,;=:password",
            ("", "a1-._~%20!$&'()*+,;=:password"),
        );

        ok(userinfo, "user@domain.com", ("@domain.com", "user"));
        ok(userinfo, "user[info]", ("[info]", "user"));
        ok(userinfo, "user]info", ("]info", "user"));

        ok(userinfo, "ABCD:1234", ("", "ABCD:1234"));
        ok(userinfo, "0123:4567", ("", "0123:4567"));
        ok(userinfo, "89AB:CDEF", ("", "89AB:CDEF"));
        ok(userinfo, "ffff:eeee", ("", "ffff:eeee"));
        ok(userinfo, "abc:def", ("", "abc:def"));
        ok(userinfo, "1:2", ("", "1:2"));

        ok(userinfo, "ABCD:1234/path", ("/path", "ABCD:1234"));
        ok(userinfo, "0123:4567?query", ("?query", "0123:4567"));
    }
}
