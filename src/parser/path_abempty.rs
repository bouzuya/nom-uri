use nom::{IResult, Input as _, Offset as _, Parser};

use crate::parser::segment;

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

/// path-abempty  = *( "/" segment )
///
/// <https://datatracker.ietf.org/doc/html/rfc3986#section-3.3>
pub fn path_abempty(i: Span) -> IResult<Span, Token> {
    let start = i;
    let (i, _) = nom::multi::many0((nom::character::complete::char('/'), segment)).parse(i)?;
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
    fn test_path_abempty() {
        ok(path_abempty, "", ("", ""));

        ok(path_abempty, "a", ("a", ""));
        ok(path_abempty, "1", ("1", ""));
        ok(path_abempty, "-._~", ("-._~", ""));
        ok(path_abempty, "%20", ("%20", ""));
        ok(path_abempty, "!$&'()*+,;=", ("!$&'()*+,;=", ""));
        ok(path_abempty, ":", (":", ""));
        ok(path_abempty, "@", ("@", ""));

        ok(path_abempty, "/a", ("", "/a"));
        ok(path_abempty, "/1", ("", "/1"));
        ok(path_abempty, "/-._~", ("", "/-._~"));
        ok(path_abempty, "/%20", ("", "/%20"));
        ok(path_abempty, "/!$&'()*+,;=", ("", "/!$&'()*+,;="));
        ok(path_abempty, "/:", ("", "/:"));
        ok(path_abempty, "/@", ("", "/@"));

        ok(path_abempty, "/a/a", ("", "/a/a"));
        ok(path_abempty, "/1/1", ("", "/1/1"));
        ok(path_abempty, "/-._~/-._~", ("", "/-._~/-._~"));
        ok(path_abempty, "/%20/%20", ("", "/%20/%20"));
        ok(
            path_abempty,
            "/!$&'()*+,;=/!$&'()*+,;=",
            ("", "/!$&'()*+,;=/!$&'()*+,;="),
        );
        ok(path_abempty, "/:/:", ("", "/:/:"));
        ok(path_abempty, "/@/@", ("", "/@/@"));

        ok(path_abempty, "/a/", ("", "/a/"));
        ok(path_abempty, "//", ("", "//"));

        ok(path_abempty, "/a?query", ("?query", "/a"));
        ok(path_abempty, "/a#fragment", ("#fragment", "/a"));
        ok(
            path_abempty,
            "/a?query=value#fragment",
            ("?query=value#fragment", "/a"),
        );

        ok(path_abempty, "/%g0", ("%g0", "/"));
    }
}
