use macroquad::prelude::*;
pub const F: bool = false;
pub const T: bool = true;

struct Graph {
	vertex: Vec<char>,
	edges: Vec<(usize, usize)>,
}

#[macroquad::main("BasicShapes")]
async fn main() {
	let g = Graph {
		vertex: vec!['A', 'B', 'C', 'D', 'E', 'F'],
		edges: vec![(0, 1), (2, 1), (4, 0), (4, 2), (3, 1), (5, 2)],
	};
	let positions = g
		.vertex
		.iter()
		.map(|_| (rand::gen_range(0., 1.), rand::gen_range(0., 1.)))
		.collect::<Vec<_>>();
	loop {
		let (width, height) = (screen_width(), screen_height());
		let positions = positions
			.iter()
			.map(|value| (value.0 * width, value.1 * height))
			.collect::<Vec<_>>();
		g.edges.iter().for_each(|(node1, node2)| {
			draw_line(
				positions[*node1].0,
				positions[*node1].1,
				positions[*node2].0,
				positions[*node2].1,
				1.,
				RED,
			);
		});
		g.vertex.iter().enumerate().for_each(|(index, value)| {
			let x = positions[index].0;
			let y = positions[index].1;
			let text_size = 50.;
			draw_circle(x, y, 25., GRAY);
			draw_text(
				value.to_string().as_str(),
				x - text_size / 4.,
				y + text_size / 4.,
				text_size,
				WHITE,
			);
		});
		next_frame().await
	}
}
