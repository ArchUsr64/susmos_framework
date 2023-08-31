use macroquad::prelude::*;

trait ScreenObject {
	fn update(&mut self);
	fn render(&self);
	fn clicked(&self) -> Option<MouseButton>;
}

#[derive(Clone, Copy, Debug)]
struct Vertex {
	name: char,
	color: Color,
	position: Vec2,
	radius: f32,
}

impl ScreenObject for Vertex {
	fn update(&mut self) {
		if let Some(MouseButton::Middle) = self.clicked() {
			self.position = Vec2::new(mouse_position().0, mouse_position().1);
		}
	}
	fn render(&self) {
		draw_circle(
			self.position.x,
			self.position.y,
			self.radius,
			if self.clicked().is_some() {
				Color {
					r: self.color.r * 1.5,
					g: self.color.g * 1.5,
					b: self.color.b * 1.5,
					a: self.color.a * 1.5,
				}
			} else {
				self.color
			},
		);
		draw_text(
			format!("{}", self.name).as_str(),
			self.position.x,
			self.position.y,
			20.,
			YELLOW,
		);
	}
	fn clicked(&self) -> Option<MouseButton> {
		let cursor_position = mouse_position();
		let cursor_inside = ((cursor_position.0 - self.position.x).powi(2)
			+ (cursor_position.1 - self.position.y).powi(2))
			< self.radius * self.radius;
		if !cursor_inside {
			return None;
		}
		let check_if_pressed = |button| {
			if is_mouse_button_down(button) {
				Some(button)
			} else {
				None
			}
		};
		check_if_pressed(MouseButton::Left).or(check_if_pressed(MouseButton::Right)
			.or(check_if_pressed(MouseButton::Middle).or(check_if_pressed(MouseButton::Unknown))))
	}
}

#[macroquad::main("susmos")]
async fn main() {
	let mut vertex = Vertex {
		name: 'A',
		color: Color::from_rgba(28, 100, 255, 255),
		position: Vec2::new(500., 500.),
		radius: 200.,
	};
	loop {
		clear_background(BLACK);
		vertex.update();
		vertex.render();
		next_frame().await
	}
}
