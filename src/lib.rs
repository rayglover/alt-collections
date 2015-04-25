#![feature(core)]
mod bit;

pub use bit_vec::BitVec;
pub use bit_set::BitSet;

#[unstable(feature = "collections",
           reason = "RFC 509")]
pub mod bit_vec {
    pub use bit::{BitVec, Iter};
}

#[unstable(feature = "collections",
           reason = "RFC 509")]
pub mod bit_set {
    pub use bit::{BitSet, Union, Intersection, Difference, SymmetricDifference};
    pub use bit::SetIter as Iter;
}