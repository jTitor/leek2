/*!
	Module definition for math.
*/

//Definitions for linear algebra constructs
//(vectors, matrices, quaternions).
pub mod linear_algebra;
//Import the common modules used here.
pub use self::linear_algebra::vec2::Vec2;
pub use self::linear_algebra::vec3::Vec3;
//pub use linear_algebra::vec4::Vec4 as math::Vec4;
pub use self::linear_algebra::mat4x4::Mat4x4;
//pub use linear_algebra::quaternion::Quaternion as math::Quaternion;
//end linear_algebra

//PRNG operations.
pub mod random;
//end random