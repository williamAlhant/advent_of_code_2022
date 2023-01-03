use crate::days::internal_common::*;

pub fn day_22_part_1<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let input = get_whole_input_as_string(input)?;
    
    let ans = day_22_common(input, get_next_part_1)?;

    println!("Ans {}", ans);

    Ok(())
}

pub fn day_22_part_2<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let input = get_whole_input_as_string(input)?;
    
    let (grid_input, moves) = parse::parse(&input)?;
    let grid = Grid::new(grid_input);

    let faces = get_face_links(&grid);

    let mut current = Point::from_xy(0, 0);
    let mut current_dir = POINT_RIGHT;
    let mut content = grid.get_content_at_point(&current).unwrap();
    while content != PointContent::Open {
        current = current + POINT_RIGHT;
        content = grid.get_content_at_point(&current).unwrap();
    }
    // println!("{:?}", &current);

    for a_move in moves {
        // println!("{:?}", &a_move);
        match a_move {
            Move::Advance(advance_len) => {
                let mut advance_left = advance_len;
                while advance_left > 0 {

                    let (next, next_content, next_dir) = get_next_part_2(&current, &current_dir, &grid, &faces);

                    if next_content == PointContent::Wall {
                        break;
                    }
                    else if next_content == PointContent::Open {
                        current = next;
                        current_dir = next_dir;
                        advance_left -= 1;
                    }
                    // println!("{:?} adv_left {}", &current, &advance_left);
                }
            },
            Move::Turn(turn_dir) => {
                current_dir = current_dir.turn(turn_dir);
            }
        }
    }

    let final_col = current.x + 1;
    let final_row = current.y + 1;
    let direction_int;
    if current_dir.x == 1 {
        direction_int = 0;
    }
    else if current_dir.x == -1 {
        direction_int = 2;
    }
    else if current_dir.y == 1 {
        direction_int = 1;
    }
    else {
        direction_int = 3;
    }
    let ans = final_row * 1000 + final_col * 4 + direction_int;
    println!("Ans {}", ans);

    Ok(())
}

fn day_22_common<F>(input: String, get_next_fn: F) -> Result<i32>
where F: Fn(&Point, &Point, &Grid) -> (Point, PointContent, Point)
{
    let (grid_input, moves) = parse::parse(&input)?;
    let grid = Grid::new(grid_input);
    
    let mut current = Point::from_xy(0, 0);
    let mut current_dir = POINT_RIGHT;
    let mut content = grid.get_content_at_point(&current).unwrap();
    while content != PointContent::Open {
        current = current + POINT_RIGHT;
        content = grid.get_content_at_point(&current).unwrap();
    }
    // println!("{:?}", &current);

    for a_move in moves {
        // println!("{:?}", &a_move);
        match a_move {
            Move::Advance(advance_len) => {
                let mut advance_left = advance_len;
                while advance_left > 0 {

                    let (next, next_content, next_dir) = get_next_fn(&current, &current_dir, &grid);

                    if next_content == PointContent::Wall {
                        break;
                    }
                    else if next_content == PointContent::Open {
                        current = next;
                        current_dir = next_dir;
                        advance_left -= 1;
                    }
                    // println!("{:?} adv_left {}", &current, &advance_left);
                }
            },
            Move::Turn(turn_dir) => {
                current_dir = current_dir.turn(turn_dir);
            }
        }
    }

    let final_col = current.x + 1;
    let final_row = current.y + 1;
    let direction_int;
    if current_dir.x == 1 {
        direction_int = 0;
    }
    else if current_dir.x == -1 {
        direction_int = 2;
    }
    else if current_dir.y == 1 {
        direction_int = 1;
    }
    else {
        direction_int = 3;
    }
    let ans = final_row * 1000 + final_col * 4 + direction_int;
    Ok(ans)
}

