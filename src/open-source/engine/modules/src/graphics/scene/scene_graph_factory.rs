/*!
 * Generates SceneGraph implementations.
 */
use graphics::scene::{SceneGraph, SceneNode};
use std::marker::PhantomData;
use super::internal::SceneGraphInternal;

pub struct SceneGraphFactory<NodeT> where NodeT : Sized + SceneNode {
	node_type: PhantomData<NodeT>
}

impl<NodeT> SceneGraphFactory<NodeT> where NodeT : Sized + SceneNode {
	fn build(&self) -> Box<SceneGraph<NodeT>> {
		unimplemented!();
	}
}