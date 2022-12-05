use std::collections::VecDeque;
use crate::days::internal_common::*;

pub fn day_5_part_1<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    day_5(input, |num_crates, from_idx, to_idx, stacks| {
        for _i in 0..num_crates {
            let crate_char = stacks.get_mut(from_idx).unwrap().pop_front().unwrap();
            stacks.get_mut(to_idx).unwrap().push_front(crate_char);
        }
    })
}

pub fn day_5_part_2<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    day_5(input, |num_crates, from_idx, to_idx, stacks| {
        let mut buffer: VecDeque<char> = VecDeque::new();
        for _i in 0..num_crates {
            let crate_char = stacks.get_mut(from_idx).unwrap().pop_front().unwrap();
            buffer.push_front(crate_char);
        }
        for _i in 0..num_crates {
            let crate_char = buffer.pop_front().unwrap();
            stacks.get_mut(to_idx).unwrap().push_front(crate_char);
        }
    })
}

fn day_5<Input, F>(input: &mut Input, mut each_move_func: F) -> Result<()>
where Input: Read,
F: FnMut(usize, usize, usize, &mut Vec<VecDeque<char>>)
{
    let content = get_whole_input_as_string(input)?;
    let (line_with_stack_numbers_idx, line_with_stack_numbers) = content.lines().enumerate().take_while(|(_, line)| line.len() != 0).last().unwrap();
    let num_of_stacks = line_with_stack_numbers.split_whitespace().last().unwrap();
    let num_of_stacks = num_of_stacks.parse::<usize>().unwrap();

    let mut content_lines = content.lines();

    let mut stacks = parse_stacks(&mut content_lines, num_of_stacks, line_with_stack_numbers_idx)?;

    content_lines.next().ok_or(Error::UnexpectedInputEnd)?;
    content_lines.next().ok_or(Error::UnexpectedInputEnd)?;

    for line in content_lines {
        let (a, b, c) = parse::parse_move(line)?;
        each_move_func(a, b, c, &mut stacks);
    }

    let mut result = String::new();
    for stack in stacks {
        result.push(*stack.front().unwrap());
    }
    println!("Result is {}", result);

    Ok(())
}

fn parse_stacks(lines: &mut std::str::Lines, num_of_stacks: usize, num_of_lines_to_parse: usize) -> Result<Vec<VecDeque<char>>>
{
    let mut stacks: Vec<VecDeque<char>> = vec![VecDeque::default(); num_of_stacks];
    let mut line_idx = 0;
    while line_idx < num_of_lines_to_parse {
        let line = lines.next();
        if line.is_none() {
            return Err(Error::UnexpectedInputEnd);
        }
        let line = line.unwrap();
        for (stack_idx, crate_char) in parse_crates(line).unwrap() {
            let stack = stacks.get_mut(stack_idx).unwrap();
            stack.push_back(crate_char);
        }
        line_idx += 1;
    }
    Ok(stacks)
}

fn parse_crates(line: &str) -> Result<Vec<(usize, char)>>
{
    let mut ret = Vec::new();
    let line: Vec<char> = line.chars().collect();
    for char_idx in 0..line.len() {
        if line[char_idx] == '[' {
            let crate_char = line[char_idx + 1];
            let stack_num = char_idx / 4;
            ret.push((stack_num, crate_char));
        }
    }
    Ok(ret)
}

mod parse {

    use crate::days::parse::*;
    use nom::{
        IResult,
        bytes::complete::tag,
        sequence::{tuple, preceded},
    };

    pub fn parse_move(line: &str) -> super::Result<(usize, usize, usize)>
    {
        let (a, b, c) = 
            parse_move_internal(line).map(|(_, r)| r).map_err(|_| super::Error::new_token(0, line.len()))?;
        Ok((a, b - 1, c - 1))
    }

    fn parse_move_internal<'a>(i: &'a str) -> IResult<&'a str, (usize, usize, usize)> {
        tuple((
            preceded(tag("move "), parse_int),
            preceded(tag(" from "), parse_int),
            preceded(tag(" to "), parse_int),
        ))(i)
    }

}