/*!
 * Implementation of the scene graph.
 */

use graphics::scene::SceneGraph;
use graphics::scene::SceneNode;

pub struct SceneGraphInternal {

}

impl SceneGraphInternal {
}

impl SceneGraph for SceneGraphInternal {
	fn add_node(&self, node: &SceneNode) {
		unimplemented!();
	}

	fn remove_node(&self, node: &SceneNode) {
		unimplemented!();
	}

	fn get_renderable_nodes(&self) -> Vec<SceneNode> {
		unimplemented!();
	}
}