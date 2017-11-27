/*!
	Module definition for linear_algebra.
*/
//Vectors.
pub mod vec_base;
pub mod vec2;
pub mod vec3;
pub mod vec4;
//Shortcut import all of these.
pub use self::vec_base::VecOps;
pub use self::vec2::{Vec2Access, Vec2Ops};
pub use self::vec3::{Vec3Access, Vec3Ops};
pub use self::vec4::{Vec4Access, Vec4Ops};

//Matrices.
pub mod mat4x4;
pub use self::mat4x4::MatOps;

//Quaternion rotation.
pub mod quaternion;

//Import the implementation structs too.
pub mod internal;
pub use self::internal::{Vec2, Vec3, Vec4, Mat4x4};