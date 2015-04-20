#![feature(test)]
#![feature(collections)]

extern crate test;
extern crate rand;
extern crate faster_bitset;

use std::collections::BitSet;
use test::Bencher;

use faster_bitset::*;

static N:usize = 1000000;

fn create_rnd(n: usize, p:f32) -> (BitSet, usize) {
    let mut s = BitSet::with_capacity(n);
    let mut k = 0;

    for i in 0..n-1 {
        if rand::random::<f32>() < p {
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
fn count_dense_fast(b: &mut Bencher) {
    let (s, k) = create_rnd(N, 0.5);

    b.iter(|| {
        let k2 = s.fast().count();
        assert_eq!(k, k2);
    })
}

#[bench]
fn copy_sparse(b: &mut Bencher) {
    let (s, _) = create_rnd(N, 0.1);
    let mut w = BitSet::with_capacity(N);

    b.iter(|| {
        for i in s.iter() {
            w.insert(i);
        }
    })
}

#[bench]
fn copy_sparse_fast(b: &mut Bencher) {
    let (s, _) = create_rnd(N, 0.1);
    let mut w = BitSet::with_capacity(N);
    
    b.iter(|| {
        for i in s.fast() {
            w.insert(i);
        }
    })
}

#[bench]
fn copy_dense(b: &mut Bencher) {
    let (s, _) = create_rnd(N, 0.5);
    let mut w = BitSet::with_capacity(N);

    b.iter(|| {
        for i in s.iter() {
            w.insert(i);
        }
    })
}

#[bench]
fn copy_dense_fast(b: &mut Bencher) {
    let (s, _) = create_rnd(N, 0.5);
    let mut w = BitSet::with_capacity(N);
    
    b.iter(|| {
        for i in s.fast() {
            w.insert(i);
        }
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
fn count_sparse_fast(b: &mut Bencher) {
    let (s, k) = create_rnd(N, 0.1);

    b.iter(|| {
        let k2 = s.fast().count();
        assert_eq!(k, k2);
    })
}