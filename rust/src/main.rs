#![feature(test)]

extern crate fnv;
extern crate murmur3;
extern crate rand;
extern crate test;

use std::collections::HashMap;

use rand::{Rng, thread_rng};

#[cfg(feature = "use_fnv")] use fnv::FnvBuildHasher;

#[cfg(feature = "use_murmur")] use std::hash::BuildHasherDefault;
#[cfg(feature = "use_murmur")] use murmur3::murmur3_32::MurmurHasher;


#[cfg(feature = "use_fnv")]
pub type HashMapT<K, V> = HashMap<K, V, FnvBuildHasher>;

#[cfg(feature = "use_murmur")]
pub type HashMapT<K, V> = HashMap<K, V,  BuildHasherDefault<MurmurHasher>>;

#[cfg(not(any(feature = "use_fnv", feature = "use_murmur")))]
pub type HashMapT<K, V> = HashMap<K, V>;

pub fn fill_linear_n(n: i32) -> HashMapT<i32, i32> {
    let mut hm = HashMapT::default();

    if cfg!(feature = "reserve_hm") {
        hm.reserve(n as usize);
    }

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

pub fn fill_linear_n_insert_random(n: i32) {
    let mut hm = fill_linear_n(n);
    let mut rng = thread_rng();

    for i in 0..n {
        hm.insert(rng.gen_range(0, n), i);
    }
}

pub fn fill_linear_n_lookup_random(n: i32) {
    let hm = fill_linear_n(n);
    let mut rng = thread_rng();

    for _ in 0..n {
        hm.get(&rng.gen_range(0, n));
    }
}

pub fn fill_linear_n_lookup_missing(n: i32) {
    let hm = fill_linear_n(n);
    let mut rng = thread_rng();

    for _ in 0..n {
        hm.get(&rng.gen_range(n, n*2));
    }
}

pub fn fill_linear_n_copy_element_wise(n: i32) {
    let hm = fill_linear_n(n);

    let mut hm_copy = HashMapT::default();
    if cfg!(feature = "reserve_hm") {
        hm_copy.reserve(n as usize);
    }

    for (key, val) in hm {
        hm_copy.insert(key, val);
    }
}

pub fn fill_linear_n_traversal(n: i32) {
    let hm = fill_linear_n(n);

    for (key, val) in hm {
        let _ = (key, val);
    }
}

// missing other key and value types

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    const SMALL_N: i32 = 10;
    //const MEDIUM_N: i32 = 100;
    //const LARGE_N: i32 = 1_000;
    //const BIG_N: i32 = 10_000;

    #[test]
    fn fill_linear_n_works() {
        let target_n = SMALL_N / 2;
        assert_eq!(&target_n, fill_linear_n(SMALL_N).get(&target_n).unwrap());
    }

    #[bench]
    fn small_fill_only(b: &mut Bencher) {
        b.iter(|| fill_linear_n(SMALL_N));
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
    fn small_insert_random(b: &mut Bencher) {
        b.iter(|| fill_linear_n_insert_random(SMALL_N));
    }

    #[bench]
    fn small_lookup_random(b: &mut Bencher) {
        b.iter(|| fill_linear_n_lookup_random(SMALL_N));
    }

    #[bench]
    fn small_lookup_missing(b: &mut Bencher) {
        b.iter(|| fill_linear_n_lookup_missing(SMALL_N));
    }

    #[bench]
    fn small_copy_element_wise(b: &mut Bencher) {
        b.iter(|| fill_linear_n_copy_element_wise(SMALL_N));
    }

    #[bench]
    fn small_traversal(b: &mut Bencher) {
        b.iter(|| fill_linear_n_traversal(SMALL_N));
    }
}

fn main() {
    let mut book_reviews = HashMap::new();
    book_reviews.insert(5, "value");
    println!("Hello, world! {}", book_reviews.get(&5).unwrap());
}
