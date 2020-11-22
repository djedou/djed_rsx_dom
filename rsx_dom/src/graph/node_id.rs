
use rsx_tree::types::Id;

use crate::types::DOMNode;

pub type DOMNodeId<E, S, C, L> = Id<DOMNode<E, S, C, L>>;
pub type DOMNodeIdPair<E, S, C, L> = (DOMNodeId<E, S, C, L>, DOMNodeId<E, S, C, L>);

pub type DOMNodeSiblingIds<E, S, C, L> = (Option<DOMNodeId<E, S, C, L>>, Option<DOMNodeId<E, S, C, L>>);
pub type DOMNodeEdgeIds<E, S, C, L> = (Option<DOMNodeId<E, S, C, L>>, Option<DOMNodeId<E, S, C, L>>);
