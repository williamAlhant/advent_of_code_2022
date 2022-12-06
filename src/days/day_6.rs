use std::collections::HashSet;
use crate::days::internal_common::*;

fn find_sequence(content_chars: Vec<char>, num_diff_required: usize) -> Result<usize>
{
    let mut slice_set: HashSet<char> = HashSet::new();
    for (pos, new_char) in content_chars.iter().enumerate() {

        if pos >= num_diff_required {
            if !content_chars[(pos - num_diff_required + 1)..pos].contains(&content_chars[pos - num_diff_required]) {
                slice_set.remove(&content_chars[pos - num_diff_required]);
            }
        }
        slice_set.insert(*new_char);

        // println!("char {} set len {}", new_char, slice_set.len());
        if slice_set.len() == num_diff_required {
            return Ok(pos + 1);
        }
    }
    Err(Error::NoSolution)
}

pub fn day_6_part_1<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let content = get_whole_input_as_string(input)?;
    let content_chars: Vec<char> = content.chars().collect();
    let solution = find_sequence(content_chars, 4)?;
    println!("Arrived at {}", solution);
    Ok(())
}

pub fn day_6_part_2<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let content = get_whole_input_as_string(input)?;
    let content_chars: Vec<char> = content.chars().collect();
    let solution = find_sequence(content_chars, 14)?;
    println!("Arrived at {}", solution);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1_examples() {
        let find_sequence = |s: &str| find_sequence(s.chars().collect(), 4).unwrap();
        assert_eq!(find_sequence("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(find_sequence("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(find_sequence("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(find_sequence("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_part_2_examples() {
        let find_sequence = |s: &str| find_sequence(s.chars().collect(), 14).unwrap();
        assert_eq!(find_sequence("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(find_sequence("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(find_sequence("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(find_sequence("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
    }
}