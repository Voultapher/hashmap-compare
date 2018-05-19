#![feature(test)]

extern crate test;

use std::collections::HashMap;

pub fn fill_linear_n(n: i32) -> HashMap<i32, i32> {
    let mut hm = HashMap::new();
    for i in 0..n {
        hm.insert(i, i);
    }
    hm
}

pub fn fill_linear_n_lookup_one(n: i32) {
    fill_linear_n(n).get(&(n / 2)).unwrap();
}

pub fn fill_linear_n_lookup_all(n: i32) {
    let hm = fill_linear_n(n);
    for i in 0..n {
        hm.get(&i).unwrap();
    }
}

// missing traversal and other key and value types

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    const SMALL_N: i32 = 10;
    const MEDIUM_N: i32 = 100;
    const LARGE_N: i32 = 1_000;
    const BIG_N: i32 = 10_000;

    #[test]
    fn fill_linear_n_works() {
        let target_n = SMALL_N / 2;
        assert_eq!(&target_n, fill_linear_n(SMALL_N).get(&target_n).unwrap());
    }

    #[bench]
    fn small_lookup_one(b: &mut Bencher) {
        b.iter(|| fill_linear_n_lookup_one(SMALL_N));
    }

    #[bench]
    fn small_lookup_all(b: &mut Bencher) {
        b.iter(|| fill_linear_n_lookup_all(SMALL_N));
    }

    #[bench]
    fn medium_lookup_one(b: &mut Bencher) {
        b.iter(|| fill_linear_n_lookup_one(MEDIUM_N));
    }

    #[bench]
    fn medium_lookup_all(b: &mut Bencher) {
        b.iter(|| fill_linear_n_lookup_all(MEDIUM_N));
    }

    #[bench]
    fn large_lookup_one(b: &mut Bencher) {
        b.iter(|| fill_linear_n_lookup_one(LARGE_N));
    }

    #[bench]
    fn large_lookup_all(b: &mut Bencher) {
        b.iter(|| fill_linear_n_lookup_all(LARGE_N));
    }

    #[bench]
    fn big_lookup_one(b: &mut Bencher) {
        b.iter(|| fill_linear_n_lookup_one(BIG_N));
    }

    #[bench]
    fn big_lookup_all(b: &mut Bencher) {
        b.iter(|| fill_linear_n_lookup_all(BIG_N));
    }
}

fn main() {
    let mut book_reviews = HashMap::new();
    book_reviews.insert(5, "value");
    println!("Hello, world! {}", book_reviews.get(&5).unwrap());
}
