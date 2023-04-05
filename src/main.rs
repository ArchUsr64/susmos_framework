use macroquad::prelude::*;

struct Graph {
	vertex: Vec<char>,
	edges: Vec<(usize, usize)>,
}

struct GraphBuilder {
	graph: Graph,
}
impl GraphBuilder {
	pub fn new() -> Self {
		Self {
			graph: Graph {
				vertex: Vec::new(),
				edges: Vec::new(),
			},
		}
	}
	pub fn add_vertex(mut self, value: char) -> Self {
		if !self.graph.vertex.contains(&value) {
			self.graph.vertex.push(value);
		}
		self
	}
	pub fn add_edge(mut self, value1: char, value2: char) -> Self {
		self = self.add_vertex(value1);
		self = self.add_vertex(value2);
		let find_index = |value| {
			self.graph
				.vertex
				.iter()
				.enumerate()
				.find(|(index, char)| **char == value)
				.map(|(index, _)| index)
				.unwrap()
		};
		let index1 = find_index(value1);
		let index2 = find_index(value2);
		println!("{index1}, {index2}");
		if !(self.graph.edges.contains(&(index1, index2))
			|| self.graph.edges.contains(&(index2, index1)))
		{
			println!("Added: {value1}, {value2}");
			self.graph.edges.push((index1, index2))
		}
		self
	}
	pub fn build(self) -> Graph {
		self.graph
	}
}

#[macroquad::main("BasicShapes")]
async fn main() {
	let g = GraphBuilder::new()
		.add_vertex('A')
		.add_vertex('B')
		.add_vertex('C')
		.add_vertex('D')
		.add_vertex('E')
		.add_vertex('F')
		.add_edge('A', 'B')
		.add_edge('C', 'B')
		.add_edge('E', 'A')
		.add_edge('E', 'C')
		.add_edge('D', 'B')
		.add_edge('F', 'C')
		.build();
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
