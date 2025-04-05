mod path_rootless;
mod pchar;
mod pct_encoded;
mod scheme;
mod segment;
mod segment_nz;
mod segment_nz_nc;
mod sub_delims;
mod unreserved;

use nom_locate::LocatedSpan;

pub(crate) type Span<'a> = LocatedSpan<&'a str>;
pub(crate) trait HasSpan<'a> {
    fn span(&self) -> Span<'a>;
}

pub use self::pchar::pchar;
pub use self::pct_encoded::pct_encoded;
pub use self::scheme::scheme;
pub use self::segment::segment;
pub use self::segment_nz::segment_nz;
pub use self::sub_delims::sub_delims;
pub use self::unreserved::unreserved;

#[cfg(test)]
mod tests {
    use nom::IResult;

    use super::*;

    pub(crate) fn err<'a, T>(f: fn(Span<'a>) -> IResult<Span<'a>, T>, s: &'a str) {
        use nom_locate::LocatedSpan;

        let s = LocatedSpan::new(s);
        assert!(f(s).is_err());
    }

    pub(crate) fn ok<'a, T: HasSpan<'a>>(
        f: fn(Span<'a>) -> IResult<Span<'a>, T>,
        s: &'a str,
        (i, o): (&'a str, &'a str),
    ) {
        use nom::Input;
        use nom_locate::LocatedSpan;

        let s = LocatedSpan::new(s);
        let expected = (
            s.take_from(s.rfind(i).expect("i")),
            s.take_from(s.find(o).expect("o")).take(o.len()),
        );
        assert_eq!(f(s).map(|(i, o)| (i, o.span())).expect("f"), expected);
    }
}
