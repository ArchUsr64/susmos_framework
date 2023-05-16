use miniquad as mq;

struct App {
	egui_mq: egui_miniquad::EguiMq,
}

impl App {
	fn new(ctx: &mut mq::Context) -> Self {
		Self {
			egui_mq: egui_miniquad::EguiMq::new(ctx),
		}
	}
}

impl mq::EventHandler for App {
	fn update(&mut self, _: &mut mq::Context) {}

	fn draw(&mut self, mq_ctx: &mut mq::Context) {
		mq_ctx.clear(Some((1., 1., 1., 1.)), None, None);
		mq_ctx.begin_default_pass(mq::PassAction::clear_color(0.0, 0.0, 0.0, 1.0));
		mq_ctx.end_render_pass();

		self.egui_mq.run(mq_ctx, |_mq_ctx, egui_ctx| {
			egui::Window::new("Egui Window").show(egui_ctx, |ui| {
				ui.heading("Hello World!");
			});
		});

		// Draw things behind egui here

		self.egui_mq.draw(mq_ctx);

		// Draw things in front of egui here

		mq_ctx.commit_frame();
	}

	fn mouse_motion_event(&mut self, _: &mut mq::Context, x: f32, y: f32) {
		self.egui_mq.mouse_motion_event(x, y);
	}

	fn mouse_wheel_event(&mut self, _: &mut mq::Context, dx: f32, dy: f32) {
		self.egui_mq.mouse_wheel_event(dx, dy);
	}

	fn mouse_button_down_event(
		&mut self,
		ctx: &mut mq::Context,
		mb: mq::MouseButton,
		x: f32,
		y: f32,
	) {
		self.egui_mq.mouse_button_down_event(ctx, mb, x, y);
	}

	fn mouse_button_up_event(
		&mut self,
		ctx: &mut mq::Context,
		mb: mq::MouseButton,
		x: f32,
		y: f32,
	) {
		self.egui_mq.mouse_button_up_event(ctx, mb, x, y);
	}

	fn char_event(
		&mut self,
		_ctx: &mut mq::Context,
		character: char,
		_keymods: mq::KeyMods,
		_repeat: bool,
	) {
		self.egui_mq.char_event(character);
	}

	fn key_down_event(
		&mut self,
		ctx: &mut mq::Context,
		keycode: mq::KeyCode,
		keymods: mq::KeyMods,
		_repeat: bool,
	) {
		self.egui_mq.key_down_event(ctx, keycode, keymods);
	}

	fn key_up_event(&mut self, _ctx: &mut mq::Context, keycode: mq::KeyCode, keymods: mq::KeyMods) {
		self.egui_mq.key_up_event(keycode, keymods);
	}
}

fn main() {
	mq::start(mq::conf::Conf::default(), |ctx| Box::new(App::new(ctx)))
}
