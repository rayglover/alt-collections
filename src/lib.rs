#![feature(collections)]
#![feature(core)]

use std::collections::bit_set::*;
use std::iter::{Cloned};
use std::u32;
use std::slice;

pub struct FastIter<'a> {
    head: u32, 
    head_offset: usize,
    tail: Cloned<slice::Iter<'a, u32>>
}

impl<'a> Iterator for FastIter<'a> {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<usize> {
        while self.head == 0 {
            match self.tail.next() {
                Some(w) => { 
                    self.head = w;
                    self.head_offset += u32::BITS;
                },
                None => return None
            }
        }

        // let t = self.head & -self.head;
        let t = self.head & !self.head + 1;
        // remove the least significant bit
        self.head &= self.head - 1;
        // return index of lsb
        Some(self.head_offset + (u32::count_ones(t-1) as usize))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.tail.size_hint() {
            (l, Some(h)) => (l * (u32::BITS as usize), Some(1 + h * (u32::BITS as usize))),
            n => n
        }
    }
}

pub trait FastBits {
    fn fast(&self) -> FastIter;
}

impl FastBits for BitSet {

    fn fast(&self) -> FastIter {
        let mut bs = self.get_ref().blocks();
        let x = bs.next().unwrap_or(0);

        FastIter {tail: bs, head: x, head_offset: 0}
    }
}


#[cfg(test)]
mod test {
    
    extern crate rand;
    
    use std::collections::BitSet;
    use FastBits;

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

    #[test]
    fn copy() {
        let (s, _) = create_rnd(10000, 0.5);

        let it1 = s.iter();
        let it2 = s.fast();
        for (i, j) in it1.zip(it2) {
            assert_eq!(i, j);
        }

    }

}