fn get_next_part_1(current: &Point, dir: &Point, grid: &Grid) -> (Point, PointContent, Point)
{
    let mut next = current + dir;
    let mut next_content = grid.get_content_at_point(&next);
    if next_content.is_none() {
        if next.x == -1 {
            next.x += grid.size_x as i32;
        }
        else if next.y == -1 {
            next.y += grid.size_y as i32;
        }
        else if next.x == grid.size_x as i32 {
            next.x = 0;
        }
        else if next.y == grid.size_y as i32 {
            next.y = 0;
        }
        else {
            panic!("Unexpected case");
        }
        next_content = grid.get_content_at_point(&next);
        assert!(next_content.is_some());
    }
    let mut next_content = next_content.unwrap();

    while next_content == PointContent::Space {
        (next, next_content, _) = get_next_part_1(&next, dir, &grid);
    }

    (next, next_content, dir.clone())
}

fn get_next_part_2(current: &Point, dir: &Point, grid: &Grid, faces: &Vec<Option<Face>>) -> (Point, PointContent, Point)
{
    let cube_side_len = (grid.size_x.max(grid.size_y) / 4) as i32;
    let rot = |p: &Point| {
        if p.y == 0 {
            Point::from_xy(cube_side_len - 1, p.x)
        }
        else if p.y == cube_side_len - 1 {
            Point::from_xy(0, p.x)
        }
        else if p.x == 0 {
            Point::from_xy(cube_side_len - 1 - p.y, 0)
        }
        else if p.x == cube_side_len - 1 {
            Point::from_xy(cube_side_len - 1 - p.y, cube_side_len - 1)
        }
        else {
            panic!("Unexpected case");
        }
    };

    let next = current + dir;
    let next_content = grid.get_content_at_point(&next);
    match next_content {
        None | Some(PointContent::Space) => {
            let face_i_x = current.x / cube_side_len;
            let face_i_y = current.y / cube_side_len;
            // let grid_size_y_in_faces = grid.size_y as i32 / cube_side_len;
            let grid_size_x_in_faces = grid.size_x as i32 / cube_side_len;
            let face_id = (face_i_y * grid_size_x_in_faces + face_i_x) as usize;
            let face = faces[face_id].as_ref().unwrap();
            let mut p_rel = Point::from_xy(current.x - face_i_x * cube_side_len, current.y - face_i_y * cube_side_len);
            let face_link;
            if dir.x == 1 {
                face_link = &face.link_right;
                p_rel.x = 0;
            }
            else if dir.x == -1 {
                face_link = &face.link_left;
                p_rel.x = cube_side_len - 1;
            }
            else if dir.y == 1 {
                face_link = &face.link_down;
                p_rel.y = 0;
            }
            else {
                face_link = &face.link_up;
                p_rel.y = cube_side_len - 1;
            }
            let next_face_x = (face_link.id as i32 % grid_size_x_in_faces) * cube_side_len;
            let next_face_y = (face_link.id as i32 / grid_size_x_in_faces) * cube_side_len;
            
            let mut next_dir = dir.clone();
            for _i in 0..face_link.num_rot {
                p_rel = rot(&p_rel);
                next_dir = next_dir.turn(TurnDirection::Right);
            }

            let next = Point::from_xy(next_face_x, next_face_y) + &p_rel;
            // dbg!(face_id, next_face_x, next_face_y, &p_rel);
            let next_content = grid.get_content_at_point(&next).unwrap();
            return (next, next_content, next_dir)
        },
        Some(next_content) => (next, next_content, dir.clone())
    }
}

#[derive(Debug)]
struct Face {
    link_up: FaceLink,
    link_down: FaceLink,
    link_left: FaceLink,
    link_right: FaceLink,
}

#[derive(Debug, Default)]
struct FaceLink {
    id: usize,
    num_rot: u8
}

type Point = crate::days::points::Point2<i32>;
type Point3 = crate::days::points::Point3<i32>;
const POINT_RIGHT: Point = Point {x: 1, y: 0};
// const POINT_LEFT: Point = Point {x: -1, y: 0};
// const POINT_UP: Point = Point {x: 0, y: -1};
// const POINT_DOWN: Point = Point {x: 0, y: 1};

