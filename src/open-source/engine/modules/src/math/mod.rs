/*!
	Module definition for math.
*/

//Definitions for linear algebra constructs
//(vectors, matrices, quaternions).
pub mod linear_algebra;
//Import the common modules used here.
pub use self::linear_algebra::{VecOps, Vec2Access, Vec3Access, Vec4Access};
pub use self::linear_algebra::{Vec2Ops, Vec3Ops, Vec4Ops};
pub use self::linear_algebra::{Vec2, Vec3, Vec4};
//pub use linear_algebra::vec4::Vec4 as math::Vec4;
pub use self::linear_algebra::{Mat4x4, MatOps};
//pub use linear_algebra::quaternion::Quaternion
//end linear_algebra

//PRNG operations.
pub mod random;
//end random

pub mod scalar;
//Import nearly_equal here since it's so commonly used.
pub use self::scalar::nearly_equal;

pub mod transform;
pub use self::transform::{Transform, TransformOps};