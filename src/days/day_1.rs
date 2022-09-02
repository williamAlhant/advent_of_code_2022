use crate::days::internal_common::*;

pub fn day_1_part_1<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let mut content = String::new();
    input.read_to_string(&mut content).or(Err(Error::NotUtf8))?;
    dbg!(content);
    Ok(())
}