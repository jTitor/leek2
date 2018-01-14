/*!
 * Scene graph.
 */
use super::SceneNode;

pub trait SceneGraph {
	fn add_node(&self, node: &SceneNode);
	fn remove_node(&self, node: &SceneNode);

	/**
	 * Given a viewing volume, return
	 a list of nodes to render.
	 The nodes should be able to be rendered
	 in the order specified.
	 */
	fn get_renderable_nodes(&self) -> Vec<SceneNode>;
}