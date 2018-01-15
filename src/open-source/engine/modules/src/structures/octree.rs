/*!
 * Tree with 8 subnodes, for organizing 3D spaces.
 */
use std::marker::PhantomData;

//TODO: should make this a type of Tree
pub trait OcTreeOps<NodeT> {
}

pub struct OcTree<NodeT> {
	node_type: PhantomData<NodeT>
}

impl<NodeT> OcTreeOps<NodeT> for OcTree<NodeT> {
}