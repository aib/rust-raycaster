use super::rays::*;

#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Intersection<T> {
	pub time: T,
	pub color: Vec3<T>
}

pub trait Intersectable<T> {
	fn intersect(&self, ray:Ray<T>) -> Option<Intersection<T>>;
}

#[derive(Debug)]
pub struct Ground {
}

impl Intersectable<f64> for Ground {
	fn intersect(&self, ray:Ray<f64>) -> Option<Intersection<f64>> {
		let normal = Vec3::new(0., 0., 1.);
		let plen = project_length(-normal, ray.dir);

		if plen <= 0. { return None; }

		let intersection_time = ray.ori.z / plen;
		let intersection_point = ray.project(intersection_time);

		fn fmod(m:f64, n:f64) -> f64 { ((m % n) + n) % n }

		fn get_color(p:Vec3<f64>) -> Vec3<f64> {
			let par_x = fmod(p.x, 10.0) < 5.0;
			let par_y = fmod(p.y, 10.0) < 5.0;

			return if par_x ^ par_y { Vec3::new(0., 0., 1.) } else { Vec3::new(0., 1., 1.) }
		}

		return Some(Intersection {
			time: intersection_time,
			color: get_color(intersection_point)
		});
	}
}
