use crate::days::internal_common::*;

pub fn day_25_part_1<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let mut sum = 0;
    do_for_each_line(input, |line| {
        sum += parse_snafu(line);
        Ok(())
    })?;

    println!("Result {}", to_snafu(sum));

    Ok(())
}

pub fn day_25_part_2<Input>(_input: &mut Input) -> Result<()>
where Input: Read
{
    Ok(())
}

fn to_snafu(mut n: usize) -> String
{
    let mut ret = String::new();
    while n > 0 {
        let m = (n + 2) % 5;
        let ch = match m {
            0 => '=',
            1 => '-',
            2 => '0',
            3 => '1',
            4 => '2',
            _ => panic!("Mod 5 should return 0-4")
        };
        ret.insert(0, ch);
        n = (n + 2) / 5;
    }
    ret
}

fn parse_snafu(s: &str) -> usize
{
    let mut ret: i64 = 0;
    let mut pow = 1;
    for ch in s.chars().rev() {
        let m = match ch {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => panic!("Unexpected snafu char")
        };
        ret += m * pow;
        pow *= 5;
    }
    ret as usize
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_convert_to_snafu() {
        assert_eq!(to_snafu(39), "2=-".to_string());
        assert_eq!(to_snafu(42), "2=2".to_string());

        assert_eq!(to_snafu(10), "20".to_string());
        assert_eq!(to_snafu(15), "1=0".to_string());
        assert_eq!(to_snafu(20), "1-0".to_string());
        assert_eq!(to_snafu(2022), "1=11-2".to_string());
    }

    #[test]
    fn test_parse_snafu() {
        assert_eq!(parse_snafu("2=2"), 42);

        assert_eq!(parse_snafu("1=-0-2"), 1747);
        assert_eq!(parse_snafu("12111"), 906);

    }
}