use nom::{IResult, Input as _, Offset as _, Parser};

use crate::parser::{authority, path_abempty, path_absolute, path_empty, path_rootless};

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

/// hier-part   = "//" authority path-abempty
///             / path-absolute
///             / path-rootless
///             / path-empty
///
/// <https://datatracker.ietf.org/doc/html/rfc3986#section-3>
pub fn hier_part(i: Span) -> IResult<Span, Token> {
    let start = i;
    let (i, _) = nom::branch::alt((
        (nom::bytes::complete::tag("//"), authority, path_abempty).map(|_| ()),
        path_absolute.map(|_| ()),
        path_rootless.map(|_| ()),
        path_empty.map(|_| ()),
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
    fn test_hier_part() {
        // "//" authority path-abempty
        ok(hier_part, "//example.com", ("", "//example.com"));
        ok(hier_part, "//example.com/", ("", "//example.com/"));
        ok(hier_part, "//example.com/path", ("", "//example.com/path"));
        // TODO: userinfo + port
        // ok(
        //     hier_part,
        //     "//user:pass@example.com:8080/path",
        //     ("", "//user:pass@example.com:8080/path"),
        // );

        // path-absolute
        ok(hier_part, "/", ("", "/"));
        ok(hier_part, "/path", ("", "/path"));
        ok(hier_part, "/path/to/resource", ("", "/path/to/resource"));
        ok(hier_part, "/path/to/resource/", ("", "/path/to/resource/"));

        // path-rootless
        ok(hier_part, "path", ("", "path"));
        ok(hier_part, "path/to/resource", ("", "path/to/resource"));
        ok(hier_part, "path/to/resource/", ("", "path/to/resource/"));

        // path-empty
        ok(hier_part, "", ("", ""));

        ok(hier_part, "#f", ("#f", ""));
        ok(hier_part, "?q", ("?q", ""));

        ok(
            hier_part,
            "//example.com?query",
            ("?query", "//example.com"),
        );
        ok(hier_part, "/path#fragment", ("#fragment", "/path"));
        ok(hier_part, "path?query", ("?query", "path"));
    }
}
