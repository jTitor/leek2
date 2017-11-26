/*!
	Module definition for math.
*/

//Definitions for linear algebra constructs
//(vectors, matrices, quaternions).
pub mod linear_algebra;
//Import the common modules used here.
pub use self::linear_algebra::vec_base;
pub use self::linear_algebra::vec_base::{VecOps, Vec2Access, Vec3Access, Vec4Access};
pub use self::linear_algebra::vec2::Vec2;
pub use self::linear_algebra::vec3::Vec3;
//pub use linear_algebra::vec4::Vec4 as math::Vec4;
pub use self::linear_algebra::mat4x4::{Mat4x4, MatOps};
//pub use linear_algebra::quaternion::Quaternion as math::Quaternion;
//end linear_algebra

//PRNG operations.
pub mod random;
//end random

pub mod scalar;
//Import nearly_equal here since it's so commonly used.
pub use self::scalar::nearly_equal;