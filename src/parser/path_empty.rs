use nom::{IResult, Input as _, Offset as _};

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

/// path-empty    = 0<pchar>
///
/// <https://datatracker.ietf.org/doc/html/rfc3986#section-3.3>
pub fn path_empty(i: Span) -> IResult<Span, Token> {
    let start = i;
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
    fn test_path_empty() {
        ok(path_empty, "", ("", ""));
        ok(path_empty, "a", ("a", ""));
        ok(path_empty, "1", ("1", ""));
    }
}
