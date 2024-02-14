// calc 24 point game in Rust

use std::rc::Rc;

use num_rational::Rational32;
use tracing::{debug, info};

fn main() {
    tracing_subscriber::fmt::init();

    let original_deck = vec![3,3,7,7];
    let deck = original_deck
        .iter()
        .map(|&n| Rational32::from(n))
        .collect();
    let target = Rational32::from(24);
    let result = eval(deck, target);
    if let Some(solution) = result {
        info!("deck: {original_deck:?}, target: {target}, solution: {} = {}", solution, target);
    } else {
        info!("no solution found")
    }
}

enum Op {
    Add(Item, Item),
    Sub(Item, Item),
    Mul(Item, Item),
    Div(Item, Item),
}

impl Op {
    fn calc(&self) -> Rational32 {
        match self {
            Op::Add(a, b) => a.calc() + b.calc(),
            Op::Sub(a, b) => a.calc() - b.calc(),
            Op::Mul(a, b) => a.calc() * b.calc(),
            Op::Div(a, b) => a.calc() / b.calc(),
        }
    }
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Op::Add(a, b) => write!(f, "{} + {}", a, b),
            Op::Sub(a, b) => write!(f, "{} - {}", a, b),
            Op::Mul(a, b) => write!(f, "{} * {}", a, b),
            Op::Div(a, b) => write!(f, "{} / {}", a, b),
        };
    }
}

enum Item {
    Number(Rational32),
    Op(Rc<Op>),
}

impl Item {
    fn calc(&self) -> Rational32 {
        match self {
            Item::Number(n) => *n,
            Item::Op(op) => op.calc(),
        }
    }
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Item::Number(n) => write!(f, "{}", n),
            Item::Op(op) => write!(f, "({})", op),
        }
    }
}

impl From<Op> for Item {
    fn from(op: Op) -> Self {
        Item::Op(op.into())
    }
}

fn eval(deck: Vec<Rational32>, target: Rational32) -> Option<Item> {
    debug!("deck: {}, target: {target:?}", deck_str(&deck));
    if deck.len() > 4 {
        panic!(
            "too many numbers in deck, shuld be 4 at most, got {}",
            deck.len()
        );
    }

    if deck.len() == 1 {
        if deck[0] == target {
            // last one matched, return the number
            return Some(Item::Number(target));
        }
        return None;
    }

    // pick one number from deck and try all possible ops
    // for each op, try to eval the remain deck with the target
    for i in 0..deck.len() {
        let mut deck = deck.clone();
        let picked_number = deck.remove(i);
        let picked = Item::Number(picked_number);
        let result = calc_possible_remain_ops(picked, target, deck);
        if result.is_some() {
            return result;
        }
    }

    // pick two numbers from deck and try all possible ops
    // for each op, try to eval the remain deck with the target
    // only consider deck.len() == 4 case
    if deck.len() == 4 {
        for [part1, part2] in split_deck(deck) {
            for op in get_possible_ops(part1) {
                let result = calc_possible_remain_ops(op.into(), 24.into(), part2.to_vec());
                if result.is_some() {
                    return result;
                }
            }
        }
    }

    None
}

fn calc_possible_remain_ops(
    picked: Item,
    original_target: Rational32,
    remain: Vec<Rational32>,
) -> Option<Item> {
    debug!("picked: {picked}, original_target: {original_target}, remain: {}", deck_str(&remain));
    let picked_number = picked.calc();

    // consider all possible targets: n / target, n * target, n - target, n + target
    // picked + original_target
    let target = picked_number + original_target;
    let remain_op = eval(remain.clone(), target);
    if let Some(remain_op) = remain_op {
        return Some(Item::Op(Rc::new(Op::Sub(remain_op, picked))));
    }

    // picked - original_target
    let target = picked_number - original_target;
    let remain_op = eval(remain.clone(), target);
    if let Some(remain_op) = remain_op {
        return Some(Item::Op(Rc::new(Op::Add(remain_op, picked))));
    }

    // original_target - picked
    let target = original_target - picked_number;
    let remain_op = eval(remain.clone(), target);
    if let Some(remain_op) = remain_op {
        return Some(Item::Op(Rc::new(Op::Add(remain_op, picked))));
    }

    // picked * original_target
    let target = picked_number * original_target;
    if target == 0.into() {
        return None;
    }
    let remain_op = eval(remain.clone(), target);
    if let Some(remain_op) = remain_op {
        return Some(Item::Op(Rc::new(Op::Div(remain_op, picked))));
    }

    // picked / original_target
    let target = picked_number / original_target;
    let remain_op = eval(remain.clone(), target);
    if let Some(remain_op) = remain_op {
        return Some(Item::Op(Rc::new(Op::Div(picked, remain_op))));
    }

    // original_target / picked
    let target = original_target / picked_number;
    let remain_op = eval(remain.clone(), target);
    if let Some(remain_op) = remain_op {
        return Some(Item::Op(Rc::new(Op::Mul(remain_op, picked))));
    }

    None
}

// split a deck with lenth == 4, into two sub-decks with length == 2, returns an iterator
// for example, original deck is [1,2,3,4], then yields:
// ([1,2], [3,4])
// ([1,3], [2,4])
// ([1,4], [2,3])
fn split_deck(deck: Vec<Rational32>) -> [[[Rational32; 2]; 2]; 3] {
    [
        [[deck[0], deck[1]], [deck[2], deck[3]]],
        [[deck[0], deck[2]], [deck[1], deck[3]]],
        [[deck[0], deck[3]], [deck[1], deck[2]]],
    ]
}

// calc all possible operations for two numbers
// for example, a and b:
// a + b
// a - b
// b - a
// a * b
// a / b
// b / a
fn get_possible_ops(parts: [Rational32; 2]) -> [Op; 6] {
    let [a, b] = parts;
    [
        Op::Add(Item::Number(a), Item::Number(b)),
        Op::Sub(Item::Number(a), Item::Number(b)),
        Op::Sub(Item::Number(b), Item::Number(a)),
        Op::Mul(Item::Number(a), Item::Number(b)),
        Op::Div(Item::Number(a), Item::Number(b)),
        Op::Div(Item::Number(b), Item::Number(a)),
    ]
}

fn deck_str(deck: &[Rational32]) -> String {
    let deck_str = deck.iter().map(|n| n.to_string()).collect::<Vec<_>>();
    deck_str.join(", ")
}