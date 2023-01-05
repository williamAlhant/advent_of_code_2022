use crate::days::internal_common::*;
use std::collections::BTreeSet;

pub fn day_24_part_1<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let input = get_whole_input_as_string(input)?;
    let grid = RawGrid::new(&input);
    let grid = Grid::new(grid);

    grid.solve_dfs();

    Ok(())
}

pub fn day_24_part_2<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let input = get_whole_input_as_string(input)?;
    let grid = RawGrid::new(&input);
    let grid = Grid::new(grid);

    grid.solve_dfs_part_2();

    Ok(())
}

type Point = crate::days::points::Point2<i32>;

struct RawGrid {
    data: Vec<PointContent>,
    size_x: usize,
    size_y: usize,
}

struct Grid {
    data: Vec<Option<PointInfo>>,
    size_x: usize,
    size_y: usize,
    start: Point,
    end: Point,
}

impl WithDataType for Grid {
    type DataType = Option<PointInfo>;
}

crate::impl_grid_2d_access_with_point!(Point, i32, Grid);

struct PointInfo {
    wind_pattern_x: Vec<bool>,
    wind_pattern_y: Vec<bool>,
}


#[derive(Clone, PartialEq, Eq)]
enum PointContent {
    Wall,
    Open,
    Wind(Direction),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Top,
    Bottom,
    Left,
    Right
}

impl RawGrid {
    fn new(input: &str) -> Self
    {
        let size_x = input.lines().map(|line| line.len()).max().unwrap();
        let size_y = input.lines().count();
        let mut data = vec![PointContent::Open; size_x * size_y];
        let mut offset = 0;
        for line in input.lines() {
            assert!(!line.is_empty());
            for (i, c) in line.chars().enumerate() {
                data[offset + i] = PointContent::from(c);
            }
            offset += size_x;
        }
        Self {
            data, size_x, size_y
        }
    }

    fn calc_patterns(&self, x_fixed: usize, y_fixed: usize) -> PointInfo
    {
        let size_x_between_walls = self.size_x - 2;
        let size_y_between_walls = self.size_y - 2;

        let mut wind_pattern_x = vec![false; size_x_between_walls];
        for x in 1..(self.size_x - 1) {
            let id = y_fixed * self.size_x + x;
            match &self.data[id] {
                PointContent::Wind(Direction::Left) => {
                    if x == x_fixed {
                        wind_pattern_x[0] = true;
                    }
                    else if x < x_fixed {
                        wind_pattern_x[size_x_between_walls - (x_fixed - x)] = true;
                    }
                    else {
                        wind_pattern_x[x - x_fixed] = true;
                    }
                },
                PointContent::Wind(Direction::Right) => {
                    if x == x_fixed {
                        wind_pattern_x[0] = true;
                    }
                    else if x < x_fixed {
                        wind_pattern_x[x_fixed - x] = true;
                    }
                    else {
                        wind_pattern_x[size_x_between_walls - (x - x_fixed)] = true;
                    }
                },
                _ => (),
            };
        }

        let mut wind_pattern_y = vec![false; size_y_between_walls];
        for y in 1..(self.size_y - 1) {
            let id = y * self.size_x + x_fixed;
            match &self.data[id] {
                PointContent::Wind(Direction::Top) => {
                    if y == y_fixed {
                        wind_pattern_y[0] = true;
                    }
                    else if y < y_fixed {
                        wind_pattern_y[size_y_between_walls - (y_fixed - y)] = true;
                    }
                    else {
                        wind_pattern_y[y - y_fixed] = true;
                    }
                },
                PointContent::Wind(Direction::Bottom) => {
                    if y == y_fixed {
                        wind_pattern_y[0] = true;
                    }
                    else if y < y_fixed {
                        wind_pattern_y[y_fixed - y] = true;
                    }
                    else {
                        wind_pattern_y[size_y_between_walls - (y - y_fixed)] = true;
                    }
                },
                _ => (),
            };
        }

        PointInfo { wind_pattern_x, wind_pattern_y }
    }
}

