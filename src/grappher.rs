use std::ops::{Sub, SubAssign};

#[derive(Clone, Copy)]
pub struct Point {
	pub x: f32,
	pub y: f32,
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
