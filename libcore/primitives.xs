primitive type bool;

primitive type f32;
operator + (lhs: f32, rhs: f32) -> f32;
operator - (lhs: f32, rhs: f32) -> f32;
operator * (lhs: f32, rhs: f32) -> f32;
operator / (lhs: f32, rhs: f32) -> f32;
implicit cast f32 -> f64;
explicit cast f32 -> i32;
explicit cast f32 -> i64;

primitive type f64;
operator + (lhs: f64, rhs: f64) -> f64;
operator - (lhs: f64, rhs: f64) -> f64;
operator * (lhs: f64, rhs: f64) -> f64;
operator / (lhs: f64, rhs: f64) -> f64;
explicit cast f64 -> f32;
explicit cast f64 -> i32;
explicit cast f64 -> i64;

primitive type i32;
operator + (lhs: i32, rhs: i32) -> i32;
operator - (lhs: i32, rhs: i32) -> i32;
operator * (lhs: i32, rhs: i32) -> i32;
operator / (lhs: i32, rhs: i32) -> i32;
implicit cast i32 -> i64;
explicit cast i32 -> f32;
explicit cast i32 -> f64;

primitive type i64;
operator + (lhs: i64, rhs: i64) -> i64;
operator - (lhs: i64, rhs: i64) -> i64;
operator * (lhs: i64, rhs: i64) -> i64;
operator / (lhs: i64, rhs: i64) -> i64;
explicit cast i64 -> f32;
explicit cast i64 -> i32;
explicit cast i64 -> i64;

primitive type vec3;
primitive type vec4;
primitive type mat4x4;