impl Grid {
    fn new(raw: RawGrid) -> Self
    {
        let mut start = None;
        let mut end = None;
        let mut data = Vec::new();
        for y in 0..raw.size_y {
            for x in 0..raw.size_x {
                let id = y * raw.size_x + x;
                let content = raw.data[id].clone();
                let is_start = y == 0 && content == PointContent::Open;
                let is_end = y == (raw.size_y - 1) && content == PointContent::Open;
                if is_start {
                    start = Some(Point::from_xy(x as i32, y as i32));
                }
                else if is_end {
                    end = Some(Point::from_xy(x as i32, y as i32));
                }

                if content != PointContent::Wall && !is_start && !is_end {
                    data.push(Some(raw.calc_patterns(x, y)));
                }
                else {
                    data.push(None);
                }
            }
        }
        let start = start.unwrap();
        let end = end.unwrap();
        Self {
            data,
            size_x: raw.size_x,
            size_y: raw.size_y,
            start,
            end,
        }
    }

    fn solve_dfs(&self)
    {
        let mut points_todo: BTreeSet<Point> = BTreeSet::new();
        points_todo.insert(self.start.clone());
        let mut time = 0;
        while !points_todo.is_empty() {
            let mut points_todo_next: BTreeSet<Point> = BTreeSet::new();
            for p in &points_todo {
                if p == &self.end {
                    println!("Reached end at time {}", time);
                    return;
                }
                self.solve_dfs_add_todo_next(time, p, &mut points_todo_next);
            }
            points_todo = points_todo_next;
            time += 1;
        }
    }

    fn solve_dfs_part_2(&self)
    {
        let mut points_todo: BTreeSet<Point> = BTreeSet::new();
        points_todo.insert(self.start.clone());
        let mut time = 0;
        let mut trip = 0;
        while !points_todo.is_empty() {
            let mut points_todo_next: BTreeSet<Point> = BTreeSet::new();
            if (trip == 0 || trip == 2) && points_todo.contains(&self.end) {
                println!("Reached end at time {}", time);
                if trip == 2 {
                    return;
                }
                points_todo_next.insert(self.end.clone());
                trip += 1;
            }
            else if trip == 1 && points_todo.contains(&self.start) {
                println!("Reached start at time {}", time);
                points_todo_next.insert(self.start.clone());
                trip += 1;
            }
            else {
                for p in &points_todo {
                    self.solve_dfs_add_todo_next(time, p, &mut points_todo_next);
                }
            }
            points_todo = points_todo_next;
            time += 1;
        }
    }

    fn solve_dfs_add_todo_next(&self, time: usize, p: &Point, todo_next: &mut BTreeSet<Point>)
    {
        let neighs = [
            p + Point::from_xy(0, -1),
            p + Point::from_xy(-1, 0),
            p.clone(),
            p + Point::from_xy(1, 0),
            p + Point::from_xy(0, 1),
        ];
        for neigh in &neighs {
            let content = self.get_ref_content_at_point(neigh);
            if content.is_none() {
                continue;
            }
            let content = content.unwrap();
            if content.is_none() {
                if neigh == &self.start || neigh == &self.end {
                    todo_next.insert(neigh.clone());
                }
                continue;
            }
            let info = content.as_ref().unwrap();
            let no_wind = !info.wind_pattern_x[(time + 1) % info.wind_pattern_x.len()] &&
                !info.wind_pattern_y[(time + 1) % info.wind_pattern_y.len()];
            if no_wind {
                todo_next.insert(neigh.clone());
            }
        }
    }
}

impl std::convert::From<char> for PointContent {
    fn from(c: char) -> Self
    {
        match c {
            '.' => PointContent::Open,
            '#' => PointContent::Wall,
            '>' => PointContent::Wind(Direction::Right),
            '<' => PointContent::Wind(Direction::Left),
            '^' => PointContent::Wind(Direction::Top),
            'v' => PointContent::Wind(Direction::Bottom),
            _ => panic!("Unexpected char")
        }
    }
}
