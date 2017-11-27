struct Vec4 {
	x: f32,
	y: f32,
	z: f32,
	w: f32,
}

fn vec4(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
	return Vec4 {
		x: x,
		y: y,
		z: z,
		w: w,
	};
}

fn main() -> Vec4 {
	return vec4(0.0, 0.0, 0.0, 0.0);
}