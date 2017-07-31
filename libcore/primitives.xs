primitive type bool;
operator || (lhs: bool, rhs: bool) -> bool;
operator && (lhs: bool, rhs: bool) -> bool;
operator ! (lhs: bool, rhs: bool) -> bool;

primitive type f32;
operator + (lhs: f32, rhs: f32) -> f32;
operator - (lhs: f32, rhs: f32) -> f32;
operator * (lhs: f32, rhs: f32) -> f32;
operator / (lhs: f32, rhs: f32) -> f32;
implicit cast (val: f32) -> f64;
explicit cast (val: f32) -> i32;
explicit cast (val: f32) -> i64;

primitive type f64;
operator + (lhs: f64, rhs: f64) -> f64;
operator - (lhs: f64, rhs: f64) -> f64;
operator * (lhs: f64, rhs: f64) -> f64;
operator / (lhs: f64, rhs: f64) -> f64;
explicit cast (val: f64) -> f32;
explicit cast (val: f64) -> i32;
explicit cast (val: f64) -> i64;

primitive type i32;
operator + (lhs: i32, rhs: i32) -> i32;
operator - (lhs: i32, rhs: i32) -> i32;
operator * (lhs: i32, rhs: i32) -> i32;
operator / (lhs: i32, rhs: i32) -> i32;
implicit cast (val: i32) -> i64;
explicit cast (val: i32) -> f32;
explicit cast (val: i32) -> f64;

primitive type i64;
operator + (lhs: i64, rhs: i64) -> i64;
operator - (lhs: i64, rhs: i64) -> i64;
operator * (lhs: i64, rhs: i64) -> i64;
operator / (lhs: i64, rhs: i64) -> i64;
explicit cast (val: f64) -> f32;
explicit cast (val: f64) -> i32;
explicit cast (val: f64) -> i64;