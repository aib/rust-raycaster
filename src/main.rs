mod rays;
mod objects;

use rays::*;
use objects::*;

struct View {
	look: Ray<f64>,
	up: Vec3<f64>,
	vfov: f64,
	width: u32,
	height: u32
}

struct Scene {
	objects: Vec<Box<dyn Intersectable<f64>>>
}

fn main() {
	let view = View {
		look: Ray {
			ori: Vec3::new(0., 0., 5.),
			dir: Vec3::new(0., 1., 0.)
		},
		up: Vec3::new(0., 0., 1.),
		vfov: f64::to_radians(65.),
		width: 1280,
		height: 960
	};

	let mut scene = Scene {
		objects: vec!()
	};

	scene.objects.push(Box::new(Ground {}));
	scene.objects.push(Box::new(Sphere { center: Vec3::new(0., 80., 12.), radius: 10.}));

	fn conv_color(c:f64) -> u8 { (c * 255.) as u8 }

	let pixels = generate_rays(&view).iter()
		.map(|r| {
			cast(
				&scene,
				vec!(),
				Ray { ori: view.look.ori, dir: *r }
			)
		})
		.flat_map(|c| { vec!(conv_color(c.x), conv_color(c.y), conv_color(c.z)) })
		.collect::<Vec<u8>>();

	write_png(view.width, view.height, "/tmp/test.png", pixels);
}

fn cast(scene:&Scene, exceptions:Vec<&Box<dyn Intersectable<f64>>>, ray:Ray<f64>) -> Vec3<f64> {
	let mut intersections = scene.objects.iter()
		.filter(|o| {
			!exceptions.iter().any(|e| *o as *const Box<dyn Intersectable<f64>> == *e as *const Box<dyn Intersectable<f64>>)
		})
		.filter_map(|o| {
			let rec_cast = |ray| {
				cast(&scene, vec!(o), ray)
			};
			o.intersect(&rec_cast, ray)
		})
		.collect::<Vec<Intersection<f64>>>();

	intersections.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());

	return intersections
		.first()
		.map_or(Vec3::new(1., 1., 1.), |i| { i.color });
}

fn write_png(width:u32, height:u32, filename:&str, pixels:Vec<u8>) {
	let f = std::fs::File::create(std::path::Path::new(filename));
	let writer = std::io::BufWriter::new(f.unwrap());

	let mut encoder = png::Encoder::new(writer, width, height);
	encoder.set_color(png::ColorType::RGB);
	encoder.set_depth(png::BitDepth::Eight);

	encoder.write_header().unwrap()
		.write_image_data(pixels.as_slice()).unwrap();
}

fn generate_rays(view:&View) -> Vec<Vec3<f64>> {
	let hfov = view.width as f64 * view.vfov / view.height as f64;
	let right = cross(view.look.dir, view.up);

	let rotate_look = |axis:Vec3<f64>, fov:f64, max:u32, val:u32, look:Vec3<f64>| {
		let normalized = (val as f64 * 2.) / ((max - 1) as f64) - 1.;
		rotate(axis, normalized * (fov / 2.), look)
	};

	let mut rays = Vec::new();

	for y in (0..view.height).rev() {
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
