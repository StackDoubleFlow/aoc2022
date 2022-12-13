use std::{fs, collections::VecDeque};

use evalexpr::{HashMapContext, ContextWithMutableVariables, Value, eval_with_context_mut, Context};

struct Monkey<'a> {
    operation: &'a str,
    divisible: i64,
    throw_to: (usize, usize),
    inspect_count: usize,
}

fn main() {
    let input = fs::read_to_string("./input2.txt").unwrap();

    let mut monkeys = Vec::new();
    let mut monkey_items = Vec::new();

    let mut lines = input.lines();
    while let Some(_) = lines.next() {
        let starting_items: VecDeque<i64> = lines.next().unwrap()["  Starting items: ".len()..].split(", ").map(|s| s.parse().unwrap()).collect();
        let operation = &lines.next().unwrap()["  Operation: ".len()..];
        let divisible = lines.next().unwrap()["  Test: divisible by ".len()..].parse().unwrap();
        let throw_true = lines.next().unwrap()["    If true: throw to monkey ".len()..].parse().unwrap();
        let throw_false = lines.next().unwrap()["    If false: throw to monkey ".len()..].parse().unwrap();

        monkey_items.push(starting_items);
        monkeys.push(Monkey {
            operation,
            divisible,
            throw_to: (throw_true, throw_false),
            inspect_count: 0,
        });
        lines.next();
    }

    let modulo: i64 = monkeys.iter().map(|m| m.divisible).product();
    for round in 0..10000 {
        if round % 1000 == 0 {
            println!("Round {}", round + 1);
            for (i, monkey_items) in monkey_items.iter().enumerate() {
                println!("Monkey {i}: {monkey_items:?}");
            }
            for (i, monkey) in monkeys.iter().enumerate() {
                println!("Monkey {i} inspected items {} times.", monkey.inspect_count);
            }
        }
        for (idx, monkey) in monkeys.iter_mut().enumerate() {
            while let Some(old) = monkey_items[idx].pop_front() {
                monkey.inspect_count += 1;
                let mut ctx = HashMapContext::new();
                ctx.set_value("old".to_string(), Value::Int(old)).unwrap();
                eval_with_context_mut(monkey.operation, &mut ctx).unwrap();
                let new = ctx.get_value("new").unwrap().as_int().unwrap();
                if new % monkey.divisible == 0 {
                    // println!("{old} -> {new}, {idx} to {}", monkey.throw_to.0);
                    monkey_items[monkey.throw_to.0].push_back(new % modulo);
                } else {
                    // println!("{old} -> {new}, {idx} to {}", monkey.throw_to.1);
                    monkey_items[monkey.throw_to.1].push_back(new % modulo);
                }
            }
        }
    }

    println!("Final results: ");
    for (i, monkey_items) in monkey_items.iter().enumerate() {
        println!("Monkey {i}: {monkey_items:?}");
    }
    for (i, monkey) in monkeys.iter().enumerate() {
        println!("Monkey {i} inspected items {} times.", monkey.inspect_count);
    }
    monkeys.sort_unstable_by_key(|m| m.inspect_count);
    monkeys.reverse();
    let monkey_business: usize = monkeys[0..2].iter().map(|m| m.inspect_count).product();
    dbg!(monkey_business);
}
