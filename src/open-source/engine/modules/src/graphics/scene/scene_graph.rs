/*!
 * Scene graph.
 */
use super::SceneNode;

pub trait SceneGraph<NodeT> where NodeT : Sized + SceneNode {
	fn add_node(&self, node: &NodeT);
	fn remove_node(&self, node: &NodeT);

	/**
	 * Given a viewing volume, return
	 a list of nodes to render.
	 The nodes should be able to be rendered
	 in the order specified.
	 */
	fn get_renderable_nodes(&self) -> Vec<NodeT>;
}