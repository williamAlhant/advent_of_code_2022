use nom::{
    IResult, 
    character::complete::digit1,
    combinator::map_res
};

pub fn parse_int<T>(input: &str) -> IResult<&str, T>
where T: std::str::FromStr
{
    map_res(digit1, |s: &str| s.parse::<T>())(input)
}