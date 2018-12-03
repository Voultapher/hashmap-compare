#![feature(test)]

extern crate fnv;
extern crate fxhash;
extern crate hashbrown;
extern crate murmur3;
extern crate rand;
extern crate test;

use std::collections::HashMap;

use rand::{Rng, thread_rng};

#[cfg(feature = "fnv_hash")] use fnv::FnvBuildHasher;

#[cfg(feature = "fx_hash")] use fxhash::FxBuildHasher;

#[cfg(feature = "murmur_hash")] use std::hash::BuildHasherDefault;
#[cfg(feature = "murmur_hash")] use murmur3::murmur3_32::MurmurHasher;

#[cfg(feature = "fnv_hash")]
pub type HashMapT<K, V> = HashMap<K, V, FnvBuildHasher>;

#[cfg(feature = "fx_hash")]
pub type HashMapT<K, V> = HashMap<K, V, FxBuildHasher>;

#[cfg(feature = "murmur_hash")]
pub type HashMapT<K, V> = HashMap<K, V,  BuildHasherDefault<MurmurHasher>>;

#[cfg(feature = "hashbrown_map")]
pub type HashMapT<K, V> = hashbrown::HashMap<K, V>;

#[cfg(not(any(
    feature = "fnv_hash",
    feature = "murmur_hash",
    feature = "fx_hash",
    feature = "hashbrown_map"
)))]
pub type HashMapT<K, V> = HashMap<K, V>;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct CustomString(String);

impl From<i32> for CustomString {
    #[cfg(feature = "string_pad")]
    fn from(v: i32) -> CustomString {
        let mut s = v.to_string();
        s.push_str("0_1_2_3_4_5_6_7_"); // padding
        CustomString(s)
    }

    #[cfg(not(feature = "string_pad"))]
    fn from(v: i32) -> CustomString {
        CustomString(v.to_string())
    }
}

#[cfg(feature = "string_key")]
pub type KeyT = CustomString;

#[cfg(not(feature = "string_key"))]
pub type KeyT = i32;

#[cfg(feature = "string_value")]
pub type ValueT = CustomString;

#[cfg(not(feature = "string_value"))]
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

pub fn fill_linear_n_lookup_one(n: i32) -> ValueT {
    fill_linear_n(n).get(&(KeyT::from(n / 2))).unwrap().clone()
}

pub fn fill_linear_n_lookup_all(n: i32) -> i32 {
    let hm = fill_linear_n(n);

    let mut side_effect: i32 = 0;
    for i in 0..n {
        if let Some(_) = hm.get(&KeyT::from(i)) {
            side_effect += 1;
        }
    }
    side_effect
}

// TODO fill random from empty

// TODO explicit rehash

pub fn fill_linear_n_insert_random(n: i32) -> i32 {
    let mut hm = fill_linear_n(n);
    let mut rng = thread_rng();

    let mut side_effect: i32 = 0;
    for i in 0..n {
        hm.insert(KeyT::from(rng.gen_range(0, n)), ValueT::from(i));
        if rng.gen_range(0, n) < (n / 2) {
            side_effect += 1;
        }
    }
    side_effect + hm.len() as i32
}

pub fn fill_linear_n_lookup_random(n: i32) -> i32 {
    let hm = fill_linear_n(n);
    let mut rng = thread_rng();

    let mut side_effect: i32 = 0;
    for _ in 0..n {
        if let Some(_) = hm.get(&KeyT::from(rng.gen_range(0, n))) {
            side_effect += 1;
        }
    }
    side_effect
}

pub fn fill_linear_n_lookup_missing(n: i32) -> i32 {
    let hm = fill_linear_n(n);
    let mut rng = thread_rng();

    let mut side_effect: i32 = 0;
    for _ in 0..n {
        if let Some(_) = hm.get(&KeyT::from(rng.gen_range(n, n*2))) {
            side_effect += 1;
        }
    }
    side_effect
}

pub fn fill_linear_n_copy_element_wise(n: i32) -> i32 {
    let hm = fill_linear_n(n);

    let mut hm_copy = HashMapT::default();
    if cfg!(feature = "reserve_hm") {
        hm_copy.reserve(n as usize);
    }

    for (key, value) in hm {
        hm_copy.insert(key, value);
    }

    hm_copy.len() as i32
}

// TODO add this benchmark to the other languages.
pub fn fill_linear_n_clone(n: i32) -> i32 {
    let hm = fill_linear_n(n);

    let hm_copy = hm.clone();

    hm_copy.len() as i32
}

pub fn fill_linear_n_traversal(n: i32) -> i32 {
    let hm = fill_linear_n(n);

    let mut side_effect: i32 = 0;
    for (_key, _value) in hm {
        side_effect += 1;
    }
    side_effect
}

pub fn random_gen_only(n: i32) -> i32 {
    let mut rng = thread_rng();

    let mut side_effect: i32 = 0;
    for _ in 0..n {
        if rng.gen_range(0, n) < (n / 2) {
            side_effect += 1;
        }
    }
    side_effect
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

    make_benchmarks!{
        fill_only: fill_linear_n,
        lookup_one: fill_linear_n_lookup_one,
        lookup_all: fill_linear_n_lookup_all,
        lookup_missing: fill_linear_n_lookup_missing,
        lookup_random: fill_linear_n_lookup_random,
        insert_random: fill_linear_n_insert_random,
        traversal: fill_linear_n_traversal,
        copy_element_wise: fill_linear_n_copy_element_wise,
        clone: fill_linear_n_clone,
        random_gen: random_gen_only
    }
}

fn main() {
    let mut book_reviews = HashMap::new();
    book_reviews.insert(5, "value");
    println!("Hello, world! {}", book_reviews.get(&5).unwrap());
}
