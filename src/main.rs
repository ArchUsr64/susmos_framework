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
	fn render(&self) -> RenderCall {
		RenderCall {
			shape: Shape::Circle {
				position: self.position,
			},
			size: self.radius,
			color: self.color,
		}
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
		position: Vec2::new(0., 0.),
		radius: 25.2,
	};
	let mut view = Viewport {
		camera: view::Camera {
			position: Vec2::splat(0.),
			zoom: 1.,
		},
		renderables: vec![vertex],
	};
	view.run().await;
}
