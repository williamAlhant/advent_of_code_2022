pub mod error;
pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod y2020_day_1;
pub mod y2020_day_25;

pub use error::{Error, Result, Parsing};

pub mod all_days {
    pub use super::day_1::day_1_part_1;
    pub use super::day_1::day_1_part_2;
    pub use super::day_2::day_2_part_1;
    pub use super::day_2::day_2_part_2;
    pub use super::day_3::day_3_part_1;
    pub use super::day_3::day_3_part_2;
    pub use super::day_4::day_4_part_1;
    pub use super::day_4::day_4_part_2;
}

mod internal_common {
    pub use super::{Result, Error};
    pub use std::io::Read;

    pub fn get_whole_input_as_string<Input>(input: &mut Input) -> Result<String>
    where Input: Read {
        let mut content = String::new();
        input.read_to_string(&mut content).map_err(|_| Error::NotUtf8)?;
        Ok(content)
    }

    pub fn do_for_each_line<I, F>(input: &mut I, mut func: F) -> Result<()>
    where I: Read,
    F: FnMut(&str) -> Result<()>
    {
        let content = get_whole_input_as_string(input)?;
        for (line_idx, line) in content.lines().enumerate() {
            func(line).map_err(
                |e| match e {
                    Error::ParsingToken(token) => Error::new_parsing_with_token(line, line_idx + 1, token),
                    _ => e
                })?;
        }
        Ok(())
    }
}