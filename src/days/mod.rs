pub mod error;
pub mod day_1;
pub mod y2020_day_1;
pub mod y2020_day_25;

pub use error::{Error, Result};

pub mod all_days {
    pub use super::day_1::day_1_part_1;
}

mod internal_common {
    pub use super::{Result, Error};
    pub use std::io::Read; 
}