//! Stride definition for Stride algorithm

use super::priority::Priority;

type StrideInner = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Stride(StrideInner);

impl Stride {
    /// magir number, / 10000 is necessary
    const BIG_STRIDE: StrideInner = StrideInner::MAX / 10000;
}

impl Stride 
{
    pub fn step(&mut self, priority: Priority) 
    {
        //self.stride += Wrapping(T::BIG_STRIDE / priority.0)
        self.0 += Self::BIG_STRIDE / priority.0 as StrideInner
    }
}

impl Ord for Stride 
{
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    } 
}

impl PartialOrd for Stride
{
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other)) // call Ord::cmp
    }
}