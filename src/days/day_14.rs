use crate::days::internal_common::*;
use std::fmt;
use macro_lib::Grid2D;

pub fn day_14_part_1<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let input = get_whole_input_as_string(input)?;
    let lines = parse::parse_and_collect_lines(&input)?;
    
    let mut grid = Grid::from_lines(&lines);

    let mut num_units_at_rest = 0;
    while let SandUnitFinalState::Rest(_) = grid.pour_sand() {
        num_units_at_rest += 1;
    }

    println!("Num of units at rest is {}", num_units_at_rest);

    Ok(())
}

pub fn day_14_part_2<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let input = get_whole_input_as_string(input)?;
    let mut lines = parse::parse_and_collect_lines(&input)?;
    
    // add floor and make it large enough
    let (min_x, max_x, max_y) = get_min_max(&lines);
    let full_height = max_y + 2;
    let min_x = min_x.min(500 - full_height);
    let max_x = max_x.max(500 + full_height);
    lines.push(vec![Point {x: min_x, y: full_height}, Point {x: max_x, y: full_height}]);

    let mut grid = Grid::from_lines(&lines);

    let source_id = 500 - grid.min_original_x as usize;
    let mut num_units_at_rest = 0;
    while let SandUnitFinalState::Rest(node) = grid.pour_sand() {
        num_units_at_rest += 1;
        if node.id == source_id {
            break;
        }
    }

    println!("Num of units at rest is {}", num_units_at_rest);

    Ok(())
}

fn get_min_max(lines: &Vec<Line>) -> (u32, u32, u32) // min_x, max_x, max_y
{
    let mut min_x = u32::MAX;
    let mut max_x = 0;
    let mut max_y = 0;
    for line in lines {
        for point in line {
            if point.x < min_x {
                min_x = point.x;
            }
            if point.x > max_x {
                max_x = point.x;
            }
            if point.y > max_y {
                max_y = point.y;
            }
        }
    }
    (min_x, max_x, max_y)
}

type Line = Vec<Point>;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Point {
    x: u32,
    y: u32
}

enum Direction {
    Up,
    Down,
    Right,
    Left
}

impl Point {
    fn get_direction_to(&self, other: &Point) -> Direction
    {
        let d_x = other.x as i32 - self.x as i32;
        let d_y = other.y as i32 - self.y as i32;
        if d_x > 0 {
            return Direction::Right;
        }
        else if d_x < 0 {
            return Direction::Left;
        }
        if d_y > 0 {
            return Direction::Down;
        }
        else if d_y < 0 {
            return Direction::Up;
        }
        else {
            panic!("No direction");
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum PointContent {
    Air,
    Sand,
    Rock
}

impl PointContent {
    fn to_char(&self) -> char
    {
        match self {
            Self::Air => '.',
            Self::Sand => 'o',
            Self::Rock => '#',
        }
    }
}

impl fmt::Debug for PointContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Debug, Grid2D)]
struct Grid {
    data: Vec<PointContent>,
    width: usize,
    height: usize,
    min_original_x: u32
}

#[derive(Debug, Clone)]
struct Node {
    id: usize,
    data: PointContent
}

impl Grid2DTypes for Grid {
    type DataType = PointContent;
    type Node = Node;
}

enum SandUnitFinalState {
    Abyss,
    Rest(Node)
}

impl Grid {
    fn from_lines(lines: &Vec<Line>) -> Self
    {
        let (min_x, max_x, max_y) = get_min_max(lines);
        let width = (max_x - min_x + 1) as usize;
        let height = (max_y + 1) as usize;
        let data = vec![PointContent::Air; width * height];

        let mut grid = Grid {
            data,
            width,
            height,
            min_original_x: min_x
        };

        // translate points to local coordinates within grid
        let lines: Vec<Line> = lines.iter().map(
            |line| line.iter().map(|point| Point {x: point.x - min_x, y: point.y}).collect()
        ).collect();
        grid.put_rock_from_lines(&lines);

        grid
    }

    fn put_rock_from_lines(&mut self, lines: &Vec<Line>)
    {
        for line in lines {
            for i in 0..(line.len() - 1) {
                let dir = line[i].get_direction_to(&line[i + 1]);
                let mut current_point = line[i].clone();
                let mut current_node = self.get_node_from_id(current_point.y as usize * self.width + current_point.x as usize);
                self.data[current_node.id] = PointContent::Rock;
                while current_point != line[i + 1] {
                    match dir {
                        Direction::Down => {
                            current_point.y += 1;
                            current_node = self.get_node_down(&current_node).unwrap();
                        }
                        Direction::Up => {
                            current_point.y -= 1;
                            current_node = self.get_node_up(&current_node).unwrap();
                        }
                        Direction::Left => {
                            current_point.x -= 1;
                            current_node = self.get_node_left(&current_node).unwrap();
                        }
                        Direction::Right => {
                            current_point.x += 1;
                            current_node = self.get_node_right(&current_node).unwrap();
                        }
                    }
                    self.data[current_node.id] = PointContent::Rock;
                }
            }
        }
    }

    fn pour_sand(&mut self) -> SandUnitFinalState
    {
        let start_id = 500 - self.min_original_x as usize;
        let mut current = self.get_node_from_id(start_id);
        while let Some(down) = self.get_node_down(&current) {
            if down.data == PointContent::Air {
                current = down;
                continue;
            }
            else {
                let next = match self.get_node_left(&down) {
                    Some(x) => x,
                    None => return SandUnitFinalState::Abyss
                };
                if next.data == PointContent::Air {
                    current = next;
                    continue;
                }
                else {
                    let next = match self.get_node_right(&down) {
                        Some(x) => x,
                        None => return SandUnitFinalState::Abyss
                    };
                    if next.data == PointContent::Air {
                        current = next;
                        continue;
                    }
                    else {
                        self.data[current.id] = PointContent::Sand;
                        return SandUnitFinalState::Rest(current);
                    }
                }
            }
        }
        SandUnitFinalState::Abyss
    }

    // fn print(&self)
    // {
    //     for i_y in 0..self.height {
    //         let mut line_buffer = String::with_capacity(self.width);
    //         for i_x in 0..self.width {
    //             line_buffer.push(self.data[i_y * self.width + i_x].to_char());
    //         }
    //         println!("{}", line_buffer);
    //     }
    // }
}

mod parse {
    use crate::days::parse::*;
    use crate::days::parse::nom_goes_brrr::*;

    pub(super) fn parse_and_collect_lines(input: &str) -> super::Result<Vec<super::Line>>
    {
        let parse_point = map_res(
            separated_pair(parse_int, tag(","), parse_int),
            |(x, y)| Ok::<_, ()>(super::Point { x, y }));
        let parse_line = separated_list0(tag(" -> "), parse_point);

        let (_, lines) = make_verbose_error_message(input,
            many0(
                terminated(parse_line, newline)
            )(input)
        )?;
        Ok(lines)
    }
}