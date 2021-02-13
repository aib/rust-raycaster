use std::ops::{Mul, Div, Add, Sub};
use std::ops::Neg;

#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Vec3<T> {
	pub x: T,
	pub y: T,
	pub z: T
}

impl<T> Vec3<T> {
	pub fn new(x:T, y:T, z:T) -> Vec3<T> {
		Vec3 { x: x, y: y, z: z }
	}
}

impl <T: Add<Output=T> + Copy> Add<Vec3<T>> for Vec3<T> {
	type Output = Vec3<T>;
	fn add(self, rhs:Vec3<T>) -> Self::Output {
		Vec3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
	}
}

impl <T: Sub<Output=T> + Copy> Sub<Vec3<T>> for Vec3<T> {
	type Output = Vec3<T>;
	fn sub(self, rhs:Vec3<T>) -> Self::Output {
		Vec3 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
	}
}

impl <T: Neg<Output=T>> Neg for Vec3<T> {
	type Output = Vec3<T>;
	fn neg(self) -> Self::Output {
		Vec3 { x: -self.x, y: -self.y, z: -self.z }
	}
}

pub fn cross <T: Mul<Output=T> + Sub<Output=T> + Copy> (a:Vec3<T>, b:Vec3<T>) -> Vec3<T> {
	Vec3 {
		x: a.y * b.z - a.z * b.y,
		y: a.z * b.x - a.x * b.z,
		z: a.x * b.y - a.y * b.x
	}
}

pub fn dot <T: Mul<Output=T> + Add<Output=T> + Copy> (a:Vec3<T>, b:Vec3<T>) -> T {
	a.x * b.x + a.y * b.y + a.z * b.z
}

impl <T: Mul<Output=T> + Copy> Mul<T> for Vec3<T> {
	type Output = Vec3<T>;
	fn mul(self, rhs:T) -> Self::Output {
		Vec3 { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
	}
}

impl <T: Div<Output=T> + Copy> Div<T> for Vec3<T> {
	type Output = Vec3<T>;
	fn div(self, rhs:T) -> Self::Output {
		Vec3 { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
	}
}

pub trait Sqrt { fn sqrt(self) -> Self; }
impl Sqrt for f64 { fn sqrt(self) -> Self { self.sqrt() } }

pub fn norm <T: Mul<Output=T> + Add<Output=T> + Sqrt + Copy> (vec:Vec3<T>) -> T {
	dot(vec, vec).sqrt()
}

pub fn normalize <T: Mul<Output=T> + Add<Output=T> + Div<Output=T> + Sqrt + Copy> (vec:Vec3<T>) -> Vec3<T> {
	vec / norm(vec)
}

pub fn project_length <T: Mul<Output=T> + Add<Output=T> + Div<Output=T> + Sqrt + Copy> (target:Vec3<T>, source:Vec3<T>) -> T {
	dot(source, normalize(target))
}

pub fn distance <T: Mul<Output=T> + Add<Output=T> + Sub<Output=T> + Sqrt + Copy> (a:Vec3<T>, b:Vec3<T>) -> T {
	let dx = a.x - b.x;
	let dy = a.y - b.y;
	let dz = a.z - b.z;
	(dx * dx + dy * dy + dz * dz).sqrt()
}

#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Ray<T> {
	pub ori: Vec3<T>,
	pub dir: Vec3<T>
}

impl <T: Mul<Output=T> + Add<Output=T> + Copy> Ray<T> {
	pub fn project(&self, t:T) -> Vec3<T> {
		self.ori + self.dir * t
	}
}
