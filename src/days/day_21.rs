use crate::days::internal_common::*;
use std::collections::HashMap;

pub fn day_21_part_1<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let input = get_whole_input_as_string(input)?;
    let monkeys = parse::parse_and_collect(&input)?;
    
    let monkeys: HashMap<String, Monkey> = HashMap::from_iter(monkeys.into_iter().map(|x| (x.name.clone(), x)));

    let root = monkeys.get("root").unwrap();
    let res = compute_recurs(root, &monkeys);

    println!("Result {}", res);

    Ok(())
}

pub fn day_21_part_2<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let input = get_whole_input_as_string(input)?;
    let monkeys = parse::parse_and_collect(&input)?;
    
    let monkeys: HashMap<String, Monkey> = HashMap::from_iter(monkeys.into_iter().map(|x| (x.name.clone(), x)));

    let root = monkeys.get("root").unwrap();

    // Assume x=humn is used only once, only on one side of the equation
    // So we only have to apply the reverse operations on the other side, until we reach x

    let (a_name, b_name) = match &root.expr {
        Expr::Lit(_) => panic!("Root should have an equation"),
        Expr::Op(op) => (&op.a, &op.b)
    };
    let a = monkeys.get(a_name).unwrap();
    let b = monkeys.get(b_name).unwrap();
    assert!(contains_humn_recurs(a, &monkeys));
    assert!(!contains_humn_recurs(b, &monkeys));
    
    let b = compute_recurs(b, &monkeys);
    let x = solve_recurs(a, &monkeys, b);

    println!("x=humn= {}", x);

    Ok(())
}

fn solve_recurs(of_x: &Monkey, monkeys: &HashMap<String, Monkey>, acc: i64) -> i64
{
    if of_x.name == "humn" {
        return acc;
    }
    match &of_x.expr {
        Expr::Lit(_) => panic!("Should be dead code"),
        Expr::Op(op) => {
            let mut a = monkeys.get(&op.a).unwrap();
            let mut b = monkeys.get(&op.b).unwrap();
            let mut reverse = false;
            if !contains_humn_recurs(a, &monkeys) {
                (a, b) = (b, a);
                reverse = true;
            }
            let b = compute_recurs(b, monkeys);
            let acc = match (&op.op_type, reverse) {
                (OpType::Add, _) => acc - b,
                (OpType::Mul, _) => acc / b,
                (OpType::Sub, false) => acc + b,
                (OpType::Div, false) => acc * b,
                (OpType::Sub, true) => b - acc,
                (OpType::Div, true) => panic!("Not sure this is solvable"),
            };
            solve_recurs(a, monkeys, acc)
        }
    }
}

fn contains_humn_recurs(target: &Monkey, monkeys: &HashMap<String, Monkey>) -> bool
{
    if target.name == "humn" {
        return true;
    }
    match &target.expr {
        Expr::Lit(_) => false,
        Expr::Op(op) => {
            let a = monkeys.get(&op.a).unwrap();
            let b = monkeys.get(&op.b).unwrap();
            let a = contains_humn_recurs(a, monkeys);
            let b = contains_humn_recurs(b, monkeys);
            a || b
        }
    }
}

fn compute_recurs(target: &Monkey, monkeys: &HashMap<String, Monkey>) -> i64
{
    match &target.expr {
        Expr::Lit(x) => *x,
        Expr::Op(op) => {
            let a = monkeys.get(&op.a).unwrap();
            let b = monkeys.get(&op.b).unwrap();
            let a = compute_recurs(a, monkeys);
            let b = compute_recurs(b, monkeys);
            match op.op_type {
                OpType::Add => a + b,
                OpType::Sub => a - b,
                OpType::Mul => a * b,
                OpType::Div => a / b,
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    name: String,
    expr: Expr
}

#[derive(Debug, Clone)]
struct Op {
    a: String,
    b: String,
    op_type: OpType
}

#[derive(Debug, Clone)]
enum OpType {
    Add,
    Sub,
    Mul,
    Div
}

#[derive(Debug, Clone)]
enum Expr {
    Lit(i64),
    Op(Op)
}

mod parse {
    use nom::Parser;

    use crate::days::parse::*;
    use crate::days::parse::nom_goes_brrr::*;
    use super::{Monkey, Op, OpType, Expr};

    pub(super) fn parse_and_collect(input: &str) -> super::Result<Vec<Monkey>>
    {
        let parse_op = alt((
            separated_pair(alpha1::<&str, _>, tag(" + "), alpha1).map(|(a, b)| (a, b, OpType::Add)),
            separated_pair(alpha1, tag(" - "), alpha1).map(|(a, b)| (a, b, OpType::Sub)),
            separated_pair(alpha1, tag(" * "), alpha1).map(|(a, b)| (a, b, OpType::Mul)),
            separated_pair(alpha1, tag(" / "), alpha1).map(|(a, b)| (a, b, OpType::Div)),
        )).map(|(a, b, op_type)| Op {a: a.to_string(), b: b.to_string(), op_type});

        let parse_monkey = terminated(alpha1, tag(": ")).and(
            alt((
                parse_int.map(|x| Expr::Lit(x)),
                parse_op.map(|op| Expr::Op(op))
            ))
        ).map(|(name, expr)| Monkey { name: name.to_string(), expr });

        let (_, ret) = make_verbose_error_message(input,
            separated_list0(
                newline, parse_monkey
            )(input)
        )?;
        Ok(ret)
    }
}