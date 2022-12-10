use crate::days::internal_common::*;
use std::collections::HashSet;

struct Vm {
    x: i32,
    cycle: i32
}

impl Vm {
    fn new() -> Self {
        Self {
            x: 1,
            cycle: 1
        }
    }

    fn increment_cycle_and_maybe_measure(&mut self, sum: &mut i32, cycles_when_we_measure: &HashSet<i32>) {
        self.cycle += 1;
        if cycles_when_we_measure.contains(&self.cycle) {
            *sum += self.x * self.cycle;
        }
    }

    fn get_pixel(&self, pos_in_line: i32) -> char {
        if (self.x - pos_in_line).abs() <= 1 {
            '#'
        }
        else {
            '.'
        }
    }

    fn increment_cycle_and_maybe_draw(&mut self, line_buffer: &mut Vec<char>) {
        let pos_in_line = self.cycle % 40;
        line_buffer[pos_in_line as usize] = self.get_pixel(pos_in_line);
        if pos_in_line == 39 {
            println!("{}", line_buffer.iter().collect::<String>());
        }
        self.cycle += 1;
    }
}

pub fn day_10_part_1<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let content = get_whole_input_as_string(input)?;

    let mut vm = Vm::new();
    let mut sum = 0;
    let cycles_when_we_measure: HashSet<i32> = HashSet::from(
        [20, 60, 100, 140, 180, 220]
    );

    parse::parse_and_do_for_each_line(content.as_str(), |instruction| {
        
        match instruction {
            Instruction::Addx(val) => {
                vm.increment_cycle_and_maybe_measure(&mut sum, &cycles_when_we_measure);
                vm.x += val;
                vm.increment_cycle_and_maybe_measure(&mut sum, &cycles_when_we_measure);
            },
            Instruction::Noop => {
                vm.increment_cycle_and_maybe_measure(&mut sum, &cycles_when_we_measure);
            }
        };
        Ok(())
    })?;

    println!("Sum is {}", sum);

    Ok(())
}

pub fn day_10_part_2<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let content = get_whole_input_as_string(input)?;

    let mut vm = Vm::new();
    let mut line_buffer: Vec<char> = vec!['.'; 40];

    parse::parse_and_do_for_each_line(content.as_str(), |instruction| {
        
        match instruction {
            Instruction::Addx(val) => {
                vm.increment_cycle_and_maybe_draw(&mut line_buffer);
                vm.x += val;
                vm.increment_cycle_and_maybe_draw(&mut line_buffer);
            },
            Instruction::Noop => {
                vm.increment_cycle_and_maybe_draw(&mut line_buffer);
            }
        };

        Ok(())
    })?;

    Ok(())
}

#[derive(Debug)]
enum Instruction {
    Addx(i32),
    Noop
}

mod parse {
    use crate::days::parse::*;
    use nom::{
        character::complete::newline,
        sequence::{preceded, terminated},
        combinator::{opt, map_res},
        branch::alt,
        bytes::complete::tag
    };

    pub(super) fn parse_and_do_for_each_line<F>(input: &str, mut func: F) -> super::Result<()>
    where F: FnMut(super::Instruction) -> super::Result<()>
    {
        let mut i = input;
        while i.len() != 0 {

            let parse_addx = map_res(
                preceded(tag("addx "), parse_int),
                |add_arg| Ok::<_, ()>(super::Instruction::Addx(add_arg))
            );

            let parse_noop = map_res(
                tag("noop"),
                |_| Ok::<_, ()>(super::Instruction::Noop)
            );

            let res = terminated(
                alt((
                    parse_addx,
                    parse_noop
                )),
                opt(newline)
            )(i);

            let (new_i, instruction) = make_verbose_error_message(input, res)?;

            func(instruction)?;
            i = new_i;
        }

        Ok(())
    }
}
