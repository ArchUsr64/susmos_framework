mod view;
use view::*;

use macroquad::prelude::*;

#[derive(Clone, Copy, Debug)]
struct Vertex {
	name: char,
	color: Color,
	position: Vec2,
	radius: f32,
}

impl ViewObject for Vertex {
	fn update(&mut self) {}
	fn render(&self) {
		draw_circle(self.position.x, self.position.y, self.radius, self.color);
		draw_text(
			format!("{}", self.name).as_str(),
			self.position.x,
			self.position.y,
			20.,
			YELLOW,
		);
	}
	fn clicked(&mut self, button: MouseButton) {
		self.color = match button {
			MouseButton::Right => YELLOW,
			MouseButton::Left => ORANGE,
			MouseButton::Middle => BLUE,
			MouseButton::Unknown => RED,
		}
	}
	fn hover(&mut self) {
		self.color = GRAY;
	}
	fn collision_box(&self, mouse_position_normalized: Vec2) -> bool {
		let vec = self.position - mouse_position_normalized;
		vec.x * vec.x + vec.y * vec.y <= self.radius * self.radius
	}

	fn released(&mut self) {
		self.color = MAGENTA;
	}
}

#[macroquad::main("susmos")]
async fn main() {
	let vertex = Vertex {
		name: 'A',
		color: Color::from_rgba(28, 100, 255, 255),
		position: Vec2::new(500., 500.),
		radius: 200.,
	};
	let mut view = Viewport {
		renderables: vec![vertex],
	};
	view.run().await;
}
