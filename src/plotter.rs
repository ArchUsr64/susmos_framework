use crate::math::Math;
use std::vec;

use crate::grappher::Point;

pub struct Plotter {
	pub size_px: (usize, usize),
	pub size_pt: Point,
	pub centre: Point,
	px_buffer: Vec<Vec<bool>>,
}

impl Plotter {
	pub fn new(size_px: (usize, usize), size_pt: Point, centre: Point) -> Plotter {
		Plotter {
			size_px,
			size_pt,
			centre,
			px_buffer: vec![vec![false; size_px.0]; size_px.1],
		}
	}
	fn get_point_corresponding_to_screen_corners(&self) -> (Point, Point) {
		let low_bound = (self.centre - (self.size_pt / 2f32)) * Point::new(1f32, -1f32);
		let high_bound = low_bound + (self.size_pt * Point::new(1, -1));
		(low_bound, high_bound)
	}
	fn centered_point_in_draw_area(&self, point: Point) -> bool {
		self.size_pt.x.abs() > point.x.abs() || self.size_pt.y.abs() > point.y.abs()
	}
	pub fn plot(&mut self, point: Point) {
		let mut point = point;
		point -= self.centre;
		if self.centered_point_in_draw_area(point) {
			let ((pt_min, pt_max), screen_max) = (
				self.get_point_corresponding_to_screen_corners(),
				Point::new(self.size_px.0, self.size_px.1),
			);
			let screen_space_coordinate =
				(point * Point::new(1, -1)).lerp(pt_min, pt_max, Point::new(0, 0), screen_max);
			self.set_point_to_buffer(screen_space_coordinate);
		}
	}
	fn set_point_to_buffer(&mut self, point: Point) {
		let point = Point::new(point.x.round_i(), point.y.round_i());
		let point = (point.x as usize, point.y as usize);
		if point.0 < self.size_px.0 && point.1 < self.size_px.1 {
			self.px_buffer[point.1 as usize][point.0 as usize] = true;
		}
	}
	pub fn render(&mut self) {
		for i in self.px_buffer.iter() {
			for j in i.iter() {
				if *j {
					print!("@")
				} else {
					print!(".")
				}
			}
			println!()
		}
	}
}