#[derive(Clone)]
struct Frame3 {
    x: Point3,
    y: Point3,
    z: Point3,
}

impl Default for Frame3 {
    fn default() -> Frame3
    {
        Frame3 {
            x: Point3 {x:1, y: 0, z: 0},
            y: Point3 {x:0, y: 1, z: 0},
            z: Point3 {x:0, y: 0, z: 1},
        }
    }
}

fn rotate_frame_up(frame: &Frame3) -> Frame3
{
    Frame3 {
        x: -frame.z.clone(),
        y: frame.y.clone(),
        z: frame.x.clone(),
    }
}

fn rotate_frame_down(frame: &Frame3) -> Frame3
{
    Frame3 {
        x: frame.z.clone(),
        y: frame.y.clone(),
        z: -frame.x.clone(),
    }
}

fn rotate_frame_left(frame: &Frame3) -> Frame3
{
    Frame3 {
        x: frame.x.clone(),
        y: frame.z.clone(),
        z: -frame.y.clone(),
    }
}

fn rotate_frame_right(frame: &Frame3) -> Frame3
{
    Frame3 {
        x: frame.x.clone(),
        y: -frame.z.clone(),
        z: frame.y.clone(),
    }
}

fn rotate_frame_clockwise(frame: &Frame3) -> Frame3
{
    Frame3 {
        x: -frame.y.clone(),
        y: frame.x.clone(),
        z: frame.z.clone(),
    }
}

fn get_num_rot_clockwise(from: &Frame3, to: &Frame3) -> u8
{
    assert!(from.z == to.z);
    if from.x == to.x {
        return 0;
    }
    let mut t = from.clone();
    for i in 1..=3 {
        t = rotate_frame_clockwise(&t);
        if t.x == to.x {
            return i;
        }
    }
    panic!("No pure clockwise rot found");
}

fn get_face_links(grid: &Grid) -> Vec<Option<Face>>
{
    let side_len = grid.size_x.max(grid.size_y) / 4;
    let mut faces: Vec<Option<Face>> = Vec::new();
    let grid_size_y_in_faces = (grid.size_y / side_len) as i32;
    let grid_size_x_in_faces = (grid.size_x / side_len) as i32;
    for i_y in 0..grid_size_y_in_faces {
        for i_x in 0..grid_size_x_in_faces {
            let content = grid.get_content_at_point(&Point::from_xy(i_x * side_len as i32, i_y * side_len as i32)).unwrap();
            if content == PointContent::Space {
                faces.push(None);
                continue;
            }
            let mut link_up = FaceLink::default();
            let mut link_down = FaceLink::default();
            let mut link_right = FaceLink::default();
            let mut link_left = FaceLink::default();
            let mut neigh_todo: Vec<(Frame3, i32, i32)> = vec![(Frame3::default(), i_x, i_y)];
            let mut visited: Vec<bool> = vec![false; (grid_size_y_in_faces * grid_size_x_in_faces) as usize];
            while !neigh_todo.is_empty() {
                let (frame, i_x, i_y) = neigh_todo.pop().unwrap();
                let face_id = (i_y * grid_size_x_in_faces + i_x) as usize;
                visited[face_id] = true;

                if frame.z == Point3::from_xyz(1, 0, 0) {
                    link_up = FaceLink {id: face_id, num_rot: get_num_rot_clockwise(&rotate_frame_up(&Frame3::default()), &frame)};
                }
                else if frame.z == Point3::from_xyz(-1, 0, 0) {
                    link_down = FaceLink {id: face_id, num_rot: get_num_rot_clockwise(&rotate_frame_down(&Frame3::default()), &frame)};
                }
                else if frame.z == Point3::from_xyz(0, 1, 0) {
                    link_right = FaceLink {id: face_id, num_rot: get_num_rot_clockwise(&rotate_frame_right(&Frame3::default()), &frame)};
                }
                else if frame.z == Point3::from_xyz(0, -1, 0) {
                    link_left = FaceLink {id: face_id, num_rot: get_num_rot_clockwise(&rotate_frame_left(&Frame3::default()), &frame)};
                }

                let has_neigh = |i_x, i_y| {
                    if !(0..grid_size_x_in_faces).contains(&i_x) || !(0..grid_size_y_in_faces).contains(&i_y) {
                        return false;
                    }
                    if visited[(i_y * grid_size_x_in_faces + i_x) as usize] {
                        return false;
                    }
                    let content = grid.get_content_at_point(&Point::from_xy(
                        i_x * side_len as i32, i_y * side_len as i32)).unwrap();
                    content != PointContent::Space
                };

                if has_neigh(i_x, i_y - 1) {
                    neigh_todo.push((rotate_frame_up(&frame), i_x, i_y - 1));
                }
                if has_neigh(i_x, i_y + 1) {
                    neigh_todo.push((rotate_frame_down(&frame), i_x, i_y + 1));
                }
                if has_neigh(i_x - 1, i_y) {
                    neigh_todo.push((rotate_frame_left(&frame), i_x - 1, i_y));
                }
                if has_neigh(i_x + 1, i_y) {
                    neigh_todo.push((rotate_frame_right(&frame), i_x + 1, i_y));
                }
            }
            faces.push(Some(Face {
                link_down,
                link_left,
                link_right,
                link_up
            }));
        }
    }
    faces
}

