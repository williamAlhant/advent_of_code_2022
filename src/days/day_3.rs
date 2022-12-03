use crate::days::internal_common::*;

fn byte_to_item_priority(c: u8) -> usize {
    if (b'a'..=b'z').contains(&c) {
        (c - b'a' + 1) as usize
    } else if (b'A'..=b'Z').contains(&c) {
        (c - b'A' + 27) as usize
    }
    else {
        c as usize
    }
}

trait RucksackAnalyzer {
    fn parse_line(&mut self, line: &str) -> Result<()>;
    fn get_sum(&self) -> usize;
}

#[derive(Default)]
struct RucksackAnalyzerPart1 {
    sum: usize
}

impl RucksackAnalyzer for RucksackAnalyzerPart1 {
    fn parse_line(&mut self, line: &str) -> Result<()> {
        if line.len() % 2 != 0 {
            return Err(Error::new_token(0, line.len()));
        }

        let mut items_count = vec![0; 53];

        for i in 0..line.len() {
            let priority = byte_to_item_priority(line.as_bytes()[i]);
            if priority >= items_count.len() {
                return Err(Error::new_token(i, 1));
            }

            if i < line.len() / 2 {
                items_count[priority] += 1;
            }
            else if items_count[priority] != 0 {
                self.sum += priority;
                break;
            }
        }

        Ok(())
    }

    fn get_sum(&self) -> usize {
        self.sum
    }
}

struct RucksackAnalyzerPart2 {
    sum: usize,
    line: usize,
    items_flags: Vec<u8>
}

impl RucksackAnalyzerPart2 {
    fn new() -> Self {
        Self {
            sum: 0,
            line: 0,
            items_flags: vec![0; 53]
        }
    }
}

impl RucksackAnalyzer for RucksackAnalyzerPart2 {
    fn parse_line(&mut self, line: &str) -> Result<()> {

        for char_pos in 0..line.len() {
            let priority = byte_to_item_priority(line.as_bytes()[char_pos]);
            if priority >= self.items_flags.len() {
                return Err(Error::new_token(char_pos, 1));
            }
            self.items_flags[priority] = self.items_flags[priority] | (1 << (self.line % 3));
        }

        self.line += 1;
        if self.line % 3 == 0 {

            let position = match self.items_flags.iter().position(|flags| *flags == 0b111u8) {
                Some(position) => position,
                None => return Err(Error::NoSolution)
            };

            self.items_flags.fill(0);
            self.sum += position;
        }

        Ok(())
    }

    fn get_sum(&self) -> usize {
        self.sum
    }
}

pub fn day_3_part_1<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let content = get_whole_input_as_string(input)?;
    let mut rucksack_analyzer = RucksackAnalyzerPart1::default();

    day_3(content, &mut rucksack_analyzer)?;

    Ok(())
}

pub fn day_3_part_2<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let content = get_whole_input_as_string(input)?;
    let mut rucksack_analyzer = RucksackAnalyzerPart2::new();

    day_3(content, &mut rucksack_analyzer)?;

    Ok(())
}

fn day_3<T>(input_content: String, rucksack_analyzer: &mut T) -> Result<()> 
where T: RucksackAnalyzer {
    for (line_idx, line) in input_content.lines().enumerate() {
        rucksack_analyzer.parse_line(line).map_err(
            |e| match e {
                Error::ParsingToken(token) => Error::new_parsing_with_token(line, line_idx + 1, token),
                _ => e
            })?;
    }

    println!("Sum is {}", rucksack_analyzer.get_sum());

    Ok(())
}