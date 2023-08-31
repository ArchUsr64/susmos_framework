//! Abstraction crate for viewport
//! Everything drawn to the viewport is normalized to a -100 to 100, Top-Right (100, 100) grid
use macroquad::prelude::*;

pub trait ViewObject {
	fn update(&mut self) {}
	fn render(&self);
	fn clicked(&mut self, button: MouseButton);
	fn released(&mut self);
	fn hover(&mut self);
	fn collision_box(&self, mouse_position_normalized: Vec2) -> bool;
}

pub struct Viewport<T>
where
	T: ViewObject,
{
	pub renderables: Vec<T>,
}

impl<T: ViewObject> Viewport<T> {
	pub async fn run(&mut self) {
		loop {
			clear_background(BLACK);
			let mouse_location = Vec2::from(mouse_position());
			self.renderables.iter_mut().for_each(|render_object| {
				let mouse_over_object = render_object.collision_box(mouse_location);
				if mouse_over_object {
					render_object.hover()
				};
				render_object.update();
				render_object.render();
			});
			next_frame().await
		}
	}
}
