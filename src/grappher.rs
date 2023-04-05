use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign};

use crate::math::Math;
use crate::plotter::Pixel;
use crate::traits::ToF32;

#[derive(Clone, Copy)]
pub struct Point {
	pub x: f32,
	pub y: f32,
}
impl Display for Point {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "[X: {}, Y: {}]", self.x, self.y)
	}
}
impl Point {
	pub fn new<T: ToF32>(x: T, y: T) -> Point {
		Point {
			x: x.to_f32(),
			y: y.to_f32(),
		}
	}
	pub fn to_pixel(&self) -> Pixel {
		Pixel::new(self.x.round_i(), self.y.round_i())
	}
	pub fn to_pixel_space(&self, size_px: Pixel, size_pt: Point, centre: Point) -> Pixel {
		let low_bound = (centre - (size_pt / 2f32)) * Point::new(1f32, -1f32);
		let high_bound = low_bound + (size_pt * Point::new(1, -1));
		self.lerp(low_bound, high_bound, Point::new(0, 0), size_px.to_point())
			.to_pixel()
	}
	pub fn lerp(
		&self,
		input_min: Point,
		input_max: Point,
		output_min: Point,
		output_max: Point,
	) -> Point {
		Point {
			x: self
				.x
				.map(input_min.x, input_max.x, output_min.x, output_max.x),
			y: self
				.y
				.map(input_min.y, input_max.y, output_min.y, output_max.y),
		}
	}
}

impl Mul<Point> for Point {
	type Output = Point;
	fn mul(self, rhs: Point) -> Point {
		Point {
			x: self.x * rhs.x,
			y: self.y * rhs.y,
		}
	}
}

impl MulAssign<Point> for Point {
	fn mul_assign(&mut self, rhs: Point) {
		*self = Point {
			x: self.x * rhs.x,
			y: self.y * rhs.y,
		};
	}
}

impl Mul<f32> for Point {
	type Output = Point;
	fn mul(self, rhs: f32) -> Point {
		Point {
			x: self.x * rhs,
			y: self.y * rhs,
		}
	}
}

impl Add<Point> for Point {
	type Output = Point;
	fn add(self, rhs: Point) -> Point {
		Point {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}
	}
}

impl Div<Point> for Point {
	type Output = Point;
	fn div(self, rhs: Point) -> Point {
		Point {
			x: self.x / rhs.x,
			y: self.y / rhs.y,
		}
	}
}
impl Div<f32> for Point {
	type Output = Point;
	fn div(self, rhs: f32) -> Point {
		Point {
			x: self.x / rhs,
			y: self.y / rhs,
		}
	}
}

impl Sub<Point> for Point {
	type Output = Point;
	fn sub(self, rhs: Point) -> Point {
		Point {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
		}
	}
}

impl SubAssign<Point> for Point {
	fn sub_assign(&mut self, rhs: Point) {
		self.x -= rhs.x;
		self.y -= rhs.y;
	}
}
impl MulAssign<f32> for Point {
	fn mul_assign(&mut self, rhs: f32) {
		self.x *= rhs;
		self.y *= rhs;
	}
}
impl AddAssign<Point> for Point {
	fn add_assign(&mut self, rhs: Point) {
		self.x += rhs.x;
		self.y += rhs.y;
	}
}
