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

// TODO other key and value types
pub type KeyT = i32;
pub type ValueT = i32;

pub fn fill_linear_n(n: i32) -> HashMapT<KeyT, ValueT> {
    let mut hm = HashMapT::default();

    if cfg!(feature = "reserve_hm") {
        hm.reserve(n as usize);
    }

    for i in 0..n {
        hm.insert(KeyT::from(i), ValueT::from(i));
    }
    hm
}

pub fn fill_linear_n_lookup_one(n: i32) {
    fill_linear_n(n).get(&(KeyT::from(n / 2))).unwrap();
}

pub fn fill_linear_n_lookup_all(n: i32) {
    let hm = fill_linear_n(n);

    for i in 0..n {
        hm.get(&KeyT::from(i)).unwrap();
    }
}

pub fn fill_linear_n_insert_random(n: i32) {
    let mut hm = fill_linear_n(n);
    let mut rng = thread_rng();

    for i in 0..n {
        hm.insert(KeyT::from(rng.gen_range(0, n)), ValueT::from(i));
    }
}

pub fn fill_linear_n_lookup_random(n: i32) {
    let hm = fill_linear_n(n);
    let mut rng = thread_rng();

    for _ in 0..n {
        hm.get(&KeyT::from(rng.gen_range(0, n)));
    }
}

pub fn fill_linear_n_lookup_missing(n: i32) {
    let hm = fill_linear_n(n);
    let mut rng = thread_rng();

    for _ in 0..n {
        hm.get(&KeyT::from(rng.gen_range(n, n*2)));
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

#[cfg(test)]
mod tests {
    use super::*;
    const TINY_N: i32 = 10;
    const SMALL_N: i32 = 100;
    const MEDIUM_N: i32 = 1_000;
    const LARGE_N: i32 = 10_000;
    const BIG_N: i32 = 100_000;

    macro_rules! make_benchmark_impl {
        ($name:ident, $benchmark:expr, $size:ident) => {
            #[bench]
            fn $name(b: &mut Bencher) {
                b.iter(|| $benchmark(test::black_box($size)));
            }
        }
    }

    macro_rules! make_benchmarks {
        {$($name:ident: $benchmark:expr),*} => {
            mod tiny {
                use super::*;
                use test::Bencher;
                $(make_benchmark_impl!($name, $benchmark, TINY_N);)*
            }
            mod small {
                use super::*;
                use test::Bencher;
                $(make_benchmark_impl!($name, $benchmark, SMALL_N);)*
            }
            mod medium {
                use super::*;
                use test::Bencher;
                $(make_benchmark_impl!($name, $benchmark, MEDIUM_N);)*
            }
            mod large {
                use super::*;
                use test::Bencher;
                $(make_benchmark_impl!($name, $benchmark, LARGE_N);)*
            }
            mod big {
                use super::*;
                use test::Bencher;
                $(make_benchmark_impl!($name, $benchmark, BIG_N);)*
            }
        }
    }

    #[test]
    fn fill_linear_n_works() {
        let target_n = TINY_N / 2;
        assert_eq!(&target_n, fill_linear_n(TINY_N).get(&target_n).unwrap());
    }

    make_benchmarks!{
        copy_element_wise: fill_linear_n_copy_element_wise,
        fill_only: fill_linear_n,
        insert_random: fill_linear_n_insert_random,
        lookup_all: fill_linear_n_lookup_all,
        lookup_missing: fill_linear_n_lookup_missing,
        lookup_one: fill_linear_n_lookup_one,
        lookup_random: fill_linear_n_lookup_random,
        traversal: fill_linear_n_traversal
    }
}

fn main() {
    let mut book_reviews = HashMap::new();
    book_reviews.insert(5, "value");
    println!("Hello, world! {}", book_reviews.get(&5).unwrap());
}
