use crate::days::internal_common::*;
use std::collections::HashMap;

pub fn day_23_part_1<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let input = get_whole_input_as_string(input)?;
    let mut grid = Grid::new(&input, 10);
    
    let mut rectangle = Rectangle::new(&grid.elves);

    for _round in 0..10 {
        grid.do_round(|new_pos| rectangle.update(&new_pos));
    }

    let empty_area = rectangle.area() as usize - grid.elves.len();
    print!("{:?}", empty_area);

    Ok(())
}

pub fn day_23_part_2<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let input = get_whole_input_as_string(input)?;
    let mut grid = Grid::new(&input, 100);
    
    let mut num_rounds = 0;
    let mut stopped_moving = false;

    while !stopped_moving {
        num_rounds += 1;
        stopped_moving = true;
        grid.do_round(|_| stopped_moving = false);
    }

    println!("Num rounds {}", num_rounds);

    Ok(())
}

type Point = crate::days::points::Point2<i32>;

#[derive(Clone, PartialEq, Eq)]
enum PointContent {
    Empty,
    Elf,
}

impl std::fmt::Display for PointContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(self.clone()))
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.size_y {
            for x in 0..self.size_x {
                write!(f, "{}", self.data[y * self.size_x + x])?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl std::convert::From<PointContent> for char {
    fn from(content: PointContent) -> Self
    {
        match content {
            PointContent::Empty => '.',
            PointContent::Elf => '#',
        }
    }
}

impl std::convert::From<char> for PointContent {
    fn from(c: char) -> Self
    {
        match c {
            '.' => PointContent::Empty,
            '#' => PointContent::Elf,
            _ => panic!("Unexpected char")
        }
    }
}

struct Grid {
    data: Vec<PointContent>,
    size_x: usize,
    size_y: usize,
    elves: Vec<Point>,
    start_dir_i: usize,
}

impl Grid {
    fn new(input: &str, num_rounds: usize) -> Self
    {
        let unpad_size_x = input.lines().map(|line| line.len()).max().unwrap();
        let unpad_size_y = input.lines().count();
        let size_x = unpad_size_x + num_rounds * 2;
        let size_y = unpad_size_y + num_rounds * 2;
        let mut elves = Vec::new();
        let mut data = vec![PointContent::Empty; size_x * size_y];
        let mut offset = num_rounds * size_x + num_rounds;
        for line in input.lines() {
            for (i, c) in line.chars().enumerate() {
                let content = PointContent::from(c);
                if content == PointContent::Elf {
                    let x = (num_rounds + i) as i32;
                    let y = (offset / size_x) as i32;
                    elves.push(Point::from_xy(x, y));
                }
                data[offset + i] = content;
            }
            offset += size_x;
        }
        Self {
            data, size_x, size_y, elves, start_dir_i: 0
        }
    }
}

impl WithDataType for Grid {
    type DataType = PointContent;
}

crate::impl_grid_2d_access_with_point!(Point, i32, Grid);

impl Grid {
    fn get_content_at_point(&self, point: &Point) -> Option<PointContent>
    {
        self.get_ref_content_at_point(point).map(|x| x.clone())
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Top,
    Bottom,
    Left,
    Right
}

impl Point {
    fn get_neigh(&self, dir: &Direction) -> Point
    {
        match dir {
            Direction::Top => self + Point::from_xy(0, -1),
            Direction::Bottom => self + Point::from_xy(0, 1),
            Direction::Left => self + Point::from_xy(-1, 0),
            Direction::Right => self + Point::from_xy(1, 0),
        }
    }
}

impl Grid {
    fn is_neigh_available(&self, pos: &Point, dir: &Direction) -> bool
    {
        match dir {
            Direction::Top => {
                let neigh = pos + Point::from_xy(0, -1);
                for d_x in -1..=1 {
                    if self.get_content_at_point(&Point::from_xy(neigh.x + d_x, neigh.y)).unwrap() == PointContent::Elf {
                        return false;
                    }
                }
                true
            },
            Direction::Bottom => {
                let neigh = pos + Point::from_xy(0, 1);
                for d_x in -1..=1 {
                    if self.get_content_at_point(&Point::from_xy(neigh.x + d_x, neigh.y)).unwrap() == PointContent::Elf {
                        return false;
                    }
                }
                true
            },
            Direction::Left => {
                let neigh = pos + Point::from_xy(-1, 0);
                for d_y in -1..=1 {
                    if self.get_content_at_point(&Point::from_xy(neigh.x, neigh.y + d_y)).unwrap() == PointContent::Elf {
                        return false;
                    }
                }
                true
            },
            Direction::Right => {
                let neigh = pos + Point::from_xy(1, 0);
                for d_y in -1..=1 {
                    if self.get_content_at_point(&Point::from_xy(neigh.x, neigh.y + d_y)).unwrap() == PointContent::Elf {
                        return false;
                    }
                }
                true
            },
        }
    }

    fn get_pos_id(&self, pos: &Point) -> i32
    {
        pos.y * self.size_x as i32 + pos.x
    }

    fn do_round<T>(&mut self, mut callback: T) where T: FnMut(Point)
    {
        let num_elves = self.elves.len();
        let mut used_positions: HashMap<i32, usize> = HashMap::with_capacity(num_elves);
        let mut next_positions = self.elves.clone();
        let directions = &[Direction::Top, Direction::Bottom, Direction::Left, Direction::Right];

        for elf_i in 0..num_elves {
            let pos = &self.elves[elf_i];

            if directions.iter().all(|dir| self.is_neigh_available(pos, &dir)) {
                continue;
            }

            for dir_i in 0..4 {
                let dir = directions[(self.start_dir_i + dir_i) % 4];
                if self.is_neigh_available(pos, &dir) {
                    let next_pos = pos.get_neigh(&dir);
                    used_positions.entry(self.get_pos_id(&next_pos))
                        .and_modify(|x| *x += 1)
                        .or_insert(1);
                    next_positions[elf_i] = next_pos;
                    break;
                }
            }
        }

        for elf_i in 0..num_elves {
            let pos = &self.elves[elf_i].clone();
            let next_pos = &next_positions[elf_i];
            if pos == next_pos {
                continue;
            }
            if *used_positions.get(&self.get_pos_id(next_pos)).unwrap() < 2 {
                self.put_content_at_point(pos, PointContent::Empty);
                self.put_content_at_point(next_pos, PointContent::Elf);
                self.elves[elf_i] = next_pos.clone();
                callback(next_pos.clone());
            }
        }

        used_positions.clear();
        self.start_dir_i = (self.start_dir_i + 1) % 4;
    }
}

#[derive(Debug)]
struct Rectangle {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl Rectangle {
    fn new(positions: &Vec<Point>) -> Self {
        let mut ret = Self {
            min_x: i32::MAX,
            max_x: i32::MIN,
            min_y: i32::MAX,
            max_y: i32::MIN,
        };
        for pos in positions {
            ret.update(pos);
        }
        ret
    }

    fn update(&mut self, pos: &Point)
    {
        self.min_x = self.min_x.min(pos.x);
        self.max_x = self.max_x.max(pos.x);
        self.min_y = self.min_y.min(pos.y);
        self.max_y = self.max_y.max(pos.y);
    }

    fn area(&self) -> i32
    {
        (self.max_x - self.min_x + 1) * (self.max_y - self.min_y + 1)
    }
}