use nom::{IResult, Input as _, Offset as _, Parser};

use crate::parser::{path_abempty, path_absolute, path_empty, path_noscheme, path_rootless};

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

/// path          = path-abempty    ; begins with "/" or is empty
///               / path-absolute   ; begins with "/" but not "//"
///               / path-noscheme   ; begins with a non-colon segment
///               / path-rootless   ; begins with a segment
///               / path-empty      ; zero characters
///
/// <https://datatracker.ietf.org/doc/html/rfc3986#section-3.3>
pub fn path(i: Span) -> IResult<Span, Token> {
    let start = i;
    let (i, _) = nom::branch::alt((
        path_absolute.map(|t| Token { span: t.span() }),
        path_noscheme.map(|t| Token { span: t.span() }),
        path_rootless.map(|t| Token { span: t.span() }),
        path_abempty.map(|t| Token { span: t.span() }),
        path_empty.map(|t| Token { span: t.span() }),
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
    fn test_path() {
        ok(path, "", ("", ""));

        ok(path, "a", ("", "a"));
        ok(path, "1", ("", "1"));
        ok(path, "-._~", ("", "-._~"));
        ok(path, "%20", ("", "%20"));
        ok(path, "!$&'()*+,;=", ("", "!$&'()*+,;="));
        ok(path, ":", ("", ":"));
        ok(path, "@", ("", "@"));

        ok(path, "/a", ("", "/a"));
        ok(path, "/1", ("", "/1"));
        ok(path, "/-._~", ("", "/-._~"));
        ok(path, "/%20", ("", "/%20"));
        ok(path, "/!$&'()*+,;=", ("", "/!$&'()*+,;="));
        ok(path, "/:", ("", "/:"));
        ok(path, "/@", ("", "/@"));

        ok(path, "/a/a", ("", "/a/a"));
        ok(path, "/1/1", ("", "/1/1"));
        ok(path, "/-._~/-._~", ("", "/-._~/-._~"));
        ok(path, "/%20/%20", ("", "/%20/%20"));
        ok(
            path,
            "/!$&'()*+,;=/!$&'()*+,;=",
            ("", "/!$&'()*+,;=/!$&'()*+,;="),
        );
        ok(path, "/:/:", ("", "/:/:"));
        ok(path, "/@/@", ("", "/@/@"));

        ok(path, "/a/", ("", "/a/"));

        // NOTE
        // if the path is path-abempty, it should be `"//"`.
        // if the path is path-absolute, it should be `"/"`.
        ok(path, "//", ("/", "/"));

        ok(path, "/a?query", ("?query", "/a"));
        ok(path, "/a#fragment", ("#fragment", "/a"));
        ok(
            path,
            "/a?query=value#fragment",
            ("?query=value#fragment", "/a"),
        );

        ok(path, "/%g0", ("%g0", "/"));
    }
}
