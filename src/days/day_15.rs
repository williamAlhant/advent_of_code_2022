use crate::days::internal_common::*;
use std::ops::Range;
use std::collections::BTreeSet;

pub fn day_15_part_1<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let input = get_whole_input_as_string(input)?;
    let points = parse::parse_and_collect(&input)?;
    let y_scan = 2_000_000;
    
    let mut ranges: Vec<Range<i32>> = Vec::new();
    for sensor_beacon in &points {
        if let Some(intersec) = get_range_intersect(y_scan, sensor_beacon) {
            ranges.push(intersec);
        }
    }
    combine_overlapping_ranges(&mut ranges);
    
    let sum: usize = ranges.iter().map(|range| range.len()).sum();
    let unique_beacons = get_unique_beacons(&points);
    let sum: usize = sum - unique_beacons.iter().filter(|beacon| beacon.y == y_scan).count();
    println!("Answer is {}", sum);

    Ok(())
}

pub fn day_15_part_2<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let input = get_whole_input_as_string(input)?;
    let points = parse::parse_and_collect(&input)?;

    for y_scan in 0..=4_000_000 {
    
        let mut ranges: Vec<Range<i32>> = Vec::new();
        for sensor_beacon in &points {
            if let Some(intersec) = get_range_intersect(y_scan, sensor_beacon) {
                ranges.push(intersec);
            }
        }
        combine_overlapping_ranges(&mut ranges);
        
        if ranges.len() > 1 {
            let x = ranges[0].end;
            let signal = x as usize * 4_000_000 + y_scan as usize;
            println!("Signal is {}", signal);
            return Ok(());
        }
    }

    Err(Error::NoSolution)
}

fn get_unique_beacons(points: &Vec<(Point, Point)>) -> BTreeSet<Point>
{
    let mut unique_beacons = BTreeSet::new();
    for (_, beacon) in points {
        unique_beacons.insert(beacon.clone());
    }
    unique_beacons
}

fn get_range_intersect(y: i32, sensor_beacon: &(Point, Point)) -> Option<Range<i32>>
{
    let (sensor, beacon) = sensor_beacon;
    let d = sensor.distance(&beacon);
    if (y - sensor.y).abs() > d {
        return None;
    }
    let half_range_len = d - (y - sensor.y).abs();
    Some((sensor.x - half_range_len)..(sensor.x + 1 + half_range_len))
}

fn combine_overlapping_ranges(combined_ranges: &mut Vec<Range<i32>>)
{
    combined_ranges.sort_by(|a, b| a.start.cmp(&b.start));

    let mut i = 0;

    while i + 1 < combined_ranges.len() {
        let mut j = i;
        while j + 1 < combined_ranges.len() && combined_ranges[j + 1].start <= combined_ranges[i].end {
            j += 1;
        }
        let max_end = combined_ranges[i..=j].iter().max_by(|a, b| a.end.cmp(&b.end)).unwrap().end;
        combined_ranges[i] = combined_ranges[i].start..max_end;
        combined_ranges.drain((i + 1)..=j);

        if j - i == 0 {
            i += 1;
        }
    }
}

// fn range_intersection(a: &Range<i32>, b: &Range<i32>) -> Option<Range<i32>>
// {
//     if a.start > b.start {
//         return range_intersection(b, a);
//     }

//     // we have a.start < b.start

//     if a.end <= b.start {
//         return None;
//     }

//     // we have a.end > b.start

//     Some(b.start..b.end.min(a.end))
// }

type Point = crate::days::points::Point2<i32>;

impl Point {
    fn distance(&self, other: &Self) -> i32
    {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

mod parse {
    use crate::days::parse::*;
    use crate::days::parse::nom_goes_brrr::*;
    use super::Point;

    fn parse_point<'a, E>(input: &'a str) -> IResult<&'a str, Point, E>
    where E: ParseError<&'a str> + FromExternalError<&'a str, ()>
    {
        map_res(
            separated_pair(preceded(tag("x="), parse_int), tag(", y="), parse_int),
            |(x, y)| Ok::<_, ()>(Point { x, y })
        )(input)
    }

    pub(super) fn parse_and_collect(input: &str) -> super::Result<Vec<(Point, Point)>>
    {
        let parse_sensor_beacon_pair = pair(
            preceded(tag("Sensor at "), parse_point),
            preceded(tag(": closest beacon is at "), parse_point)
        );

        let (_, sensor_beacon_pairs) = make_verbose_error_message(input,
            many0(
                terminated(parse_sensor_beacon_pair, newline)
            )(input)
        )?;
        Ok(sensor_beacon_pairs)
    }
}