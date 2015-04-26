#![feature(test)]
#![feature(collections)]
#![feature(core)]

extern crate test;
extern crate rand;
extern crate faster_bitset;

use std::{u32};
use rand::*;
use test::{Bencher, black_box};


#[cfg(feature = "std")] use std::collections::{BitSet, BitVec};
#[cfg(not(feature = "std"))] use faster_bitset::{BitSet, BitVec};

const N : usize = 500_000;
const BENCH_BITS : usize = 1 << 14;


fn rng() -> rand::IsaacRng {
    let seed: &[_] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    rand::SeedableRng::from_seed(seed)
}

fn create_rnd(n: usize, p:f32) -> (BitSet, usize) {
    let mut r = rng();
    let mut s = BitSet::with_capacity(n);
    let mut k = 0;

    for i in 0..n-1 {
        if r.next_f32() < p {
            s.insert(i);
            k += 1;
        }
    }

    (s, k)
}


#[bench]
fn count_dense(b: &mut Bencher) {
    let (s, k) = create_rnd(N, 0.5);

    b.iter(|| {
        let k2 = s.iter().count();
        assert_eq!(k, k2);
    })
}

#[bench]
fn count_sparse(b: &mut Bencher) {
    let (s, k) = create_rnd(N, 0.1);

    b.iter(|| {
        let k2 = s.iter().count();
        assert_eq!(k, k2);
    })
}

#[bench]
fn copy_sparse(b: &mut Bencher) {
    let (s, _) = create_rnd(N, 0.1);
    let mut w = BitSet::with_capacity(N);

    b.iter(|| {
        for i in s.iter() { w.insert(i); }
        black_box(&w);
    })
}

#[bench]
fn copy_dense(b: &mut Bencher) {
    let (s, _) = create_rnd(N, 0.5);
    let mut w = BitSet::with_capacity(N);

    b.iter(|| {
        for i in s.iter() { w.insert(i); }
        black_box(&w);
    })
}

#[bench]
fn intersect_sparse(b: &mut Bencher) {
    let (s, _) = create_rnd(N, 0.1);
    let (s2, _) = create_rnd(N, 0.1);

    b.iter(|| {
        let k2 = s.intersection(&s2).count();
        black_box(&k2);
    })
}

#[bench]
fn bench_usize_small(b: &mut Bencher) {
    let mut r = rng();
    let mut bit_vec = 0 as usize;
    b.iter(|| {
        for _ in 0..100 {
            bit_vec |= 1 << ((r.next_u32() as usize) % u32::BITS);
        }
        black_box(&bit_vec);
    });
}

#[bench]
fn bench_bit_set_big_fixed(b: &mut Bencher) {
    let mut r = rng();
    let mut bit_vec = BitVec::from_elem(BENCH_BITS, false);
    b.iter(|| {
        for _ in 0..100 {
            bit_vec.set((r.next_u32() as usize) % BENCH_BITS, true);
        }
        black_box(&bit_vec);
    });
}

#[bench]
fn bench_bit_set_big_variable(b: &mut Bencher) {
    let mut r = rng();
    let mut bit_vec = BitVec::from_elem(BENCH_BITS, false);
    b.iter(|| {
        for _ in 0..100 {
            bit_vec.set((r.next_u32() as usize) % BENCH_BITS, r.gen());
        }
        black_box(&bit_vec);
    });
}

#[bench]
fn bench_bit_set_small(b: &mut Bencher) {
    let mut r = rng();
    let mut bit_vec = BitVec::from_elem(u32::BITS, false);
    b.iter(|| {
        for _ in 0..100 {
            bit_vec.set((r.next_u32() as usize) % u32::BITS, true);
        }
        black_box(&bit_vec);
    });
}

#[bench]
fn bench_bit_vec_big_union(b: &mut Bencher) {
    let mut b1 = BitVec::from_elem(BENCH_BITS, false);
    let b2 = BitVec::from_elem(BENCH_BITS, false);
    b.iter(|| {
        b1.union(&b2)
    })
}

#[bench]
fn bench_bit_vec_small_iter(b: &mut Bencher) {
    let bit_vec = BitVec::from_elem(u32::BITS, false);
    b.iter(|| {
        let mut sum = 0;
        for _ in 0..10 {
            for pres in &bit_vec {
                sum += pres as usize;
            }
        }
        sum
    })
}

#[bench]
fn bench_bit_vec_big_iter(b: &mut Bencher) {
    let bit_vec = BitVec::from_elem(BENCH_BITS, false);
    b.iter(|| {
        let mut sum = 0;
        for pres in &bit_vec {
            sum += pres as usize;
        }
        sum
    })
}

#[bench]
fn bench_bit_vecset_small(b: &mut Bencher) {
    let mut r = rng();
    let mut bit_vec = BitSet::new();
    b.iter(|| {
        for _ in 0..100 {
            bit_vec.insert((r.next_u32() as usize) % u32::BITS);
        }
        black_box(&bit_vec);
    });
}

#[bench]
fn bench_bit_vecset_big(b: &mut Bencher) {
    let mut r = rng();
    let mut bit_vec = BitSet::new();
    b.iter(|| {
        for _ in 0..100 {
            bit_vec.insert((r.next_u32() as usize) % BENCH_BITS);
        }
        black_box(&bit_vec);
    });
}

#[bench]
fn bench_bit_vecset_iter(b: &mut Bencher) {
    let bit_vec = BitSet::from_bit_vec(BitVec::from_fn(BENCH_BITS,
                                          |idx| {idx % 3 == 0}));
    b.iter(|| {
        let mut sum = 0;
        for idx in &bit_vec {
            sum += idx as usize;
        }
        sum
    })
}
