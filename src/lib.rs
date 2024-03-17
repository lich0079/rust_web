#![feature(test)]

extern crate test;

pub fn add_two(mut a: i32) -> i32 {
    for _ in 0..1000000000 {
        a += 2;
    }
    a + 2
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;
    use test::Bencher;
    use std::sync::{Arc};
    use std::collections::HashMap;
    use std::rc::Rc;

    fn nude() {
        let mut map = HashMap::new();
        for i in 0..1000000 {
            map.insert(i, String::from("xxx {i}"));
        }
    }

    fn arc_works() {
        let mut map = Arc::new(Mutex::new(HashMap::new()));
        let mut real_map = map.lock().unwrap();
        for i in 0..1000000 {
            real_map.insert(i, String::from("xxx {i}"));
        }
    }

    fn rc_works() {
        let mut map = Rc::new(Mutex::new(HashMap::new()));
        let mut real_map = map.lock().unwrap();
        for i in 0..1000000 {
            real_map.insert(i, String::from("xxx {i}"));
        }
    }

    #[bench]
    fn map(b: &mut Bencher) {
        b.iter(|| nude());
    }

    #[bench]
    fn arc(b: &mut Bencher) {
        b.iter(|| arc_works());
    }

    #[bench]
    fn rc(b: &mut Bencher) {
        b.iter(|| rc_works());
    }
}