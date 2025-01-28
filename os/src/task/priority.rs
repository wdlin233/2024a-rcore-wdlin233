//! Priority definition

use core::isize;

type PriorityInner = u32;

/// Task Priority
pub type Priority = PriorityImpl<PriorityInner>;

/// Task priority
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PriorityImpl<T>(pub(super) T);

impl<T> PriorityImpl<T> {
    pub const DEFAULT: isize = 16; // available in the whole module? 
}

impl<T> Default for PriorityImpl<T> 
where
    T: TryFrom<isize>,
    <T as TryFrom<isize>>::Error: core::fmt::Debug,
{
    fn default() -> Self {
        Self(T::try_from(Self::DEFAULT).unwrap())
    }
    
}

impl<T> TryFrom<isize> for PriorityImpl<T> 
where
    T: TryFrom<isize>,
    //<T as TryFrom<isize>>::Error: core::fmt::Debug,
{
    type Error = ();
    
    fn try_from(value: isize) -> Result<Self, Self::Error> {
        match value {
           // [variable @ subpattern](https://doc.rust-lang.org/reference/patterns.html#identifier-patterns)

           // primary version
           // value @ 2..=isize::MAX => Ok(Self::new(value.try_into().unwrap())),

           // value @ 2..=isize::MAX => Ok(Self(T::try_from(value).unwrap())),
           // if T type conversion failed, return Err(TryFromIntError), then `unwrap()` kernel panicked

           // instead  
           value @ 2..=isize::MAX => T::try_from(value)
                .map(Self) // PriorityImpl<T>
                .map_err(|_| ()), // type Error = ()
           _ => Err(()),  
        }
    }
}
