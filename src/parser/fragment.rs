use nom::{IResult, Input as _, Offset as _, Parser};

use crate::parser::pchar;

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

/// fragment    = *( pchar / "/" / "?" )
///
/// <https://datatracker.ietf.org/doc/html/rfc3986#section-3.5>
pub fn fragment(i: Span) -> IResult<Span, Token> {
    let start = i;
    let (i, _) = nom::multi::many0(nom::branch::alt((
        pchar.map(|_| ()),
        nom::character::complete::char('/').map(|_| ()),
        nom::character::complete::char('?').map(|_| ()),
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
    fn test_fragment() {
        ok(fragment, "", ("", ""));

        ok(fragment, "k=v", ("", "k=v"));
        ok(fragment, "a=1&b=2", ("", "a=1&b=2"));

        ok(fragment, "a1-._~", ("", "a1-._~"));
        ok(fragment, "%20%21", ("", "%20%21"));
        ok(fragment, "!$&'()*+,;=", ("", "!$&'()*+,;="));
        ok(fragment, ":", ("", ":"));
        ok(fragment, "@", ("", "@"));

        ok(fragment, "/", ("", "/"));
        ok(fragment, "///", ("", "///"));
        ok(fragment, "?", ("", "?"));
        ok(fragment, "???", ("", "???"));

        ok(fragment, "k=v#f2", ("#f2", "k=v"));
    }
}
