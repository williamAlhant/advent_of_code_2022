pub mod error;
pub mod day_1;
pub mod y2020_day_1;
pub mod y2020_day_25;

pub use error::{Error, Result};

pub mod all_days {
    pub use super::day_1::day_1_part_1;
    pub use super::day_1::day_1_part_2;
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
}