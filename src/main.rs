mod grappher;
mod math;
mod plotter;
mod traits;
use grappher::Point;
use plotter::Plotter;

struct Ball {
	position: Point,
	velocity: Point,
	size: Point,
	speed: f32,
}

impl Ball {
	fn handle_collision(&mut self) {
		if self.position.x + self.size.x > 1f32 || self.position.x - self.size.x < -1f32 {
			self.velocity *= Point::new(-1f32, 1f32)
		}
		if self.position.y + self.size.y > 1f32 || self.position.y - self.size.y < -1f32 {
			self.velocity *= Point::new(1f32, -1f32)
		}
	}
	fn update_pos(&mut self) {
		self.handle_collision();
		self.position += self.velocity * self.speed;
	}
	fn render(&self, plotter: &mut Plotter) {
		print!("\x1B[2J\x1B[1;1H");
		plotter.clear();
		plotter.plot(self.position);
		plotter.render();
	}
}

fn main() {
	let full_screen_size = (455, 107);
	let mut plotter = Plotter::new(
		plotter::Pixel::new(full_screen_size.0, full_screen_size.1),
		Point::new(2, 2),
		Point::new(0, 0),
	);
	let mut ball = Ball {
		size: Point::new(0., 0.),
		speed: 0.2,
		position: Point::new(0, 0),
		velocity: Point::new(0.38, -0.21),
	};
	loop {
		std::thread::sleep(std::time::Duration::from_millis(30));
		ball.update_pos();
		ball.render(&mut plotter);
	}
}
