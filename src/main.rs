mod view;
use macroquad::window::clear_background;
use view::*;
mod screen;
use screen::*;

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
		RenderCall::new(Shape::Circle(self.position), self.radius, self.color)
	}
	fn clicked(&mut self, button: MouseButton) {
		self.color = match button {
			MouseButton::Right => colors::YELLOW,
			MouseButton::Left => colors::ORANGE,
			MouseButton::Middle => colors::BLUE,
			MouseButton::Unknown => colors::RED,
		}
	}
	fn hover(&mut self) {
		self.color = colors::YELLOW;
	}
	fn no_hover(&mut self) {
		self.color = colors::ORANGE;
	}
	fn collision_box(&self, mouse_position_normalized: Vec2) -> bool {
		let vec = self.position - mouse_position_normalized;
		// Radius is multiplied by 4 cuz it's normalized to the range 0 to 1
		// whereas mouse_position_normalized is normalized to the range -1 to 1
		vec.x * vec.x + vec.y * vec.y <= self.radius * self.radius * 4.
	}

	fn released(&mut self) {
		self.color = colors::MAGENTA;
	}
}

#[macroquad::main("susmos")]
async fn main() {
	let vertex = Vertex {
		name: 'A',
		color: Color::from_rgba(28, 100, 255, 255),
		position: Vec2::new(0., 0.),
		radius: 0.5,
	};
	let mut view = Viewport {
		camera: view::Camera {
			position: Vec2::splat(0.),
			zoom: 1.,
		},
		renderables: vec![vertex],
	};
	loop {
		clear_background(colors::BLACK);
		view.render();
		next_frame().await;
	}
}
