use nom::{
    IResult, 
    character::complete::digit1,
    error::{ParseError, VerboseError, convert_error},
    sequence::pair,
    combinator::{opt, recognize},
    bytes::complete::tag
};
use nom::Finish;
use crate::days;

pub fn parse_int<'a, T, E>(input: &'a str) -> IResult<&'a str, T, E>
where T: std::str::FromStr,
E: ParseError<&'a str>
{
    match recognize(pair(
        opt(tag("-")), digit1
    ))(input) {
        Ok((rem, digits)) => match digits.parse::<T>() {
            Ok(res) => Ok((rem, res)),
            Err(_) => Err(nom::Err::Error(E::from_error_kind(input, nom::error::ErrorKind::Digit))),
        },
        Err(a) => Err(a),
    }
}

pub fn make_verbose_error_message<I, O>(
    full_input: I,
    res: IResult<I, O, VerboseError<I>>) -> Result<(I, O), days::error::Error>
where I: std::ops::Deref<Target = str> + std::fmt::Debug
{
    if res.is_err() {
        let e = res.finish().err().unwrap();
        return Err(days::Error::ParsingWithVerboseErrorMessage(convert_error(full_input, e)));
    }

    Ok(res.unwrap())
}