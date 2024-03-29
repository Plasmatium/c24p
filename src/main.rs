// calc 24 point game in Rust

use std::{
    collections::HashSet,
    hash::Hash,
    sync::Arc,
};

use num_rational::Rational32;
use rayon::prelude::*;
use tracing::{debug, info};

fn main() {
    tracing_subscriber::fmt::init();
    let original_deck = vec![1,1,2,3,5,8];
    let deck = original_deck
        .iter()
        .map(|&n| Expr::Number(Rational32::from(n).into()))
        .collect::<Vec<_>>();
    info!("start on deck: {}", sprint_deck(&deck));
    let target = Rational32::from(24);
    let results = build_trees(&deck, target, true);
    info!("founded {} results", results.len());
    for r in results {
        info!("{r}");
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
enum Op {
    Add(Expr, Expr),
    Sub(Expr, Expr),
    Mul(Expr, Expr),
    Div(Expr, Expr),
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

// // this is to ensure that the hash values of additions and multiplications that
// // satisfy the commutative law are independent of the order of the operands.
// impl std::cmp::PartialEq for Op {
//     fn eq(&self, other: &Self) -> bool {
//         self.calc() == other.calc()
//     }
// }

impl From<Op> for Expr {
    fn from(op: Op) -> Self {
        Expr::Op(op.into())
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
enum Expr {
    Number(Arc<Rational32>),
    Op(Arc<Op>),
}

impl Expr {
    fn calc(&self) -> Rational32 {
        match self {
            Expr::Number(n) => **n,
            Expr::Op(op) => op.calc(),
        }
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Number(n) => write!(f, "{}", n),
            Expr::Op(op) => write!(f, "({})", op),
        }
    }
}

fn sprint_deck(deck: &[Expr]) -> String {
    deck.iter()
        .map(|item| item.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn build_all_possible(a: &Expr, b: &Expr) -> Vec<Expr> {
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

fn build_trees(deck: &[Expr], target: Rational32, top_level: bool) -> HashSet<Expr> {
    if deck.len() == 1 {
        return deck.iter().cloned().collect();
    }
    // use bitset to construct selection
    let max_bitset: u128 = 1 << (deck.len() - 1);
    let ret_iter = (1..max_bitset).into_par_iter().flat_map(|bitset| {
        let (left, right) = select(deck, bitset);
        let left_trees = build_trees(&left, target, false);
        let right_trees = build_trees(&right, target, false);
        left_trees
            .par_iter()
            .flat_map(|left_tree| {
                right_trees
                    .par_iter()
                    .flat_map(move |right_tree| build_all_possible(&left_tree, &right_tree))
            })
            .collect::<HashSet<_>>()
    });
    if top_level {
        ret_iter.filter(|item| item.calc() == target).collect()
    } else {
        ret_iter.collect()
    }
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
