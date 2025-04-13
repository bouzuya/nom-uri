use nom::{IResult, Input as _, Offset as _, Parser};

use super::{HasSpan, Span};
use crate::parser::{sub_delims, unreserved};

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub span: Span<'a>,
}

impl<'a> HasSpan<'a> for Token<'a> {
    fn span(&self) -> Span<'a> {
        self.span
    }
}

/// IPvFuture  = "v" 1*HEXDIG "." 1*( unreserved / sub-delims / ":" )
///
/// <https://datatracker.ietf.org/doc/html/rfc3986#section-3.2.2>
pub fn ipvfuture(i: Span) -> IResult<Span, Token> {
    let start = i;
    let (i, _) = (
        nom::character::complete::char('v'),
        nom::multi::many1(nom::character::satisfy(|c| c.is_ascii_hexdigit())),
        nom::character::complete::char('.'),
        nom::multi::many1(nom::branch::alt((
            unreserved.map(|_| ()),
            sub_delims.map(|_| ()),
            nom::character::complete::char(':').map(|_| ()),
        ))),
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
    fn test_ipvfuture() {
        ok(ipvfuture, "v1.G", ("", "v1.G"));
        ok(ipvfuture, "vA.G", ("", "vA.G"));
        ok(ipvfuture, "v12AF.G", ("", "v1.G"));

        ok(ipvfuture, "v1.G1-._~", ("", "v1.1-._~"));
        ok(ipvfuture, "vF.!$&'()*+,;=", ("", "vF.!$&'()*+,;="));
        ok(ipvfuture, "vF.:", ("", "vF.:"));
    }
}
