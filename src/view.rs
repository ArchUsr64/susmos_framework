//! Abstraction crate for viewport
//! Everything drawn to the viewport is normalized to a -1 to 1, Top-Right (1, 1) grid
use crate::screen::*;

pub trait ViewObject {
	fn update(&mut self) {}
	fn render(&self) -> RenderCall;
	fn clicked(&mut self, button: MouseButton);
	fn released(&mut self);
	fn hover(&mut self);
	fn no_hover(&mut self);
	fn collision_box(&self, mouse_position_normalized: Vec2) -> bool;
}

pub struct Camera {
	pub position: Vec2,
	pub zoom: f32,
}
impl Camera {
	fn to_view_space(&self, world_space: Vec2) -> Vec2 {
		world_space - self.position
	}
}

pub struct Viewport<T>
where
	T: ViewObject,
{
	pub camera: Camera,
	pub renderables: Vec<T>,
}

impl<T: ViewObject> Viewport<T> {
	pub fn render(&mut self) {
		let mouse_location = Screen::normalized_mouse_position();
		self.renderables.iter_mut().for_each(|render_object| {
			let mouse_over_object = render_object.collision_box(mouse_location);
			if mouse_over_object {
				render_object.hover()
			} else {
				render_object.no_hover()
			}
			render_object.update();
			let render_call = render_object.render();
			render_call.render();
		});
	}
}
