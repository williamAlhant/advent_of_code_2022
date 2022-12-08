use crate::days::internal_common::*;
use std::collections::HashSet;

type Position = (i32, i32);

fn move_in_direction(pos: &mut Position, dir: Direction)
{
    match dir {
        Direction::U => pos.1 += 1,
        Direction::D => pos.1 -= 1,
        Direction::L => pos.0 -= 1,
        Direction::R => pos.0 += 1,
    }
}

fn is_pulling(head_pos: Position, tail_pos: Position) -> bool
{
    (head_pos.0 - tail_pos.0).abs() > 1
    || (head_pos.1 - tail_pos.1).abs() > 1
}

fn pull_for_part_2(tail: &mut Position, head: Position)
{
    if head.0 - tail.0 < 0 {
        tail.0 -= 1;
    }
    else if head.0 - tail.0 > 0 {
        tail.0 += 1;
    }

    if head.1 - tail.1< 0 {
        tail.1 -= 1;
    }
    else if head.1 - tail.1 > 0 {
        tail.1 += 1;
    }
}

pub fn day_9_part_1<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let content = get_whole_input_as_string(input)?;

    let mut visited_pos: HashSet<Position> = HashSet::new();
    let mut head_pos: Position = (0, 0);
    let mut tail_pos: Position = (0, 0);
    visited_pos.insert(tail_pos);

    parse::parse_and_do_for_each_line(content.as_str(), |dir, num_steps| {
        for _ in 0..num_steps {
            let previous_head_pos = head_pos;
            move_in_direction(&mut head_pos, dir);
            if is_pulling(head_pos, tail_pos) {
                tail_pos = previous_head_pos;
                visited_pos.insert(tail_pos);
            }
        }
        Ok(())
    })?;

    println!("Num of visited positions is {}", visited_pos.len());

    Ok(())
}

pub fn day_9_part_2<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let content = get_whole_input_as_string(input)?;

    let mut visited_pos: HashSet<Position> = HashSet::new();
    let mut rope_pos: Vec<Position> = vec![(0, 0); 10];
    visited_pos.insert(rope_pos[9]);

    parse::parse_and_do_for_each_line(content.as_str(), |dir, num_steps| {
        for _ in 0..num_steps {
            move_in_direction(&mut rope_pos[0], dir);
            for i in 1..10 {
                if is_pulling(rope_pos[i], rope_pos[i - 1]) {
                    let head = rope_pos[i - 1];
                    pull_for_part_2(&mut rope_pos[i], head);
                }
            }
            visited_pos.insert(rope_pos[9]);
        }
        Ok(())
    })?;

    println!("Num of visited positions is {}", visited_pos.len());

    Ok(())
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    U,
    D,
    R,
    L
}

mod parse {
    use crate::days::parse::*;
    use nom::Finish;
    use nom::{
        character::complete::{newline, space1, anychar},
        sequence::{tuple, preceded, terminated},
        combinator::{opt, map_res},
        error::context
    };

    pub(super) fn parse_and_do_for_each_line<F>(input: &str, mut func: F) -> super::Result<()>
    where F: FnMut(super::Direction, usize) -> super::Result<()>
    {
        let mut i = input;
        while i.len() != 0 {

            let parse_dir = context("parse_dir", map_res(anychar,
                |c| match c {
                    'U' => Ok(super::Direction::U),
                    'D' => Ok(super::Direction::D),
                    'L' => Ok(super::Direction::L),
                    'R' => Ok(super::Direction::R),
                    _ => Err(())
                }
            ));

            let res = terminated(
                tuple((parse_dir, preceded(space1, parse_int::<usize, _>))),
                opt(newline)
            )(i);

            if res.is_err() {
                let e = res.finish().err().unwrap();
                return Err(super::Error::ParsingWithVerboseErrorMessage(nom::error::convert_error(input, e)));
            }

            let (new_i, (a, b)) = res.ok().unwrap();

            func(a, b)?;
            i = new_i;
        }

        Ok(())
    }

}