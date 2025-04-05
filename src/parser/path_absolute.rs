use nom::{IResult, Input as _, Offset as _, Parser};

use crate::parser::{segment, segment_nz};

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

/// path-absolute = "/" [ segment-nz *( "/" segment ) ]
///
/// <https://datatracker.ietf.org/doc/html/rfc3986#section-3.3>
pub fn path_absolute(i: Span) -> IResult<Span, Token> {
    let start = i;
    let (i, _) = nom::character::complete::char('/').parse(i)?;
    let (i, _) = nom::combinator::opt((
        segment_nz,
        nom::multi::many0((nom::character::complete::char('/'), segment)),
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
    fn test_path_absolute() {
        ok(path_absolute, "/abc", ("", "/abc"));
        ok(path_absolute, "/123", ("", "/123"));
        ok(path_absolute, "/a-b.c_d~", ("", "/a-b.c_d~"));
        ok(path_absolute, "/%2f", ("", "/%2f"));
        ok(path_absolute, "/!$&'()*+,;=", ("", "/!$&'()*+,;="));
        ok(path_absolute, "/@", ("", "/@"));

        ok(path_absolute, "/abc/def", ("", "/abc/def"));
        ok(path_absolute, "/abc/def/ghi", ("", "/abc/def/ghi"));
        ok(path_absolute, "/abc/123/%20", ("", "/abc/123/%20"));
        ok(
            path_absolute,
            "/segment1/segment2/segment3",
            ("", "/segment1/segment2/segment3"),
        );

        ok(
            path_absolute,
            "/abc/!$&'()*+,;=/def",
            ("", "/abc/!$&'()*+,;=/def"),
        );
        ok(path_absolute, "/a:b/c@d/e~f", ("", "/a:b/c@d/e~f"));

        ok(path_absolute, "/abc//def", ("", "/abc//def"));
        ok(path_absolute, "/abc///def", ("", "/abc///def"));

        ok(path_absolute, "/abc/", ("", "/abc/"));
        ok(path_absolute, "/abc/def/", ("", "/abc/def/"));

        ok(path_absolute, "/abc/def?query", ("?query", "/abc/def"));
        ok(
            path_absolute,
            "/abc/def#fragment",
            ("#fragment", "/abc/def"),
        );
        ok(
            path_absolute,
            "/abc/def/ghi?query=value#fragment",
            ("?query=value#fragment", "/abc/def/ghi"),
        );

        err(path_absolute, "");
        err(path_absolute, ":");
        err(path_absolute, "a");
        err(path_absolute, "1");
        err(path_absolute, "%g0");
        ok(path_absolute, "/%g0", ("%g0", "/"));
        ok(path_absolute, "//host/path", ("/host/path", "/"));
    }
}
