use crate::days::internal_common::*;

fn get_nums<Input>(input: &mut Input) -> Result<Vec<i32>>
where Input: Read
{
    let mut content = String::new();
    input.read_to_string(&mut content).map_err(|_| Error::NotUtf8)?;
    let mut nums: Vec<i32> = Vec::new();
    for (line, content) in content.lines().enumerate() {
        let num = content.parse::<i32>().map_err(|_| Error::new_parsing(content, line))?;
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