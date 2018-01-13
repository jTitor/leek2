/*!
 * Defines the transform object that provides a
 * basic scale/rotation/translation for an object.
 */
use math::{Vec3, Vec3Ops};
use math::{Vec4, Vec4Ops};
use math::{Mat4x4, MatOps};
// use math::Quaternion;

pub type ScaleType = Vec3;
pub type RotationType = Vec4; //TODO: quaternion
pub type PositionType = Vec3;

pub trait TransformOps<T> {
	fn scale(&self) -> ScaleType;
	fn rotate(&self) -> RotationType;
	fn position(&self) -> PositionType;

	fn mut_scale(mut& self) -> &ScaleType;
	fn mut_rotate(mut& self) -> &RotationType;
	fn mut_position(mut& self) -> &PositionType;

	fn concatenate(&self, other: &T) -> T;
	fn as_matrix(&self) -> Mat4x4;
}

pub struct Transform {
	pub scale_val: ScaleType,
	pub rotate_val: RotationType,
	pub position_val: PositionType
}

impl TransformOps<Transform> for Transform {
	fn scale(&self) -> ScaleType {
		self.scale_val
	}

	fn rotate(&self) -> RotationType {
		self.rotate_val
	}

	fn position(&self) -> PositionType {
		self.position_val
	}

	fn mut_scale(mut& self) -> &ScaleType {
		&self.scale_val
	}

	fn mut_rotate(mut& self) -> &RotationType {
		&self.rotate_val
	}

	fn mut_position(mut& self) -> &PositionType {
		&self.position_val
	}

	fn concatenate(&self, other: &Transform) -> Transform {
		unimplemented!()
	}

	fn as_matrix(&self) -> Mat4x4 {
		unimplemented!()
	}
}