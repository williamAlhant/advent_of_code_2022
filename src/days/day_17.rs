use crate::days::internal_common::*;
use std::fmt;

pub fn day_17_part_1<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let content = get_whole_input_as_string(input)?;
    let mut move_chars_iter = content.chars().cycle();
    let stop_after_n_rocks = 2022;
    let width = 7;
    let height = stop_after_n_rocks * 4;
    let mut grid = Grid {
        data: vec![PointContent::Air; width * height],
        width,
        height,
        reached_y: 0
    };
    let mut num_rocks = 0;
    let mut current_shape_id = 0;
    let shape_defs = get_shape_defs();

    while num_rocks < stop_after_n_rocks {
        let current_shape = &shape_defs[current_shape_id];
        let mut pos = Point::from_xy(2, grid.reached_y + 3);
        assert!(pos.y < grid.height as i32);
        loop {
            let move_char = move_chars_iter.next().unwrap();
            move_if_possible(current_shape, &mut pos, &grid, move_char);
            if can_fall(current_shape, &pos, &grid) {
                pos.y -= 1;
            }
            else {
                break;
            }
        }
        put_rocks_in_grid(current_shape, &pos, &mut grid);
        num_rocks += 1;
        current_shape_id = (current_shape_id + 1) % shape_defs.len();
    }

    println!("Reached height {}", &grid.reached_y);

    Ok(())
}

pub fn day_17_part_2<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let content = get_whole_input_as_string(input)?;
    let move_chars: Vec<char> = content.chars().collect();
    let mut current_move_id: usize = 0;
    let width = 7;
    let height = 10_000;
    let mut grid = Grid {
        data: vec![PointContent::Air; width * height],
        width,
        height,
        reached_y: 0
    };
    let mut num_rocks = 0;
    let mut current_shape_id = 0;
    let shape_defs = get_shape_defs();

    let mut known_states: Vec<Option<(MoveShape, NumStuff)>> = vec![None; 256];
    let mut found_period = false;
    let mut stop_after_n_rocks = 0;
    let mut periodic_height_diff = 0;
    let mut predicted_num_periods = 0;

    while !found_period || num_rocks < stop_after_n_rocks {
        let current_shape = &shape_defs[current_shape_id];

        if grid.reached_y > 0 && !found_period {
            let current_state = MoveShape {move_id: current_move_id, shape_id: current_shape_id};
            let current_num_stuff = NumStuff {num_rocks, reached_height: grid.reached_y as usize};
            let id = grid.highest_row_row_state_id();
            match known_states[id as usize] {
                None => {
                    known_states[id as usize] = Some((current_state, current_num_stuff));
                },
                Some((previous_state, previous_num_stuff)) => {
                    if previous_state == current_state {
                        found_period = true;
                        println!("Matching already known state at y={}, num_rocks={}", grid.reached_y, num_rocks);
                        let period = current_num_stuff.num_rocks - previous_num_stuff.num_rocks;
                        periodic_height_diff = current_num_stuff.reached_height - previous_num_stuff.reached_height;
                        // A + N*period + B = I
                        let terms_i: usize = 1_000_000_000_000;
                        let terms_a: usize = previous_num_stuff.num_rocks;
                        let terms_n = (terms_i - terms_a) / period;
                        let terms_b = terms_i - terms_a - terms_n * period;
                        stop_after_n_rocks = terms_a + period + terms_b;
                        predicted_num_periods = terms_n;
                    }
                },
            };
        }

        let mut pos = Point::from_xy(2, grid.reached_y + 3);
        assert!(pos.y < grid.height as i32);
        loop {
            let move_char = move_chars[current_move_id];
            current_move_id = (current_move_id + 1) % move_chars.len();
            move_if_possible(current_shape, &mut pos, &grid, move_char);
            if can_fall(current_shape, &pos, &grid) {
                pos.y -= 1;
            }
            else {
                break;
            }
        }
        put_rocks_in_grid(current_shape, &pos, &mut grid);
        num_rocks += 1;
        current_shape_id = (current_shape_id + 1) % shape_defs.len();
    }

    let predicted_height = grid.reached_y as usize + (predicted_num_periods - 1) * periodic_height_diff;
    println!("predicted_height {}", &predicted_height);

    Ok(())
}

#[derive(Clone, Copy, PartialEq)]
struct MoveShape {
    shape_id: usize,
    move_id: usize,
}

#[derive(Clone, Copy)]
struct NumStuff {
    num_rocks: usize,
    reached_height: usize
}

fn can_fall(shape: &ShapeDef, pos: &Point, grid: &Grid) -> bool
{
    let mut next_pos = pos.clone();
    next_pos.y -= 1;
    is_possible_pos(shape, &next_pos, grid)
}

