//! Priority definition

use core::isize;

use super::task::TaskControlBlockInner;

type PriorityInner = u32;

/// Task priority
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Priority(PriorityInner);

impl Priority {
    const DEFAULT: PriorityInner = 16; // available in the whole module? 

    /// new with priority
    pub fn new(value: PriorityInner) -> Self {
        Self(value)
    }
}

impl Default for Priority {
    fn default() -> Self {
        Self(Self::DEFAULT)
    }
    
}

impl TryFrom<isize> for Priority {
    type Error = ();
    
    fn try_from(value: isize) -> Result<Self, Self::Error> {
        match value {
            // [variable @ subpattern](https://doc.rust-lang.org/reference/patterns.html#identifier-patterns)
           value @ 2..=isize::MAX => Ok(Self::new(value.try_into().unwrap())),
           _ => Err(()),  
        }
    }
}

impl TaskControlBlockInner {
    /// set task priority
    pub fn set_priority(&mut self, priority: Priority) {
        self.priority = priority
    }
}