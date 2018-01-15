/*!
 * Tree with arbitrary number of nodes.
 */
use std::marker::PhantomData;

pub trait TreeOps<NodeT> {}

pub struct Tree<NodeT> {
	node_type: PhantomData<NodeT>
}

impl<NodeT> TreeOps<NodeT> for Tree<NodeT> {}