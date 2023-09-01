//! Abstraction crate for viewport
//! Everything drawn to the viewport is normalized to a -1 to 1, Top-Right (1, 1) grid
use macroquad::prelude::*;

pub enum Shape {
	Circle { position: Vec2 },
}

pub struct RenderCall {
	pub shape: Shape,
	pub size: f32,
	pub color: Color,
}

pub trait ViewObject {
	fn update(&mut self) {}
	fn render(&self) -> RenderCall;
	fn clicked(&mut self, button: MouseButton);
	fn released(&mut self);
	fn hover(&mut self);
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
	fn to_screen_space(view_space: Vec2) -> Vec2 {
		Vec2 {
			x: (view_space.x + 1.) * screen_width() / 2.,
			y: (1. - view_space.y) * screen_height() / 2.,
		}
	}
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
				let render_call = render_object.render();
				match render_call.shape {
					Shape::Circle { position } => {
						let screen_coords =
							Viewport::<T>::to_screen_space(self.camera.to_view_space(position));
						draw_circle(
							screen_coords.x,
							screen_coords.y,
							render_call.size,
							render_call.color,
						);
					}
				}
			});
			next_frame().await
		}
	}
}
