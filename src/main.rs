mod rays;
use rays::*;

#[derive(Debug)]
struct View {
	look: Ray<f64>,
	up: Vec3<f64>,
	vfov: f64,
	width: u32,
	height: u32
}

fn main() {
	let view = View {
		look: Ray {
			ori: Vec3::new(0., 0., 0.),
			dir: Vec3::new(0., 1., 0.)
		},
		up: Vec3::new(0., 0., 1.),
		vfov: f64::to_radians(45.),
		width: 10,
		height: 10
	};

	let rays = generate_rays(view);

	for ray in rays { println!("{}", ray); }
}

fn generate_rays(view:View) -> Vec<Vec3<f64>> {
	let hfov = view.width as f64 * view.vfov / view.height as f64;
	let right = cross(view.look.dir, view.up);

	let rotate_look = |axis:Vec3<f64>, fov:f64, max:u32, val:u32, look:Vec3<f64>| {
		let normalized = (val as f64 * 2.) / ((max - 1) as f64) - 1.;
		rotate(axis, normalized * fov, look)
	};

	let mut rays = Vec::new();

	for y in 0..view.height {
		let y_rotated = rotate_look(right, view.vfov, view.height, y, view.look.dir);
		for x in 0..view.width {
			let rotated = rotate_look(-view.up, hfov, view.width, x, y_rotated);
			rays.push(rotated);
		}
	}

	rays
}

fn rotate (axis: Vec3<f64>, theta: f64, vec: Vec3<f64>) -> Vec3<f64> {
	let a = (theta / 2.).cos();
	let v = normalize(axis) * -(theta / 2.).sin();
	let (b, c, d) = (v.x, v.y, v.z);
	let (aa, bb, cc, dd) = (a * a, b * b, c * c, d * d);
	let (bc, ad, ac, ab, bd, cd) = (b * c, a * d, a * c, a * b, b * d, c * d);

	return Vec3 {
		x: vec.x * (aa + bb - cc - dd) + vec.y * (2. * (bc + ad)) + vec.z * (2. * (bd - ac)),
		y: vec.x * (2. * (bc - ad)) + vec.y * (aa + cc - bb - dd) + vec.z * (2. * (cd + ab)),
		z: vec.x * (2. * (bd + ac)) + vec.y * (2. * (cd - ab)) + vec.z * (aa + dd - bb - cc),
	}
}

impl std::fmt::Display for Vec3<f64> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "({:+.3}, {:+.3}, {:+.3})", self.x, self.y, self.z)
	}
}
