pub mod error;
pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;
pub mod day_6;
pub mod day_7;
pub mod day_8;
pub mod day_9;
pub mod day_10;
pub mod day_11;
pub mod day_12;
pub mod day_13;
pub mod day_14;
pub mod day_15;
pub mod day_16;
pub mod day_17;
pub mod day_18;
pub mod day_19;
pub mod y2020_day_1;
pub mod y2020_day_25;
mod parse;
mod points;

pub use error::{Error, Result, Parsing};

// will generate something like
// [
//     ("day_1_part1", day_1::day_1_part_1 as fn(&mut Input) -> Result<()>),
//     ("day_1_part2", day_1::day_1_part_2 as fn(&mut Input) -> Result<()>),
//     ("day_2_part1", day_2::day_2_part_1 as fn(&mut Input) -> Result<()>),
//     (etc)
// ]
macro_rules! make_days_funcs_names_and_ptrs {
    ($day_max:expr, $Input:ty) => {
        pub const DAYS_FUNCS_NAMES_AND_PTRS: [(&str, fn(&mut $Input) -> Result<()>); $day_max*2] =
            seq_macro::seq!(N in 1..=$day_max {
                [
                    #(
                        (concat!("day_", N, "_part_1"), paste::paste!([<day_ N>]::[<day_ N _part_1>]) as fn(&mut $Input) -> Result<()>),
                        (concat!("day_", N, "_part_2"), paste::paste!([<day_ N>]::[<day_ N _part_2>]) as fn(&mut $Input) -> Result<()>),
                    )*
                ]
            });
    };
}
make_days_funcs_names_and_ptrs!(19, std::fs::File);

mod internal_common {
    pub use super::{Result, Error};
    pub use std::io::Read;

    pub trait Grid2DTypes {
        type DataType;
        type Node;
    }

    pub trait Grid2D: Grid2DTypes {
        fn get_node_from_id(&self, id: usize) -> Self::Node;
        fn get_node_left(&self, current: &Self::Node) -> Option<Self::Node>;
        fn get_node_right(&self, current: &Self::Node) -> Option<Self::Node>;
        fn get_node_up(&self, current: &Self::Node) -> Option<Self::Node>;
        fn get_node_down(&self, current: &Self::Node) -> Option<Self::Node>;
    }

    pub fn get_whole_input_as_string<Input>(input: &mut Input) -> Result<String>
    where Input: Read {
        let mut content = String::new();
        input.read_to_string(&mut content).map_err(|_| Error::NotUtf8)?;
        let content = content.replace("\r", "");
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