fn move_if_possible(shape: &ShapeDef, pos: &mut Point, grid: &Grid, move_char: char)
{
    let mut next_pos = pos.clone();
    next_pos.x += match move_char {
        '>' => 1,
        '<' => -1,
        _ => panic!("Unexpected move char"),
    };
    if is_possible_pos(shape, &next_pos, grid) {
        pos.x = next_pos.x;
    }
}

fn is_possible_pos(shape: &ShapeDef, pos: &Point, grid: &Grid) -> bool
{
    for rock_rel_pos in &shape.rocks {
        let rock_abs_pos = pos + rock_rel_pos;
        let content = grid.get_content_at_point(&rock_abs_pos);
        if content.is_none() {
            return false;
        }
        let content = content.unwrap();
        if content == PointContent::Rock {
            return false;
        }
    }
    true
}

fn put_rocks_in_grid(shape: &ShapeDef, pos: &Point, grid: &mut Grid)
{
    for rock_rel_pos in &shape.rocks {
        let rock_abs_pos = pos + rock_rel_pos;
        let reached_height = rock_abs_pos.y + 1;
        if reached_height > grid.reached_y {
            grid.reached_y = reached_height;
        }
        grid.put_content_at_point(&rock_abs_pos, PointContent::Rock);
    }
}

#[derive(Debug)]
struct Grid {
    data: Vec<PointContent>,
    width: usize,
    height: usize,
    reached_y: i32
}

impl Grid {
    fn get_content_at_point(&self, point: &Point) -> Option<PointContent>
    {
        if !(0..(self.width as i32)).contains(&point.x) || !(0..(self.height as i32)).contains(&point.y) {
            return None;
        }
        let id = (point.y as usize) * self.width + (point.x as usize);
        Some(self.data[id])
    }

    fn put_content_at_point(&mut self, point: &Point, content: PointContent)
    {
        let id = (point.y as usize) * self.width + (point.x as usize);
        self.data[id] = content;
    }

    fn highest_row_row_state_id(&self) -> u8
    {
        assert!(self.reached_y != 0);
        let row_start_id = (self.reached_y as usize - 1) * self.width;
        make_row_state_id(&self.data[row_start_id..(row_start_id + self.width)])
    }
}

fn make_row_state_id(row_state: &[PointContent]) -> u8
{
    let mut id: u8 = 0;
    assert!(row_state.len() <= 8);
    for i in 0..row_state.len() {
        let v = match row_state[i] {
            PointContent::Air => 0,
            PointContent::Rock => 1
        };
        id |= v << i;
    }
    id
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i_y in (0..self.height).rev() {
            let mut line_buffer = String::with_capacity(self.width);
            for i_x in 0..self.width {
                line_buffer.push(self.data[i_y * self.width + i_x].to_char());
            }
            writeln!(f, "{}", line_buffer)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq)]
enum PointContent {
    Air,
    Rock
}

impl PointContent {
    fn to_char(&self) -> char
    {
        match self {
            Self::Air => '.',
            Self::Rock => '#',
        }
    }
}

impl fmt::Debug for PointContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

struct ShapeDef {
    rocks: Vec<Point>
}

impl ShapeDef {
    fn from_rocks(rocks: Vec<Point>) -> Self
    {
        Self {
            rocks
        }
    }
}

fn get_shape_defs() -> Vec<ShapeDef>
{
    let mut shape_defs = Vec::new();
    shape_defs.push(ShapeDef::from_rocks(vec![
        Point::from_xy(0, 0),
        Point::from_xy(1, 0),
        Point::from_xy(2, 0),
        Point::from_xy(3, 0),
    ]));
    shape_defs.push(ShapeDef::from_rocks(vec![
        Point::from_xy(1, 0),
        Point::from_xy(0, 1),
        Point::from_xy(1, 1),
        Point::from_xy(2, 1),
        Point::from_xy(1, 2),
    ]));
    shape_defs.push(ShapeDef::from_rocks(vec![
        Point::from_xy(0, 0),
        Point::from_xy(1, 0),
        Point::from_xy(2, 0),
        Point::from_xy(2, 1),
        Point::from_xy(2, 2),
    ]));
    shape_defs.push(ShapeDef::from_rocks(vec![
        Point::from_xy(0, 0),
        Point::from_xy(0, 1),
        Point::from_xy(0, 2),
        Point::from_xy(0, 3),
    ]));
    shape_defs.push(ShapeDef::from_rocks(vec![
        Point::from_xy(0, 0),
        Point::from_xy(1, 0),
        Point::from_xy(0, 1),
        Point::from_xy(1, 1),
    ]));
    shape_defs
}

type Point = crate::days::points::Point2<i32>;
