use nom::{IResult, Input as _, Offset as _, Parser};

use crate::parser::{fragment, hier_part, query, scheme};

use super::{HasSpan, Span};

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub span: Span<'a>,
}

impl<'a> HasSpan<'a> for Token<'a> {
    fn span(&self) -> Span<'a> {
        self.span
    }
}

/// URI         = scheme ":" hier-part [ "?" query ] [ "#" fragment ]
///
/// <https://datatracker.ietf.org/doc/html/rfc3986#section-3>
pub fn uri(i: Span) -> IResult<Span, Token> {
    let start = i;
    let (i, _) = (
        scheme,
        nom::character::complete::char(':'),
        hier_part,
        nom::combinator::opt((nom::character::complete::char('?'), query)),
        nom::combinator::opt((nom::character::complete::char('#'), fragment)),
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
    fn test_uri() {
        // <https://datatracker.ietf.org/doc/html/rfc3986#section-1.1.2>
        for s in [
            "ftp://ftp.is.co.za/rfc/rfc1808.txt",
            "http://www.ietf.org/rfc/rfc2396.txt",
            // TODO: IP-literal + query
            //   ldap://[2001:db8::7]/c=GB?objectClass?one
            "mailto:John.Doe@example.com",
            "news:comp.infosystems.www.servers.unix",
            "tel:+1-816-555-1212",
            // TODO: port
            // "telnet://192.0.2.16:80/",
            "urn:oasis:names:specification:docbook:dtd:xml:4.1.2",
        ] {
            ok(uri, s, ("", s));
        }

        ok(uri, "http://example.com", ("", "http://example.com"));
        ok(uri, "http://192.168.0.1/", ("", "http://192.168.0.1/"));
        ok(
            uri,
            "http://example.com/path/to?q=v",
            ("", "http://example.com/path/to?q=v"),
        );
        ok(
            uri,
            "http://example.com/path/to#f",
            ("", "http://example.com/path/to#f"),
        );
        ok(
            uri,
            "http://example.com/path/to?q=v#f",
            ("", "http://example.com/path/to?q=v#f"),
        );
        // TODO: port
        // ok(
        //     uri,
        //     "http://example.com:8080/",
        //     ("", "http://example.com:8080/"),
        // );

        // TODO: userinfo
        // ok(
        //     uri,
        //     "http://username:password@example.com/",
        //     ("", "http://username:password@example.com/"),
        // );

        ok(
            uri,
            "file:///path/to/file.txt",
            ("", "file:///path/to/file.txt"),
        );

        ok(
            uri,
            "http://example.com/ trailing text",
            (" trailing text", "http://example.com/"),
        );

        ok(uri, "a:", ("", "a:"));

        err(uri, "://example.com");
        err(uri, "");
        err(uri, "1http://example.com");
        err(uri, "http//example.com");
    }
}
