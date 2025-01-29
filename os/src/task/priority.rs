//! Priority definition

use core::isize;

type PriorityInner = isize;

/// Task priority
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Priority(pub PriorityInner); 

impl Priority {
    /// default value for Priority
    pub const DEFAULT: PriorityInner = 16; // available in the whole module? 

    /// new with priority
    pub fn new(value: PriorityInner) -> Self {
        Self(value)
    }
}

impl Default for Priority 
{
    fn default() -> Self {
        Self(Self::DEFAULT)
    }
    
}

impl TryFrom<isize> for Priority 
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
           value @ 2..=isize::MAX => PriorityInner::try_from(value)
                .map(Self) // PriorityImpl<T>
                .map_err(|_| ()), // type Error = ()
            
           _ => Err(()),  
        }
    }
}
