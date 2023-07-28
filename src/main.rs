use macroquad::prelude::*;

#[derive(Clone, Debug)]
struct Graph {
	vertex: Vec<char>,
	edges: Vec<(usize, usize)>,
	weight_matrix: Vec<Vec<f32>>,
	hash: u64,
}

impl Graph {
	fn update(&mut self) {
		rand::srand(self.hash);
		if is_key_pressed(KeyCode::Space) {
			self.hash ^= rand::gen_range(f64::MIN, f64::MAX).to_bits() ^ rand::rand() as u64;
		}
		if is_key_pressed(KeyCode::A) {
			apply_dijkstra(self.weight_matrix.clone());
		}
	}
	fn render(&mut self) {
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
		let draw_character = |char, pos: (f32, f32)| {
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
			draw_character(self.vertex[index], *pos);
		});
		self.edges.iter().for_each(|(point_1, point_2)| {
			let pos_1 = pos[*point_1];
			let pos_2 = pos[*point_2];
			let weight = ((pos_1.0 - pos_2.0).powi(2) + (pos_1.1 - pos_2.1).powi(2)).sqrt();
			self.weight_matrix[*point_1][*point_2] = weight;
			self.weight_matrix[*point_2][*point_1] = weight;
			let mid_point = ((pos_1.0 + pos_2.0) / 2f32, (pos_1.1 + pos_2.1) / 2f32);
			draw_text(
				format!("{:.2}", weight).as_str(),
				mid_point.0 * screen_width(),
				mid_point.1 * screen_height(),
				30.,
				WHITE,
			)
		})
	}
}

fn apply_dijkstra(weight_matrix: Vec<Vec<f32>>) {
	let vertex_count = weight_matrix.len();
	let mut distances = vec![f32::INFINITY; vertex_count];
	distances[0] = 0f32;
	let mut from: Vec<Option<usize>> = vec![None; vertex_count];
	from[0] = Some(0);
	let mut visited = vec![false; vertex_count];
	visited[0] = true;
	for (dest_index, weight) in weight_matrix[0].iter().enumerate() {
		if *weight != 0f32 {
			distances[dest_index] = *weight;
			from[dest_index] = Some(0);
		}
	}
	loop {
		let unvisited_count: usize = visited.iter().map(|i| if *i { 0 } else { 1 }).sum();
		if unvisited_count == 0 {
			break;
		}
		let mut unvisited_nodes = distances
			.iter()
			.enumerate()
			.filter(|(node_index, _)| !visited[*node_index])
			.map(|(i, j)| (i, *j))
			.collect::<Vec<_>>();
		unvisited_nodes.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
		let (next_visit_index, next_visit_distance) = unvisited_nodes[0];
		println!("Next Visit: {next_visit_index}");
		println!("Visited: {visited:?}");
		visited[next_visit_index] = true;
		for (other_index, other_weight) in weight_matrix[next_visit_index].iter().enumerate() {
			if *other_weight != 0f32 {
				let distance_to_other_from_next_visit = next_visit_distance + other_weight;
				if distance_to_other_from_next_visit < distances[other_index] {
					distances[other_index] = distance_to_other_from_next_visit;
					from[other_index] = Some(next_visit_index);
				}
			}
		}
	}
	println!(
		"{:#.2?}",
		distances
			.iter()
			.zip(from.iter())
			.map(|(i, j)| (*i, *j))
			.collect::<Vec<(f32, Option<usize>)>>()
	)
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
		let vertex_count = self.vertex.len();
		Graph {
			vertex: self.vertex,
			edges: self.edges,
			hash,
			weight_matrix: vec![vec![0f32; vertex_count]; vertex_count],
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
		.add_edge('E', 'A')
		.add_edge('E', 'C')
		.add_edge('D', 'B')
		.add_edge('F', 'C')
		.build();
	loop {
		clear_background(BLACK);
		g.update();
		g.render();
		next_frame().await
	}
}
