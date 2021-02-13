use super::rays::*;

#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Intersection<T> {
	pub time: T,
	pub color: Vec3<T>
}

pub trait Intersectable<T> {
	fn intersect(&self, cast:&dyn Fn(Ray<T>) -> Vec3<T>, ray:Ray<T>) -> Option<Intersection<T>>;
}

#[derive(Debug)]
pub struct Ground {
}

impl Intersectable<f64> for Ground {
	fn intersect(&self, _cast:&dyn Fn(Ray<f64>) -> Vec3<f64>, ray:Ray<f64>) -> Option<Intersection<f64>> {
		let normal = Vec3::new(0., 0., 1.);
		let plen = project_length(-normal, ray.dir);

		if plen <= 0. { return None; }

		let intersection_time = ray.ori.z / plen;
		let intersection_point = ray.project(intersection_time);

		fn fmod(m:f64, n:f64) -> f64 { ((m % n) + n) % n }
		fn clamp01(v:f64) -> f64 {
			if v < 0.0 { return 0.0; }
			if v > 1.0 { return 1.0; }
			return v;
		}

		fn get_color(p:Vec3<f64>) -> Vec3<f64> {
			let par_x = fmod(p.x, 10.0) < 5.0;
			let par_y = fmod(p.y, 10.0) < 5.0;
			let cz = clamp01(p.y / 50.0);

			return if par_x ^ par_y { Vec3::new(cz, 0., 1.) } else { Vec3::new(cz, 1., 1.) }
		}

		return Some(Intersection {
			time: intersection_time,
			color: get_color(intersection_point)
		});
	}
}

#[derive(Debug)]
pub struct Sphere {
	pub center: Vec3<f64>,
	pub radius: f64
}

impl Intersectable<f64> for Sphere {
	fn intersect(&self, cast:&dyn Fn(Ray<f64>) -> Vec3<f64>, ray:Ray<f64>) -> Option<Intersection<f64>> {
		let center_ray = self.center - ray.ori;
		let closest_approach_time = project_length(ray.dir, center_ray);
		let closest_approach_point = ray.project(closest_approach_time);
		let closest_approach_distance = distance(closest_approach_point, self.center);

		if closest_approach_distance >= self.radius { return None; }

		let intersection_distance = (self.radius * self.radius + closest_approach_distance * closest_approach_distance).sqrt();
		let first_hit_time = closest_approach_time - intersection_distance;

		if first_hit_time < 0. { return None; }

		let first_hit_point = ray.project(first_hit_time);
		let normal = normalize(first_hit_point - self.center);

		let casted = cast(Ray { ori: first_hit_point, dir: reflect(normal, ray.dir) });

		return Some(Intersection {
			time: closest_approach_time,
			color: casted * 0.9
		});
	}
}
