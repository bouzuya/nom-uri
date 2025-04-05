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

/// path-rootless = segment-nz *( "/" segment )
///
/// <https://datatracker.ietf.org/doc/html/rfc3986#section-3.3>
pub fn path_rootless(i: Span) -> IResult<Span, Token> {
    let start = i;
    let (i, _) = segment_nz.parse(i)?;
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
    use crate::parser::tests::{err, ok};

    use super::*;

    #[test]
    fn test_path_rootless() {
        ok(path_rootless, "abc", ("", "abc"));
        ok(path_rootless, "123", ("", "123"));
        ok(path_rootless, "a-b.c_d~", ("", "a-b.c_d~"));
        ok(path_rootless, "%2f", ("", "%2f"));
        ok(path_rootless, "!$&'()*+,;=", ("", "!$&'()*+,;="));
        ok(path_rootless, "@", ("", "@"));
        ok(path_rootless, ":", ("", ":"));

        ok(path_rootless, "abc/def", ("", "abc/def"));
        ok(path_rootless, "abc/def/ghi", ("", "abc/def/ghi"));
        ok(path_rootless, "abc/123/%20", ("", "abc/123/%20"));
        ok(
            path_rootless,
            "segment1/segment2/segment3",
            ("", "segment1/segment2/segment3"),
        );

        ok(
            path_rootless,
            "abc/!$&'()*+,;=/def",
            ("", "abc/!$&'()*+,;=/def"),
        );
        ok(path_rootless, "a:b/c@d/e~f", ("", "a:b/c@d/e~f"));

        ok(path_rootless, "abc//def", ("", "abc//def"));
        ok(path_rootless, "abc///def", ("", "abc///def"));

        ok(path_rootless, "abc/", ("", "abc/"));
        ok(path_rootless, "abc/def/", ("", "abc/def/"));

        ok(path_rootless, "abc/def?query", ("?query", "abc/def"));
        ok(path_rootless, "abc/def#fragment", ("#fragment", "abc/def"));
        ok(
            path_rootless,
            "abc/def/ghi?query=value#fragment",
            ("?query=value#fragment", "abc/def/ghi"),
        );

        err(path_rootless, "");
        err(path_rootless, "/abc");
        err(path_rootless, "//host/path");
        err(path_rootless, "%g0");
    }
}
