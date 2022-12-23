use crate::days::internal_common::*;

pub fn day_18_part_1<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let input = get_whole_input_as_string(input)?;
    let points = parse::parse_and_collect(&input)?;
    
    let (size_x, size_y, size_z) = compute_grid_size(&points);

    let mut grid = Grid {
        data: vec![0; size_x * size_y * size_z],
        size_x,
        size_y,
        size_z,
    };

    for point in &points {
        let content = grid.get_content_at_point(point).unwrap();
        grid.put_content_at_point(point, content + 6);
        for point_neigh in point.get_cube_neighbors() {
            let content = grid.get_content_at_point(&point_neigh);
            if content.is_none() {
                continue;
            }
            let content = content.unwrap();
            grid.put_content_at_point(&point_neigh, content - 1);
        }
    }

    let mut surface: usize = 0;
    for &x in &grid.data {
        if x > 0 {
            surface += x as usize;
        }
    }
    println!("Surface: {}", surface);

    Ok(())
}

// values for flood fill
const UNINIT: i8 = -1;
const LAVA: i8 = 1;
const WATER: i8 = 2;

pub fn day_18_part_2<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let input = get_whole_input_as_string(input)?;
    let points = parse::parse_and_collect(&input)?;
    
    let (size_x, size_y, size_z) = compute_grid_size(&points);

    let mut flood_fill_grid = Grid {
        data: vec![UNINIT; size_x * size_y * size_z],
        size_x,
        size_y,
        size_z,
    };

    for point in &points {
        flood_fill_grid.put_content_at_point(point, LAVA);
    }

    //flood_fill_recurs(&Point::from_xyz(0, 0, 0), &mut flood_fill_grid);
    flood_fill(&mut flood_fill_grid);

    let mut surface_grid = Grid {
        data: vec![0; size_x * size_y * size_z],
        size_x,
        size_y,
        size_z,
    };
    for point in &points {
        let content = surface_grid.get_content_at_point(point).unwrap();
        surface_grid.put_content_at_point(point, content + 6);
        for point_neigh in point.get_cube_neighbors() {
            let content = surface_grid.get_content_at_point(&point_neigh);
            if content.is_none() {
                continue;
            }
            let content = content.unwrap();
            let flood_fill_value = flood_fill_grid.get_content_at_point(&point_neigh).unwrap();
            if flood_fill_value != WATER {
                surface_grid.put_content_at_point(&point_neigh, content - 1);
            }
        }
    }

    let mut surface: i64 = 0;
    for &x in &surface_grid.data {
        surface += x as i64;
    }
    println!("Surface: {}", surface);

    Ok(())
}

// fn flood_fill_recurs(point: &Point, grid: &mut Grid)
// {
//     for neigh in point.get_cube_neighbors() {
//         let neigh_content = grid.get_content_at_point(&neigh);
//         if neigh_content.is_none() {
//             continue;
//         }
//         let neigh_content = neigh_content.unwrap();
//         if neigh_content == UNINIT {
//             grid.put_content_at_point(&neigh, WATER);
//             flood_fill_recurs(&neigh, grid);
//         }
//     }
// }

fn flood_fill(grid: &mut Grid)
{
    let mut to_visit: Vec<Point> = vec![Point::from_xyz(0, 0, 0)];
    while !to_visit.is_empty() {
        let point = to_visit.pop().unwrap();
        for neigh in point.get_cube_neighbors() {
            let neigh_content = grid.get_content_at_point(&neigh);
            if neigh_content.is_none() {
                continue;
            }
            let neigh_content = neigh_content.unwrap();
            if neigh_content == UNINIT {
                grid.put_content_at_point(&neigh, WATER);
                to_visit.push(neigh);
            }
        }
    }
}

fn compute_grid_size(lava_points: &Vec<Point>) -> (usize, usize, usize)
{
    let mut max_coord = Point::from_xyz(0, 0, 0);
    for point in lava_points {
        if point.x > max_coord.x {
            max_coord.x = point.x;
        }
        if point.y > max_coord.y {
            max_coord.y = point.y;
        }
        if point.z > max_coord.z {
            max_coord.z = point.z;
        }
    }

    // + 1 is enough for containing all the points, but +2 could be useful for floodfill
    let size_x = max_coord.x as usize + 2;
    let size_y = max_coord.y as usize + 2;
    let size_z = max_coord.z as usize + 2;

    (size_x, size_y, size_z)
}

type Point = crate::days::points::Point3<i32>;

impl Point {

    fn get_cube_neighbors(&self) -> [Self; 6] {
        [
            Point::from_xyz(self.x + 1, self.y    , self.z    ),
            Point::from_xyz(self.x - 1, self.y    , self.z    ),
            Point::from_xyz(self.x    , self.y + 1, self.z    ),
            Point::from_xyz(self.x    , self.y - 1, self.z    ),
            Point::from_xyz(self.x    , self.y    , self.z + 1),
            Point::from_xyz(self.x    , self.y    , self.z - 1),
        ]
    }

}

struct Grid {
    data: Vec<i8>,
    size_x: usize,
    size_y: usize,
    size_z: usize,
}

trait Grid3DAccessWithPoint {
    type Content;
    fn get_content_at_point(&self, point: &Point) -> Option<Self::Content>;
    fn put_content_at_point(&mut self, point: &Point, content: Self::Content);
}

impl Grid3DAccessWithPoint for Grid {
    type Content = i8;

    fn get_content_at_point(&self, point: &Point) -> Option<Self::Content>
    {
        if !(0..(self.size_x as i32)).contains(&point.x) ||
            !(0..(self.size_y as i32)).contains(&point.y) ||
            !(0..(self.size_z as i32)).contains(&point.z) {
            return None;
        }

        let (x, y, z) = ((point.x as usize), (point.y as usize), (point.z as usize));
        let id = z * (self.size_x * self.size_y) + y * self.size_x + x;
        Some(self.data[id])
    }

    fn put_content_at_point(&mut self, point: &Point, content: Self::Content)
    {
        let (x, y, z) = ((point.x as usize), (point.y as usize), (point.z as usize));
        let id = z * (self.size_x * self.size_y) + y * self.size_x + x;
        self.data[id] = content;
    }
}

mod parse {
    use crate::days::parse::*;
    use crate::days::parse::nom_goes_brrr::*;
    use nom::Parser;
    use super::Point;

    pub(super) fn parse_and_collect(input: &str) -> super::Result<Vec<Point>>
    {
        let parse_point = tuple((
            parse_int,
            preceded(tag(","), parse_int),
            preceded(tag(","), parse_int),
        )).map(|(x,y,z)| Point::from_xyz(x, y, z));

        let (_, ret) = make_verbose_error_message(input,
            many0(
                terminated(parse_point, newline)
            )(input)
        )?;
        Ok(ret)
    }
}