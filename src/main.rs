use macroquad::prelude::*;

#[derive(Clone, Debug)]
struct Graph {
	vertex: Vec<char>,
	edges: Vec<(usize, usize)>,
	hash: u64,
}

impl Graph {
	fn update(&mut self) {
		rand::srand(self.hash);
		if is_key_pressed(KeyCode::Space) {
			self.hash ^= rand::gen_range(f64::MIN, f64::MAX).to_bits() ^ rand::rand() as u64;
		}
	}
	fn render(&self) {
		let float_rand = || rand::gen_range(0f32, 1f32);
		let line = |p1: (f32, f32), p2: (f32, f32)| {
			draw_line(
				screen_width() * p1.0,
				screen_height() * p1.1,
				screen_width() * p2.0,
				screen_height() * p2.1,
				1.,
				WHITE,
			);
		};
		let circle = |p: (f32, f32), color| {
			draw_circle(screen_width() * p.0, screen_height() * p.1, 50., color);
		};
		let character = |char, pos: (f32, f32)| {
			draw_text(
				format!("{char}").as_str(),
				pos.0 * screen_width(),
				pos.1 * screen_height(),
				30.,
				WHITE,
			)
		};
		let pos = self
			.vertex
			.iter()
			.map(|_| (float_rand(), float_rand()))
			.collect::<Vec<_>>();
		let color = self
			.vertex
			.iter()
			.map(|_| Color {
				r: float_rand(),
				g: float_rand(),
				b: float_rand(),
				a: 1.,
			})
			.collect::<Vec<_>>();
		self.edges
			.iter()
			.for_each(|vert_index| line(pos[vert_index.0], pos[vert_index.1]));
		pos.iter().zip(color).for_each(|(pos, color)| {
			circle(*pos, color);
		});
		pos.iter().enumerate().for_each(|(index, pos)| {
			character(self.vertex[index], *pos);
		});
	}
}

#[derive(Hash)]
struct GraphBuilder {
	vertex: Vec<char>,
	edges: Vec<(usize, usize)>,
}
impl GraphBuilder {
	pub fn new() -> Self {
		Self {
			vertex: Vec::new(),
			edges: Vec::new(),
		}
	}
	pub fn add_vertex(mut self, value: char) -> Self {
		if !self.vertex.contains(&value) {
			self.vertex.push(value);
		}
		self
	}
	pub fn add_edge(mut self, value1: char, value2: char) -> Self {
		self = self.add_vertex(value1);
		self = self.add_vertex(value2);
		let find_index = |value| {
			self.vertex
				.iter()
				.enumerate()
				.find(|(_, char)| **char == value)
				.map(|(index, _)| index)
				.unwrap()
		};
		let index1 = find_index(value1);
		let index2 = find_index(value2);
		println!("{index1}, {index2}");
		if !(self.edges.contains(&(index1, index2)) || self.edges.contains(&(index2, index1))) {
			println!("Added: {value1}, {value2}");
			self.edges.push((index1, index2))
		}
		self
	}
	pub fn build(self) -> Graph {
		use std::collections::hash_map::DefaultHasher;
		use std::hash::{Hash, Hasher};
		let mut hasher = DefaultHasher::default();
		self.hash(&mut hasher);
		let hash = hasher.finish();
		Graph {
			vertex: self.vertex,
			edges: self.edges,
			hash,
		}
	}
}

#[macroquad::main("susmos")]
async fn main() {
	let mut g = GraphBuilder::new()
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
	println!("{g:?}");
	loop {
		clear_background(BLACK);
		g.update();
		g.render();
		next_frame().await
	}
}
