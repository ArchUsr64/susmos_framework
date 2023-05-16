use miniquad as mq;
struct App;
impl mq::EventHandler for App {
	fn update(&mut self, _ctx: &mut mq::Context) {}
	fn draw(&mut self, _ctx: &mut mq::Context) {}
}

fn main() {
	mq::start(mq::conf::Conf::default(), |_| Box::new(App));
}
