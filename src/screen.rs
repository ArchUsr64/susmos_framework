//! Abstraction for the screenspace
//! Accepts -1 to 1 normalized coordinates form View and renders them to screen
//! Re-exports useful types from macroquad

pub use macroquad::color::colors;
pub use macroquad::prelude::{next_frame, Color, MouseButton, Vec2};

use macroquad::prelude::*;

#[derive(Debug)]
pub enum Shape {
	Circle(Vec2),
	Line(Vec2, Vec2),
	Text(String, Vec2),
	Point(Vec2),
}

#[derive(Debug)]
pub struct RenderCall {
	shape: Shape,
	size: f32,
	color: Color,
}

pub mod Screen {
	use macroquad::prelude::*;
	pub fn normalized_mouse_position() -> Vec2 {
		let mouse_position = Vec2::from(mouse_position());
		Vec2 {
			x: (mouse_position.x * 2. / screen_width()) - 1.,
			y: 1. - (mouse_position.y * 2. / screen_height()),
		}
	}
	pub fn draw_point(position: Vec2, color: Color) {
		draw_line(
			position.x,
			position.y,
			position.x + 1.,
			position.y,
			1.,
			color,
		);
	}
	pub fn draw_ellipse(position: Vec2, size: Vec2, color: Color) {
		let unsigned_distance_field = |point: Vec2| -> f32 {
			let mut point = point - position;
			point /= size;
			(point.x.powi(2) + point.y.powi(2) - 1.).abs()
		};
		let a = size.x;
		let b = size.y;
		let mut dx = size.x;
		let mut dy = 0.;
		let slope_inflection_point_ordinate = -(b.powi(2) / (1. + (a / b).powi(2))).sqrt();
		while dy >= slope_inflection_point_ordinate {
			draw_point(position + Vec2::new(dx, dy), color);
			draw_point(position + Vec2::new(-dx, dy), color);
			draw_point(position + Vec2::new(dx, -dy), color);
			draw_point(position + Vec2::new(-dx, -dy), color);
			let cost_above = unsigned_distance_field(position + Vec2::new(dx, dy - 1.));
			let cost_above_left = unsigned_distance_field(position + Vec2::new(dx - 1., dy - 1.));
			if cost_above_left < cost_above {
				dx -= 1.;
			}
			dy -= 1.;
		}
		let mut dx = 0.;
		let mut dy = size.y;
		let slope_inflection_point_abcissa = (a.powi(2) / (1. + (b / a).powi(2))).sqrt();
		while dx <= slope_inflection_point_abcissa {
			draw_point(position + Vec2::new(dx, dy), color);
			draw_point(position + Vec2::new(-dx, dy), color);
			draw_point(position + Vec2::new(dx, -dy), color);
			draw_point(position + Vec2::new(-dx, -dy), color);
			let cost_right = unsigned_distance_field(position + Vec2::new(dx + 1., dy));
			let cost_below_right = unsigned_distance_field(position + Vec2::new(dx + 1., dy - 1.));
			if cost_below_right < cost_right {
				dy -= 1.;
			}
			dx += 1.;
		}
		draw_point(position + Vec2::new(dx, dy), color);
		draw_point(position + Vec2::new(-dx, dy), color);
		draw_point(position + Vec2::new(dx, -dy), color);
		draw_point(position + Vec2::new(-dx, -dy), color);
	}
}

impl RenderCall {
	pub fn new(shape: Shape, size: f32, color: Color) -> Self {
		Self { shape, size, color }
	}
	pub fn render(&self) {
		let to_screen_space = |view_space: Vec2| -> Vec2 {
			Vec2 {
				x: (view_space.x + 1.) * screen_width() / 2.,
				y: (1. - view_space.y) * screen_height() / 2.,
			}
		};
		let screen_space_scalar = |size: f32| -> f32 { screen_height().min(screen_width()) * size };
		let Self { shape, color, .. } = self;
		let size = screen_space_scalar(self.size);
		match shape {
			Shape::Circle(position) => {
				let center = to_screen_space(*position);
				Screen::draw_ellipse(
					center,
					self.size * Vec2::new(screen_width(), screen_height()),
					*color,
				);
			}
			Shape::Text(text, position) => {
				let position = to_screen_space(*position);
				// Using size directly here is probably not correct due to 2:1
				// aspect ratio of monospaced characters
				draw_text(text.as_str(), position.x, position.y, size, *color);
			}
			_ => (),
		}
	}
}
