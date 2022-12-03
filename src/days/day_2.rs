use crate::days::internal_common::*;

#[derive(Clone, Copy, Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors
}

#[derive(Clone, Copy, Debug)]
enum Outcome {
    Win,
    Loss,
    Draw
}

fn get_shape_value(shape: Shape) -> i32 {
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}

fn get_round_outcome(my_shape: Shape, his_shape: Shape) -> Outcome {
    use Shape::*;
    match my_shape {
        Rock => match his_shape {
            Rock => Outcome::Draw,
            Paper => Outcome::Loss,
            Scissors => Outcome::Win,
        },
        Paper => match his_shape {
            Rock => Outcome::Win,
            Paper => Outcome::Draw,
            Scissors => Outcome::Loss,
        },
        Scissors => match his_shape {
            Rock => Outcome::Loss,
            Paper => Outcome::Win,
            Scissors => Outcome::Draw,
        },
    }
}

fn get_round_score(my_shape: Shape, his_shape: Shape) -> i32 {
    let outcome = get_round_outcome(my_shape, his_shape);
    let outcome_score = match outcome {
        Outcome::Loss => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6,
    };
    outcome_score + get_shape_value(my_shape)
}

fn parse_first_column(c: char) -> Result<Shape> {
    match c {
        'A' => Ok(Shape::Rock),
        'B' => Ok(Shape::Paper),
        'C' => Ok(Shape::Scissors),
        _ => Err(Error::new_token(0, 1))
    }
}

fn parse_second_column(c: char) -> Result<Shape> {
    match c {
        'X' => Ok(Shape::Rock),
        'Y' => Ok(Shape::Paper),
        'Z' => Ok(Shape::Scissors),
        _ => Err(Error::new_token(2, 1))
    }
}

fn get_shape_for_desired_outcome(his_shape: Shape, desired_outcome: Outcome) -> Shape {
    use Shape::*;
    use Outcome::*;
    match his_shape {
        Rock => match desired_outcome {
            Loss => Scissors,
            Draw => Rock,
            Win => Paper
        },
        Paper => match desired_outcome {
            Loss => Rock,
            Draw => Paper,
            Win => Scissors
        },
        Scissors => match desired_outcome {
            Loss => Paper,
            Draw => Scissors,
            Win => Rock
        },
    }
}

fn parse_second_column_part_2(c: char, his_shape: Shape) -> Result<Shape> {
    match c {
        'X' => Ok(get_shape_for_desired_outcome(his_shape, Outcome::Loss)),
        'Y' => Ok(get_shape_for_desired_outcome(his_shape, Outcome::Draw)),
        'Z' => Ok(get_shape_for_desired_outcome(his_shape, Outcome::Win)),
        _ => Err(Error::new_token(2, 1))
    }
}

fn do_each_line<F>(content: String, mut func: F) -> Result<()> 
where F: FnMut(&str) -> Result<()> {
    for (i, line) in content.lines().enumerate() {
        let line_num = i + 1;
        func(line).map_err(
            |e| match e {
                Error::ParsingToken(token) => Error::new_parsing_with_token(line, line_num, token),
                _ => e
            })?;
    }
    Ok(())
}

pub fn day_2_part_1<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let content = get_whole_input_as_string(input)?;

    let mut total_score = 0;

    do_each_line(content, |line| {
        let line: Vec<char> = line.chars().collect();
        if line.len() != 3 {
            return Err(Error::new_token(0, line.len()));
        }
        let his_shape = parse_first_column(line[0])?;
        let my_shape = parse_second_column(line[2])?;
        total_score += get_round_score(my_shape, his_shape);
        Ok(())
    })?;

    println!("Total score: {}", total_score);

    Ok(())
}

pub fn day_2_part_2<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let content = get_whole_input_as_string(input)?;

    let mut total_score = 0;

    do_each_line(content, |line| {
        let line: Vec<char> = line.chars().collect();
        if line.len() != 3 {
            return Err(Error::new_token(0, line.len()));
        }
        let his_shape = parse_first_column(line[0])?;
        let my_shape = parse_second_column_part_2(line[2], his_shape)?;
        total_score += get_round_score(my_shape, his_shape);
        Ok(())
    })?;

    println!("Total score: {}", total_score);

    Ok(())
}