impl Point {
    fn turn(&self, direction: TurnDirection) -> Self
    {
        match direction {
            TurnDirection::Left => Self::from_xy(self.y, -self.x),
            TurnDirection::Right => Self::from_xy(-self.y, self.x),
        }
    }
}

struct Grid {
    data: Vec<PointContent>,
    size_x: usize,
    size_y: usize
}

impl Grid {
    fn new(input: &str) -> Self
    {
        let size_x = input.lines().map(|line| line.len()).max().unwrap();
        let size_y = input.lines().count();
        let mut data = vec![PointContent::Space; size_x * size_y];
        let mut offset = 0;
        for line in input.lines() {
            for (i, c) in line.chars().enumerate() {
                data[offset + i] = PointContent::from(c);
            }
            offset += size_x;
        }
        Self {
            data, size_x, size_y
        }
    }
}

impl WithDataType for Grid {
    type DataType = PointContent;
}

crate::impl_grid_2d_access_with_point!(Point, i32, Grid);

#[derive(Debug)]
enum Move {
    Turn(TurnDirection),
    Advance(i32)
}

#[derive(Debug)]
enum TurnDirection {
    Left,
    Right
}

#[derive(Clone, PartialEq, Eq)]
enum PointContent {
    Open,
    Wall,
    Space
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
            PointContent::Open => '.',
            PointContent::Wall => '#',
            PointContent::Space => ' ',
        }
    }
}

impl std::convert::From<char> for PointContent {
    fn from(c: char) -> Self
    {
        match c {
            '.' => PointContent::Open,
            '#' => PointContent::Wall,
            ' ' => PointContent::Space,
            _ => panic!("Unexpected char")
        }
    }
}


mod parse {
    use nom::Parser;
    use nom::bytes::complete::take_until;

    use crate::days::parse::*;
    use crate::days::parse::nom_goes_brrr::*;
    use super::*;

    pub(super) fn parse(input: &str) -> super::Result<(&str, Vec<Move>)>
    {
        let parse_turn_direction = alt((
            tag("L").map(|_| Move::Turn(TurnDirection::Left)),
            tag("R").map(|_| Move::Turn(TurnDirection::Right)),
        ));

        let parse_moves = many0(alt((
            parse_turn_direction,
            parse_int.map(|x| Move::Advance(x))
        )));

        let (_, ret) = make_verbose_error_message(input,
            tuple((
                take_until("\n\n"),
                preceded(tag("\n\n"), parse_moves)
            ))(input)
        )?;
        Ok(ret)
    }
}