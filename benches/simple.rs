use std::{rc::Rc, sync::Arc};

#[allow(unused)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const SIZE: usize = 1024;
static data: Item = Item { a: 0, b: 1, s: [128; SIZE] };

pub fn simple(c: &mut Criterion) {
    let mut group = c.benchmark_group("simple");
    let s = [128; SIZE];
    let raw_item = Item { a: 0, b: 1, s};
    let boxed_item = Box::new(Item { a: 0, b: 1, s});
    let rced_item = Rc::new(Item { a: 0, b: 1, s });
    let arced_item = Arc::new(Item { a: 0, b: 1, s});
    group.bench_function("raw", |b| {
        b.iter(|| raw(black_box(&raw_item)));
    });
    group.bench_function("box", |b| {
        b.iter(|| boxed(black_box(&boxed_item)));
    });
    // group.bench_function("rc", |b| {
    //     b.iter(|| rced(black_box(&rced_item)));
    // });
    // group.bench_function("arc", |b| {
    //     b.iter(|| arced(black_box(&arced_item)));
    // });
    group.finish();
}

#[allow(unused)]
#[derive(Clone)]
struct Item {
    a: i32,
    b: i32,
    s: [i32; SIZE],
}

#[inline(always)]
fn raw(x: &Item) -> Item{
    black_box((*x).clone())
}

#[inline(always)]
fn boxed(x: &Box<Item>) -> Box<Item> {
    black_box((*x).clone())
}

#[inline(always)]
fn rced(x: &Rc<Item>) -> Rc<Item> {
    (*x).clone()
}

#[inline(always)]
fn arced(x: &Arc<Item>) -> Arc<Item> {
    (*x).clone()
}

criterion_group!(benches, simple);
criterion_main!(benches);

