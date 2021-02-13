pub fn fmod(m:f64, n:f64) -> f64 {
	((m % n) + n) % n
}

pub fn clamp01(v:f64) -> f64 {
	if v < 0.0 { return 0.0; }
	if v > 1.0 { return 1.0; }
	return v;
}
