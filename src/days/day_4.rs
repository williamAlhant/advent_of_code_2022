use crate::days::internal_common::*;

fn range_contains_range(
    first_range: &std::ops::RangeInclusive<u32>,
    second_range: &std::ops::RangeInclusive<u32>
) -> bool {
    first_range.contains(second_range.start()) && first_range.contains(second_range.end())
}

fn range_overlaps_range(
    first_range: &std::ops::RangeInclusive<u32>,
    second_range: &std::ops::RangeInclusive<u32>
) -> bool {
    first_range.contains(second_range.start()) || second_range.contains(first_range.start())
}

pub fn day_4_part_1<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let mut sum = 0;
    do_for_each_line(input, |line| {
        let (first_range, second_range) = parse::parse_line(line)?;
        if range_contains_range(&first_range, &second_range)|| range_contains_range(&second_range, &first_range) {
            sum += 1;
        }
        Ok(())
    })?;
    println!("Sum is {}", sum);
    Ok(())
}

pub fn day_4_part_2<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let mut sum = 0;
    do_for_each_line(input, |line| {
        let (first_range, second_range) = parse::parse_line(line)?;
        if range_overlaps_range(&first_range, &second_range) {
            sum += 1;
        }
        Ok(())
    })?;
    println!("Sum is {}", sum);
    Ok(())
}

mod parse {

    use std::ops::RangeInclusive;
    use nom::{
        IResult, 
        bytes::complete::tag, 
        character::complete::digit1,
        sequence::tuple,
        combinator::map_res
    };

    pub fn parse_line(line: &str) -> super::Result<(RangeInclusive<u32>, RangeInclusive<u32>)>
    {
        parse_line_internal(line)
            .map(|(_, r)| r)
            .map_err(|_| super::Error::new_token(0, line.len()))
    }

    fn parse_line_internal(input: &str) -> IResult<&str, (RangeInclusive<u32>, RangeInclusive<u32>)>
    {
        let (input, first_range) = parse_range(input)?;
        let (input, _) = tag::<&str, &str, nom::error::Error<&str>>(",")(input)?;
        let (input, second_range) = parse_range(input)?;

        Ok((input, (first_range, second_range)))
    }

    pub fn parse_range<'a>(i: &'a str) -> IResult<&'a str, RangeInclusive<u32>> {
        tuple((parse_u32, tag("-"), parse_u32))(i)
            .and_then(|(i, (a, _, b))| Ok((i, a..=b)))
    }

    fn parse_u32(input: &str
    ) -> IResult<&str, u32> {
        map_res(digit1, |s: &str| s.parse::<u32>())(input)
    }

}