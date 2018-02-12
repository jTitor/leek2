/*!
 * Implementation of the scene graph.
 */
use std::marker::PhantomData;

use graphics::scene::SceneGraph;
use graphics::scene::SceneNode;

pub struct SceneGraphInternal<NodeT> where NodeT : Sized + SceneNode {
	node_type: PhantomData<NodeT>
}

impl<NodeT> SceneGraphInternal<NodeT> where NodeT : Sized + SceneNode {
}

impl<NodeT> SceneGraph<NodeT> for SceneGraphInternal<NodeT> where NodeT : Sized + SceneNode {
	fn add_node(&self, node: &NodeT) {
		let _unimplemented = node;
		unimplemented!();
	}

	fn remove_node(&self, node: &NodeT) {
		let _unimplemented = node;
		unimplemented!();
	}

	fn get_renderable_nodes(&self) -> Vec<NodeT> {
		unimplemented!();
	}
}