// calc 24 point game in Rust

mod pointer_test;

use rayon::prelude::*;

use std::{
    collections::HashSet,
    hash::{DefaultHasher, Hash, Hasher}, sync::Arc,
};

use num_rational::Rational32;
use tracing::{debug, info};

fn main() {
    tracing_subscriber::fmt::init();
    let original_deck = [4,5,6,7];
    let deck = original_deck
        .iter()
        .map(|&n| Item::Number(Rational32::from(n)))
        .collect::<Vec<_>>();
    info!("start on deck: {}", sprint_deck(&deck));
    let target = Rational32::from(24);
    let mut results = build_trees(&deck);
    results.retain(|item| item.calc() == target);
    info!("founded {} results", results.len());
    for r in results {
        info!("{r}");
    }
}

#[derive(Clone, Eq)]
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
        match self {
            Op::Add(a, b) => write!(f, "{} + {}", a, b),
            Op::Sub(a, b) => write!(f, "{} - {}", a, b),
            Op::Mul(a, b) => write!(f, "{} * {}", a, b),
            Op::Div(a, b) => write!(f, "{} / {}", a, b),
        }
    }
}

// this is to ensure that the hash values of additions and multiplications that
// satisfy the commutative law are independent of the order of the operands.
impl std::cmp::PartialEq for Op {
    fn eq(&self, other: &Self) -> bool {
        self.calc() == other.calc()
    }
}

// this is to ensure that the hash values of additions and multiplications
// that satisfy the commutative law are independent of the order of the operands.
impl Hash for Op {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let (op, a, b) = match self {
            Op::Add(a, b) => ("+", a, b),
            Op::Sub(a, b) => ("-", a, b),
            Op::Mul(a, b) => ("*", a, b),
            Op::Div(a, b) => ("/", a, b),
        };
        if ["-", "/"].contains(&op) {
            a.hash(state);
            b.hash(state);
        } else {
            let mut hasher1 = DefaultHasher::new();
            let mut hasher2 = DefaultHasher::new();
            a.hash(&mut hasher1);
            b.hash(&mut hasher2);
            let hash1 = hasher1.finish();
            let hash2 = hasher2.finish();
            let combined_hash = hash1.wrapping_add(hash2);
            combined_hash.hash(state);
        }
    }
}

impl From<Op> for Item {
    fn from(op: Op) -> Self {
        Item::Op(Arc::new(op))
    }
}

#[derive(Clone, Eq, Hash)]
enum Item {
    Number(Rational32),
    Op(Arc<Op>),
}

impl Item {
    fn calc(&self) -> Rational32 {
        match self {
            Item::Number(n) => *n,
            Item::Op(op) => op.calc(),
        }
    }
}

impl std::cmp::PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Item::Number(a), Item::Number(b)) => a == b,
            (Item::Op(a), Item::Op(b)) => a == b,
            _ => false,
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

fn sprint_deck(deck: &[Item]) -> String {
    deck.iter()
        .map(|item| item.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn build_all_possible(a: &Item, b: &Item) -> Vec<Item> {
    debug!("building all possible for {} and {}", a, b);
    let mut ret = Vec::with_capacity(6);
    ret.push(Op::Add(a.clone(), b.clone()).into());
    ret.push(Op::Sub(a.clone(), b.clone()).into());
    ret.push(Op::Sub(b.clone(), a.clone()).into());
    ret.push(Op::Mul(a.clone(), b.clone()).into());
    if b.calc() != 0.into() {
        ret.push(Op::Div(a.clone(), b.clone()).into());
    }
    if a.calc() != 0.into() {
        ret.push(Op::Div(b.clone(), a.clone()).into());
    }

    ret
}

fn build_trees(deck: &[Item]) -> HashSet<Item> {
    if deck.len() == 1 {
        return deck.iter().cloned().collect();
    }
    // use bitset to construct selection
    let max_bitset: u128 = 1 << (deck.len() - 1);
    (1..max_bitset).into_par_iter()
        .flat_map(|bitset| {
            let (left, right) = select(deck, bitset);
            let left_trees = build_trees(&left);
            let right_trees = build_trees(&right);
            left_trees
                .par_iter()
                .flat_map(|left_tree| {
                    right_trees
                        .par_iter()
                        .flat_map(move |right_tree| build_all_possible(&left_tree, &right_tree))
                })
                .collect::<HashSet<_>>()
        })
        .collect()
}

fn select<T: Clone>(slice: &[T], bitset: u128) -> (Vec<T>, Vec<T>) {
    if bitset > 1 << slice.len() {
        panic!("bitset out of range");
    }
    let mut left = Vec::with_capacity(bitset.count_ones() as usize);
    let mut right = Vec::with_capacity(bitset.count_zeros() as usize);
    for i in 0..slice.len() {
        if bitset & (1 << i) != 0 {
            left.push(slice[i].clone());
        } else {
            right.push(slice[i].clone());
        }
    }
    (left, right)
}

fn test() {
    let par_iter = (1..10).into_par_iter();
    let iter = (1..10).into_iter().collect::<Vec<_>>().iter();
}