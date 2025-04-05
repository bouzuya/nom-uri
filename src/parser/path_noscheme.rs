use nom::{IResult, Input as _, Offset as _, Parser};

use crate::parser::{segment, segment_nz_nc};

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

/// path-noscheme = segment-nz-nc *( "/" segment )
///
/// <https://datatracker.ietf.org/doc/html/rfc3986#section-3.3>
pub fn path_noscheme(i: Span) -> IResult<Span, Token> {
    let start = i;
    let (i, _) = segment_nz_nc.parse(i)?;
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
        ok(path_noscheme, "abc", ("", "abc"));
        ok(path_noscheme, "123", ("", "123"));
        ok(path_noscheme, "a-b.c_d~", ("", "a-b.c_d~"));
        ok(path_noscheme, "%2f", ("", "%2f"));
        ok(path_noscheme, "!$&'()*+,;=", ("", "!$&'()*+,;="));
        ok(path_noscheme, "@", ("", "@"));

        ok(path_noscheme, "abc/def", ("", "abc/def"));
        ok(path_noscheme, "abc/def/ghi", ("", "abc/def/ghi"));
        ok(path_noscheme, "abc/123/%20", ("", "abc/123/%20"));
        ok(
            path_noscheme,
            "segment1/segment2/segment3",
            ("", "segment1/segment2/segment3"),
        );

        ok(
            path_noscheme,
            "abc/!$&'()*+,;=/def",
            ("", "abc/!$&'()*+,;=/def"),
        );
        ok(path_noscheme, "a:b/c@d/e~f", (":b/c@d/e~f", "a"));

        ok(path_noscheme, "abc//def", ("", "abc//def"));
        ok(path_noscheme, "abc///def", ("", "abc///def"));

        ok(path_noscheme, "abc/", ("", "abc/"));
        ok(path_noscheme, "abc/def/", ("", "abc/def/"));

        ok(path_noscheme, "abc/def?query", ("?query", "abc/def"));
        ok(path_noscheme, "abc/def#fragment", ("#fragment", "abc/def"));
        ok(
            path_noscheme,
            "abc/def/ghi?query=value#fragment",
            ("?query=value#fragment", "abc/def/ghi"),
        );

        err(path_noscheme, "");
        err(path_noscheme, ":");
        err(path_noscheme, "/abc");
        err(path_noscheme, "//host/path");
        err(path_noscheme, "%g0");
    }
}
