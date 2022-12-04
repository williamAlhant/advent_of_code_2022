use clap::{Command, Arg};
use std::collections::BTreeMap;
use advent_of_code_2022::days;
use advent_of_code_2022::days::all_days::*;
use anyhow::{Result, Context, anyhow};
use std::fs::File;

fn main() -> Result<()> {
    let matches = Command::new("advent_box")
        .arg(
            Arg::new("day")
            .long("day")
            .required(true)
            .takes_value(true)
        )
        .get_matches();

    type DayFn<Input, T> = fn(&mut Input) -> days::Result<T>;
    let map_day_str_to_fn = BTreeMap::from([
        ("day_1_part_1", day_1_part_1 as DayFn<_, _>),
        ("day_1_part_2", day_1_part_2 as DayFn<_, _>),
        ("day_2_part_1", day_2_part_1 as DayFn<_, _>),
        ("day_2_part_2", day_2_part_2 as DayFn<_, _>),
        ("day_3_part_1", day_3_part_1 as DayFn<_, _>),
        ("day_3_part_2", day_3_part_2 as DayFn<_, _>),
        ("day_4_part_1", day_4_part_1 as DayFn<_, _>),
        ("day_4_part_2", day_4_part_2 as DayFn<_, _>),
        ("y2020_day_1_part_1", days::y2020_day_1::y2020_day_1_part_1 as DayFn<_, _>),
        ("y2020_day_1_part_2", days::y2020_day_1::y2020_day_1_part_2 as DayFn<_, _>),
        ("y2020_day_25_part_1", days::y2020_day_25::y2020_day_25_part_1 as DayFn<_, _>),
    ]);
    let day_str = matches.get_one::<String>("day").unwrap();
    let day_fn = map_day_str_to_fn.get(day_str.as_str()).context("Did not find day fn")?;
    let day_input_filepath = format!("inputs/{day_str}.txt");
    let mut day_input_file = File::open(&day_input_filepath)
        .with_context(|| format!("Cannot open {day_input_filepath}"))?;

    day_fn(&mut day_input_file).map_err(|err| {
        match err {
            days::Error::Parsing(parsing) => anyhow!(get_parsing_error_msg(parsing)),
            _ => anyhow!("Encountered {err} while running day fn\n\
                          details: {err:?}")
        }

    })?;

    Ok(())
}

fn get_parsing_error_msg(parsing: days::error::Parsing) -> String {
    let line_num = parsing.line;
    if parsing.content.is_none() {
        return format!("Parsing error on line {line_num}");
    }
    let content = parsing.content.unwrap();
    let (token_pos, token_len) = {
        match parsing.token {
            Some(token) => (token.line_pos, token.token_len),
            None => (0, content.len())
        }
    };
    let mut underline = String::new();
    for _i in 0..token_pos {
        underline.insert(underline.len(), ' ');
    }
    for _i in 0..token_len {
        underline.insert(underline.len(), '^');
    }
    format!("Parsing error at line/col: {line_num}/{token_pos}\n\
             {content}\n\
             {underline}")
}