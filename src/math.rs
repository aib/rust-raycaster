use std::ops::{Mul, Add};

pub fn fmod(m:f64, n:f64) -> f64 {
	((m % n) + n) % n
}

pub fn clamp01(v:f64) -> f64 {
	if v < 0.0 { return 0.0; }
	if v > 1.0 { return 1.0; }
	return v;
}

pub fn mix <T: Mul<f64, Output=T> + Add<Output=T>> (a:T, b:T, t:f64) -> T {
	a * (1. - t) + b * t
}
