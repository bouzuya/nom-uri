use nom::{IResult, Input as _, Offset as _, Parser};

use super::{HasSpan, Span};
use crate::parser::host;

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
        host.map(|_| ())
        // TODO: port support
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

        ok(authority, "", ("", ""));

        ok(authority, "<invalid>", ("<invalid>", ""));

        // TODO: Add tests for userinfo@host once implemented
        // TODO: Add tests for host:port once implemented
        // TODO: Add tests for userinfo@host:port once implemented
    }
}
