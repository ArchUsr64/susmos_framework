use crate::{math::Math, traits::ToF32, traits::ToUsize};
use std::{fmt::Display, vec};

use crate::grappher::Point;

#[derive(Clone, Copy)]
pub struct Pixel {
	pub x: usize,
	pub y: usize,
}
impl Display for Pixel {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "[X: {}, Y: {}]", self.x, self.y)
	}
}
impl Pixel {
	pub fn new<T: ToUsize>(x: T, y: T) -> Pixel {
		Pixel {
			x: x.to_usize().unwrap_or(0),
			y: y.to_usize().unwrap_or(0),
		}
	}
	pub fn to_point(&self) -> Point {
		Point::new(self.x, self.y)
	}
	pub fn to_point_space(&self, size_px: Pixel, size_pt: Point, centre: Point) -> Point {
		let low_bound = (centre - (size_pt / 2f32)) * Point::new(1f32, -1f32);
		let high_bound = low_bound + (size_pt * Point::new(1, -1));
		self.to_point()
			.lerp(Point::new(0, 0), self.to_point(), low_bound, high_bound)
	}
}

pub struct Plotter {
	pub size_px: Pixel,
	pub size_pt: Point,
	pub centre: Point,
	px_buffer: Vec<Vec<char>>,
}

impl Plotter {
	pub fn new(size_px: Pixel, size_pt: Point, centre: Point) -> Plotter {
		Plotter {
			size_px,
			size_pt,
			centre: centre * Point::new(1, -1),
			px_buffer: vec![vec![' '; size_px.x]; size_px.y],
		}
	}
	fn render_axis(&mut self) {
		let aspect_ratio = self.size_pt.x / self.size_pt.y;
		let label_count = Point::new(10f32 * aspect_ratio, 10f32);
		let label_gap = (self.size_pt / label_count).goodify();
		let origin = Point::new(0, 0).to_pixel_space(self.size_px, self.size_pt, self.centre);
		let lowest_bound = self.centre - self.size_pt;
		if origin.x < self.size_px.x && origin.x > 0 {
			for i in 0..self.size_px.y {
				self.set_pixel_to_buffer(Pixel::new(origin.x, i), '|');
			}
			for i in 0..label_count.y.round_i() {
				self.set_point_to_buffer(
					Point::new(0f32, lowest_bound.y + ((i as f32) * label_gap.y)),
					'k',
				);
			}
		}
		if origin.y < self.size_px.y && origin.y > 0 {
			for i in 0..self.size_px.x {
				self.set_pixel_to_buffer(Pixel::new(i, origin.y), '-');
			}
		}
	}
	fn centered_point_in_draw_area(&self, point: Point) -> bool {
		self.size_pt.x.abs() > point.x.abs() || self.size_pt.y.abs() > point.y.abs()
	}
	pub fn clear(&mut self) {
		for i in self.px_buffer.iter_mut() {
			for j in i.iter_mut() {
				*j = ' ';
			}
		}
	}
	pub fn plot(&mut self, point: Point) {
		let pixel = point.to_pixel_space(self.size_px, self.size_pt, self.centre);
		self.set_pixel_to_buffer(pixel, 'x');
	}
	fn set_pixel_to_buffer(&mut self, pixel: Pixel, label: char) {
		if pixel.x < self.size_px.x && pixel.y < self.size_px.y {
			self.px_buffer[pixel.y][pixel.x] = label;
		}
	}
	fn set_point_to_buffer(&mut self, point: Point, label: char) {
		let pixel = point.to_pixel_space(self.size_px, self.size_pt, self.centre);
		self.set_pixel_to_buffer(pixel, label);
	}
	fn render_horizontal_border(&self) {
		for _i in 0..self.size_px.x {
			print!("=");
		}
		println!();
	}
	pub fn render(&mut self) {
		self.render_axis();
		self.render_horizontal_border();
		for (m, i) in self.px_buffer.iter().enumerate() {
			if m == 0 || m == self.size_px.y - 1 {
				continue;
			}
			for (i, val) in i.iter().enumerate() {
				if i == self.size_px.x - 1 {
					print!("|");
					continue;
				}
				match i {
					0 => print!("|"),
					_ => print!("{}", val),
				}
			}
			println!()
		}
		self.render_horizontal_border();
	}
}
