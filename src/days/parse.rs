use nom::{
    IResult, 
    character::complete::digit1,
    error::ParseError
};

pub fn parse_int<'a, T, E>(input: &'a str) -> IResult<&'a str, T, E>
where T: std::str::FromStr,
E: ParseError<&'a str>
{
    match digit1(input) {
        Ok((rem, digits)) => match digits.parse::<T>() {
            Ok(res) => Ok((rem, res)),
            Err(_) => Err(nom::Err::Error(E::from_error_kind(input, nom::error::ErrorKind::Digit))),
        },
        Err(a) => Err(a),
    }
}