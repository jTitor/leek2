/*!
 * Scene graph.
 */

pub trait SceneGraph {

}

pub struct SceneGraphFactory {

}

pub impl SceneGraphFactory {
	fn build(&self) -> Box<SceneGraph> {
		unimplemented!();
	}
}