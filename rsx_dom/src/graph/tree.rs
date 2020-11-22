
use std::ops::{Deref, DerefMut};
use std::any::Any;
use djed_rsx_shared::traits::{
    style_traits::{TComputedStyles, TStyleDeclarations},
    event_traits::{TGenericEvent},
    layout_traits::{TLayoutNode},
    dom_traits::{ TDOMTree}
};
use rsx_tree::types::Tree;

use crate::types::{DOMArenaRef, DOMArenaRefMut, DOMArenaRefMutPair, DOMNode, DOMNodeId, DOMNodeIdPair};

#[derive(Debug, PartialEq)]
pub struct DOMTree<E, S, C, L> {
    raw: Tree<DOMNode<E, S, C, L>>
}

impl<E, S, C, L> Default for DOMTree<E, S, C, L>
where
    E: TGenericEvent,
    S: TStyleDeclarations,
    C: TComputedStyles<Styles = S>,
    L: TLayoutNode<Styles = S>
{
    fn default() -> Self {
        DOMTree {
            raw: Tree::new(DOMNode::default())
        }
    }
}

impl<E, S, C, L> Deref for DOMTree<E, S, C, L> {
    type Target = DOMNode<E, S, C, L>;

    fn deref(&self) -> &Self::Target {
        self.root().into_value()
    }
}

impl<E, S, C, L> DerefMut for DOMTree<E, S, C, L> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.root_mut().into_value()
    }
}

impl<E, S, C, L> DOMTree<E, S, C, L> {
    pub(crate) fn into_inner(self) -> Tree<DOMNode<E, S, C, L>> {
        self.raw
    }

    pub fn root(&self) -> DOMArenaRef<E, S, C, L> {
        let id = self.raw.root();
        DOMArenaRef::from(self.raw.get(id))
    }

    pub fn root_mut(&mut self) -> DOMArenaRefMut<E, S, C, L> {
        let id = self.raw.root();
        DOMArenaRefMut::from(self.raw.get_mut(id))
    }

    pub fn document(&self) -> DOMArenaRef<E, S, C, L> {
        let id = self.root().first_child_id().unwrap();
        DOMArenaRef::from(self.raw.get(id))
    }

    pub fn document_mut(&mut self) -> DOMArenaRefMut<E, S, C, L> {
        let id = self.root().first_child_id().unwrap();
        DOMArenaRefMut::from(self.raw.get_mut(id))
    }

    pub fn alloc(&mut self, node: DOMNode<E, S, C, L>) -> DOMNodeId<E, S, C, L> {
        self.raw.alloc(node)
    }

    pub fn get(&self, id: DOMNodeId<E, S, C, L>) -> DOMArenaRef<E, S, C, L> {
        DOMArenaRef::from(self.raw.get(id))
    }

    pub fn get_mut(&mut self, id: DOMNodeId<E, S, C, L>) -> DOMArenaRefMut<E, S, C, L> {
        DOMArenaRefMut::from(self.raw.get_mut(id))
    }

    pub fn get_mut_pair(&mut self, ids: DOMNodeIdPair<E, S, C, L>) -> DOMArenaRefMutPair<E, S, C, L> {
        DOMArenaRefMutPair::from(self.raw.get_mut_pair(ids))
    }
}

impl<E, S, C, L> TDOMTree for DOMTree<E, S, C, L>
where
    E: TGenericEvent,
    S: TStyleDeclarations,
    C: TComputedStyles,
    L: TLayoutNode
{
    type Node = DOMNode<E, S, C, L>;

    fn get_node(&self, id: DOMNodeId<E, S, C, L>) -> &Self::Node {
        self.get(id).into_value()
    }

    fn get_node_mut(&mut self, id: DOMNodeId<E, S, C, L>) -> &mut Self::Node {
        self.get_mut(id).into_value()
    }

    fn get_node_mut_pair(&mut self, ids: DOMNodeIdPair<E, S, C, L>) -> (&mut Self::Node, &mut Self::Node) {
        self.get_mut_pair(ids).into_values()
    }
}

impl<E, S, C, L> DOMTree<E, S, C, L>
where
    E: TGenericEvent,
    S: TStyleDeclarations,
    C: TComputedStyles,
    L: TLayoutNode
{
    pub fn generate_layout_tree(&mut self, resources: &L::Resources)
    where
        L: TLayoutNode<TextMeasureMetadata = C, ImageMeasureMetadata = (), NormalMeasureMetadata = Box<dyn Any>>
    {
        self.root_mut().build_layout(resources);
    }
}
