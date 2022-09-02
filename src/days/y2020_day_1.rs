use crate::days::internal_common::*;

fn get_nums<Input>(input: &mut Input) -> Result<Vec<i32>>
where Input: Read
{
    let mut content = String::new();
    input.read_to_string(&mut content).or(Err(Error::NotUtf8))?;
    let mut nums: Vec<i32> = Vec::new();
    for line in content.lines() {
        let num = line.parse::<i32>().or(Err(Error::Parsing {content: String::from(line)}))?;
        nums.push(num);
    }
    Ok(nums)
}

pub fn y2020_day_1_part_1<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let nums = get_nums(input)?;
    
    for i in 0..nums.len() {
        for j in i..nums.len() {
            if nums[i] + nums[j] == 2020 {
                println!("Answer: {}", nums[i] * nums[j]);
                return Ok(());
            }
        }
    }

    Err(Error::NoSolution)
}

pub fn y2020_day_1_part_2<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let nums = get_nums(input)?;
    
    for i in 0..nums.len() {
        for j in i..nums.len() {
            for k in j..nums.len() {
                if nums[i] + nums[j] + nums[k] == 2020 {
                    println!("Answer: {}", nums[i] * nums[j] * nums[k]);
                    return Ok(());
                }
            }
        }
    }

    Err(Error::NoSolution)
}