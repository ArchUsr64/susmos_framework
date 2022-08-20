mod grappher;
mod math;
mod plotter;
mod traits;
use grappher::Point;
use plotter::Plotter;

fn main() {
	let mut plotter = Plotter::new(
		(10, 10), Point::new(2, 2), 
		Point::new(0, 0));
	plotter.plot(Point::new(0.5, 0.5));
	plotter.render();
	println!("------");
	plotter.plot(Point::new(0, 0));
	plotter.render();
	println!("------");
	plotter.plot(Point::new(-0.5, -0.5));
	plotter.render();
}
