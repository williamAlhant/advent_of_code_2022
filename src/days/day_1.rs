use crate::days::internal_common::*;

struct HighestNValues {
    v: Vec<i32>
}

impl HighestNValues {
    fn new(size: usize) -> Self {
        Self {
            v: vec![0; size]
        }
    }

    fn update(&mut self, candidate: i32) {
        let mut index_of_smallest: usize = 0;
        let mut smallest = i32::MAX;
        let mut candidate_goes_in = false;
        for i in 0..self.v.len() {
            if self.v[i] < smallest {
                smallest = self.v[i];
                index_of_smallest = i;
            }
            if candidate > self.v[i] {
                candidate_goes_in = true;
            }
        }
        if candidate_goes_in {
            self.v[index_of_smallest] = candidate;
        }
    }

    fn sum(&self) -> i32 {
        self.v.iter().sum()
    }
}

pub fn day_1_common(content: String, num_highest: usize) -> Result<i32>
{
    let mut lines = content.lines();
    let mut highest_sums = HighestNValues::new(num_highest);
    let mut current_sum = 0;
    let mut line_num = 1;
    while let Some(line) = lines.next() {
        match line.len() {
            0 => {
                highest_sums.update(current_sum);
                current_sum = 0;
            }
            _ => {
                let item = line.parse::<i32>().map_err(|_| Error::new_parsing(line, line_num))?;
                current_sum += item;
            }
        }
        line_num += 1;
    }
    if current_sum != 0 {
        highest_sums.update(current_sum);
    }
    Ok(highest_sums.sum())
}

pub fn day_1_part_1<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let mut content = String::new();
    input.read_to_string(&mut content).map_err(|_| Error::NotUtf8)?;

    println!("Max is {}", day_1_common(content, 1)?);

    Ok(())
}

pub fn day_1_part_2<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let mut content = String::new();
    input.read_to_string(&mut content).map_err(|_| Error::NotUtf8)?;

    println!("Sum of highest sums is {}", day_1_common(content, 3)?);

    Ok(())
}