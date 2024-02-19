#![allow(unused)]

use std::{rc::Rc, sync::Arc};

#[derive(Clone)]
struct Item {
    s: [i32; 1024],
    a: i32,
    b: i32,
}

#[inline(always)]
fn raw(x: &Item) -> Item{
    (*x).clone()
}

#[inline(always)]
fn boxed(x: &Box<Item>) -> Box<Item> {
    (*x).clone()
}

#[inline(always)]
fn rced(x: &Rc<Item>) -> Rc<Item> {
    (*x).clone()
}

#[inline(always)]
fn arced(x: &Arc<Item>) -> Arc<Item> {
    (*x).clone()
}

#[cfg(test)]
mod tests {
    use tracing::info;
    use tracing_subscriber::fmt;

    use super::*;

    #[test]
    fn test_ptr() {
        tracing_subscriber::fmt::init();

        let x = Item { a: 1, b: 2, s: [128; 1024] };
        let y = raw(&x);
        // print pointer address of x.s and y.s
        info!("raw:   x.s: {:p} | y.s: {:p}", &x.s, &y.s);
        info!("raw:     x: {:p} |   y: {:p}", &x, &y);
    
        let x = Box::new(x);
        let y = boxed(&x);
        // print pointer address of x.s and y.s
        info!("boxed: x.s: {:p} | y.s: {:p}", &x.s, &y.s);
        info!("boxed:   x: {:p} |   y: {:p}", &x, &y);

        let ptr_data = unsafe { std::mem::transmute::<Box<Item>, *const Item>(x) };
        info!("boxed ptr_data is {:p}", ptr_data);
    }
}