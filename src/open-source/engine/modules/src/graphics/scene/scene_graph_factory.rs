/*!
 * Generates SceneGraph implementations.
 */
use graphics::scene::{SceneGraph, SceneNode};
use std::marker::PhantomData;

pub struct SceneGraphFactory<NodeT> where NodeT : Sized + SceneNode {
	node_type: PhantomData<NodeT>
}

impl<NodeT> SceneGraphFactory<NodeT> where NodeT : Sized + SceneNode {
	pub fn build(&self) -> Box<SceneGraph<NodeT>> {
		unimplemented!();
	}
}