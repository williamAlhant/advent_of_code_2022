use crate::days::internal_common::*;

pub fn day_11_part_1<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let content = get_whole_input_as_string(input)?;

    let mut monkeys: Vec<Monkey> = Vec::new();
    
    parse::parse_and_do_for_each_monkey(content.as_str(), |monkey| {
        monkeys.push(monkey);
        Ok(())
    })?;

    let mut num_inspections: Vec<usize> = vec![0; monkeys.len()];

    for _ in 0..20 {
        do_round(&mut monkeys, &mut num_inspections, |x| x / 3);
    }

    num_inspections.sort();
    
    let ans = num_inspections[num_inspections.len() - 1] * num_inspections[num_inspections.len() - 2];
    println!("Ans {}", ans);

    Ok(())
}

fn do_round<F>(monkeys: &mut Vec<Monkey>, num_inspections: &mut Vec<usize>, reduce_level_func: F)
where F: Fn(usize) -> usize
{
    for monkey_idx in 0..monkeys.len() {
        for item_idx in 0..monkeys[monkey_idx].items.len() {
            let old_level = monkeys[monkey_idx].items[item_idx];
            let new_level = match monkeys[monkey_idx].op {
                Operation::Add(x) => old_level + x,
                Operation::Mul(x) => old_level * x,
                Operation::Square => old_level * old_level,
            };
            let new_level = reduce_level_func(new_level);
            let test = &monkeys[monkey_idx].test;
            let target = match new_level % test.divisor {
                0 => test.target_if_true,
                _ => test.target_if_false,
            };
            monkeys[target].items.push(new_level);
        }
        num_inspections[monkey_idx] += monkeys[monkey_idx].items.len();
        monkeys[monkey_idx].items.clear();
    }
}

pub fn day_11_part_2<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let content = get_whole_input_as_string(input)?;

    let mut monkeys: Vec<Monkey> = Vec::new();
    
    parse::parse_and_do_for_each_monkey(content.as_str(), |monkey| {
        monkeys.push(monkey);
        Ok(())
    })?;

    let mut num_inspections: Vec<usize> = vec![0; monkeys.len()];

    // The idea is to decrease x while keeping the same modulo for all divisors d
    // We use the fact that x = x - n*d (mod d) for any n
    // So x = x - d0*d1...*dn preserves the modulo for all d
    // Doing a "% n" operation is like subtracting n multiple times, so we can
    // do x = x % d0*d1...*dn
    let magic_value = monkeys.iter().fold(1, |acc, monkey| acc * monkey.test.divisor);
    for _ in 0..10000 {
        do_round(&mut monkeys, &mut num_inspections, |x| x % magic_value);
    }

    num_inspections.sort();
    
    let ans = num_inspections[num_inspections.len() - 1] * num_inspections[num_inspections.len() - 2];
    println!("Ans {}", ans);

    Ok(())
}

#[derive(Debug)]
enum Operation {
    Add(usize),
    Mul(usize),
    Square
}

#[derive(Debug, Default)]
struct Test {
    divisor: usize,
    target_if_true: usize,
    target_if_false: usize
}

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    op: Operation,
    test: Test
}

mod parse {
    use crate::days::parse::*;
    use crate::days::parse::nom_goes_brrr::*;

    pub(super) fn parse_and_do_for_each_monkey<F>(input: &str, mut func: F) -> super::Result<()>
    where F: FnMut(super::Monkey) -> super::Result<()>
    {
        let mut i = input;
        while i.starts_with("Monkey") {

            let parse_int_list = many0(terminated(parse_int::<usize, _>, opt(tag(", "))));
            let parse_operation_args = context("operation_args",
                alt((
                    map_res(
                        separated_pair(anychar, tag(" "), parse_int::<usize, _>),
                        |(c, i)| match c {
                            '+' => Ok(super::Operation::Add(i)),
                            '*' => Ok(super::Operation::Mul(i)),
                            _ => Err(())
                        }),
                    map_res(
                        tag("* old"),
                        |_| Ok::<_, ()>(super::Operation::Square)
                    )
                ))
            );

            let mut parse_monkey_idx = delimited(tag("Monkey "), parse_int::<usize, _>, tag(":\n"));
            let mut parse_starting_items = delimited(tag("  Starting items: "), parse_int_list, newline);
            let mut parse_operation = delimited(tag("  Operation: new = old "), parse_operation_args, newline);
            let parse_test = tuple((
                delimited(tag("  Test: divisible by "), parse_int::<usize, _>, newline),
                delimited(tag("    If true: throw to monkey "), parse_int::<usize, _>, newline),
                delimited(tag("    If false: throw to monkey "), parse_int::<usize, _>, newline),
            ));
            let mut parse_test = map_res(parse_test,
                |(divisor, target_if_true, target_if_false)| {
                    Ok::<_, ()>(super::Test {divisor, target_if_true, target_if_false})
                }
            );

            let items;
            let op;
            let test;
            (i, _) = make_verbose_error_message(input, parse_monkey_idx(i))?;
            (i, items) = make_verbose_error_message(input, parse_starting_items(i))?;
            (i, op) = make_verbose_error_message(input, parse_operation(i))?;
            (i, test) = make_verbose_error_message(input, parse_test(i))?;

            let monkey = super::Monkey { items, op, test };
            func(monkey)?;

            (i, _) = make_verbose_error_message(input, opt(newline)(i))?;
        }
        Ok(())
    }
}