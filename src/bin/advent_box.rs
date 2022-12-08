use clap::{Command, Arg};
use std::collections::BTreeMap;
use advent_of_code_2022::days;
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

    let map_day_str_to_fn = BTreeMap::from(
        days::DAYS_FUNCS_NAMES_AND_PTRS
    );
    let day_str = matches.get_one::<String>("day").unwrap();
    let day_fn = map_day_str_to_fn.get(day_str.as_str()).context("Did not find day fn")?;

    let mut day_input_file = get_input_file(day_str)?;

    day_fn(&mut day_input_file).map_err(|err| {
        match err {
            days::Error::Parsing(parsing) => anyhow!(get_parsing_error_msg(parsing)),
            _ => anyhow!("Encountered {err} while running day fn\n\
                          details: {err:?}")
        }

    })?;

    Ok(())
}

fn get_input_file(day_str: &String) -> Result<File>
{
    let day_input_filepath_1 = format!("inputs/{day_str}.txt");
    let day_input_file = File::open(&day_input_filepath_1);
    if day_input_file.is_ok() {
        return Ok(day_input_file.unwrap());
    }
    let day_input_filepath_2 = format!("inputs/{}.txt", strip_part(day_str)?);
    let day_input_file = File::open(&day_input_filepath_2);
    if day_input_file.is_ok() {
        return Ok(day_input_file.unwrap());
    }

    Err(anyhow!("Cannot open input file (either {} or {})", day_input_filepath_1, day_input_filepath_2))
}

fn strip_part(day_str: &String) -> Result<String>
{
    let part_pos = day_str.find("_part_").ok_or(anyhow!("No _part_ in day_str"))?;
    Ok(format!("{}", &day_str[0..part_pos]))
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