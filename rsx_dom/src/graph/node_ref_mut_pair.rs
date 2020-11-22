
use rsx_tree::types::RefMutPair;

use crate::types::DOMNode;

#[derive(Debug, PartialEq)]
pub struct DOMArenaRefMutPair<'a, E: 'a, S: 'a, C: 'a, L: 'a> {
    raw: RefMutPair<'a, DOMNode<E, S, C, L>>
}

impl<'a, E, S, C, L> From<RefMutPair<'a, DOMNode<E, S, C, L>>> for DOMArenaRefMutPair<'a, E, S, C, L> {
    fn from(raw: RefMutPair<'a, DOMNode<E, S, C, L>>) -> Self {
        DOMArenaRefMutPair { raw }
    }
}

impl<'a, E, S, C, L> DOMArenaRefMutPair<'a, E, S, C, L> {
    
    pub fn values(&mut self) -> (&mut DOMNode<E, S, C, L>, &mut DOMNode<E, S, C, L>) {
        self.raw.try_values().expect("Nodes deallocated")
    }

    
    pub fn into_values(self) -> (&'a mut DOMNode<E, S, C, L>, &'a mut DOMNode<E, S, C, L>) {
        self.raw.try_into_values().expect("Nodes deallocated")
    }
}
