use crate::days::internal_common::*;

pub fn day_20_part_1<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let input = get_whole_input_as_string(input)?;
    let numbers = parse::parse_and_collect(&input)?;

    let numbers = mix_numbers(&numbers, 1);
    for i in 0..numbers.len() {
        if numbers[i] == 0 {
            // dbg!(numbers[(i + 1000) % numbers.len()], numbers[(i + 2000) % numbers.len()], numbers[(i + 3000) % numbers.len()]);
            let sum =
                numbers[(i + 1000) % numbers.len()] +
                numbers[(i + 2000) % numbers.len()] +
                numbers[(i + 3000) % numbers.len()];
            println!("Sum {}", sum);
            return Ok(());
        }
    }

    Ok(())
}

pub fn day_20_part_2<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let input = get_whole_input_as_string(input)?;
    let mut numbers = parse::parse_and_collect(&input)?;

    for number in &mut numbers {
        *number *= 811589153;
    }

    let numbers = mix_numbers(&numbers, 10);
    for i in 0..numbers.len() {
        if numbers[i] == 0 {
            // dbg!(numbers[(i + 1000) % numbers.len()], numbers[(i + 2000) % numbers.len()], numbers[(i + 3000) % numbers.len()]);
            let sum =
                numbers[(i + 1000) % numbers.len()] +
                numbers[(i + 2000) % numbers.len()] +
                numbers[(i + 3000) % numbers.len()];
            println!("Sum {}", sum);
            return Ok(());
        }
    }

    Ok(())
}

fn mix_numbers(numbers: &Vec<i64>, num_rounds: usize) -> Vec<i64>
{
    let mut positions: Vec<i64> = (0..(numbers.len() as i64)).collect();

    for _i in 0..num_rounds {
        mix_numbers_once(numbers, &mut positions);
    }

    get_numbers_in_positions(numbers, &positions)
}

fn mix_numbers_once(numbers: &Vec<i64>, positions: &mut Vec<i64>)
{
    let numbers_len = numbers.len() as i64;

    for i in 0..numbers.len() {
        let position_before = positions[i];
        let mut position_after = position_before + numbers[i] % (numbers_len - 1);
        if position_after >= numbers_len {
            position_after = position_after - numbers_len + 1;
        }
        else if position_after < 0 {
            position_after = position_after + numbers_len - 1;
        }
        // println!("{} will move from pos {} to {}", numbers[i], position_before, position_after);
        let offset_in_positions = position_after - position_before;
        let displacement;
        if offset_in_positions > 0 {
            displacement = -1;
        }
        else {
            displacement = 1;
        }

        for pos in positions.iter_mut() {
            let displaced =
                (offset_in_positions > 0 &&
                    (*pos > position_before && *pos <= position_after)
                ) ||
                (offset_in_positions < 0 &&
                    (*pos < position_before && *pos >= position_after)
                )
                ;
            if displaced {
                *pos += displacement;
                assert!(*pos < numbers_len && *pos >= 0);
            }
        }
        positions[i] = position_after;

        // println!("{:?}", get_numbers_in_positions(&numbers, &positions));
    }
}

fn get_numbers_in_positions(numbers: &Vec<i64>, positions: &Vec<i64>) -> Vec<i64>
{
    let mut check = positions.clone();
    check.sort();
    // dbg!(&check);
    for i in 0..check.len() {
        assert_eq!(check[i], i as i64);
    }
    let mut ret = vec![-42; numbers.len()];
    for (original_position, new_position) in positions.iter().enumerate() {
        ret[*new_position as usize] = numbers[original_position];
    }
    ret
}

mod parse {
    use crate::days::parse::*;
    use crate::days::parse::nom_goes_brrr::*;

    pub(super) fn parse_and_collect(input: &str) -> super::Result<Vec<i64>>
    {
        let (_, ret) = make_verbose_error_message(input,
            separated_list0(
                newline, parse_int
            )(input)
        )?;
        Ok(ret)
    }
}
