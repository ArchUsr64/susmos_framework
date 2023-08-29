use macroquad::prelude::*;

#[derive(Clone, Debug)]
struct Graph {
	vertex: Vec<char>,
	edges: Vec<(usize, usize)>,
	weight_matrix: Vec<Vec<f32>>,
	hash: u64,
}

enum Traversal {
	DepthFirst,
	BreadthFirst,
}

/// TODO: Add support for custom starting vertex
fn traverse(weight_matrix: Vec<Vec<f32>>, order: Traversal) -> Vec<usize> {
	let vertex_count = weight_matrix.len();
	let mut adjacency_matrix = vec![vec![false; vertex_count]; vertex_count];
	let mut result = vec![];
	for (i, row) in adjacency_matrix.iter_mut().enumerate() {
		for (j, connected) in row.iter_mut().enumerate() {
			*connected = weight_matrix[i][j] != 0f32;
		}
	}
	let mut visiting_list = vec![0];
	let mut visited = vec![false; vertex_count];
	visited[0] = true;
	loop {
		if visiting_list.is_empty() {
			break;
		}
		let visiting = match order {
			Traversal::DepthFirst => visiting_list.pop().unwrap(),
			Traversal::BreadthFirst => visiting_list.remove(0),
		};
		visited[visiting] = true;
		result.push(visiting);
		for (vertex_index, connected) in adjacency_matrix[visiting].iter().enumerate() {
			if *connected && !visited[vertex_index] {
				visiting_list.push(vertex_index);
			}
		}
	}
	result
}

impl Graph {
	fn update(&mut self) {
		rand::srand(self.hash);
		if is_key_pressed(KeyCode::Space) {
			self.hash ^= rand::gen_range(f64::MIN, f64::MAX).to_bits() ^ rand::rand() as u64;
		}
		let print_ordering = |name: &'static str, ordering: Vec<usize>| {
			println!(
				"{name} : {}",
				ordering
					.iter()
					.map(|index| format!("{}->", self.vertex[*index]))
					.collect::<String>()
			);
		};
		if is_key_pressed(KeyCode::F1) {
			print_ordering(
				"BFS",
				traverse(self.weight_matrix.clone(), Traversal::BreadthFirst),
			);
		}
		if is_key_pressed(KeyCode::F2) {
			print_ordering(
				"DFS",
				traverse(self.weight_matrix.clone(), Traversal::DepthFirst),
			);
		}
		if is_key_pressed(KeyCode::Tab) {
			apply_bellman_ford(self.weight_matrix.clone());
		}
		match get_char_pressed() {
			Some(pressed_char) if self.vertex.contains(&pressed_char) => {
				let (vertex_index, vertex_char_code) = self
					.vertex
					.iter()
					.enumerate()
					.find(|(_, vertex_char)| **vertex_char == pressed_char)
					.unwrap();
				// TODO: Remove the need for this clone
				let dijkstra_result = apply_dijkstra(vertex_index, self.weight_matrix.clone());
				println!("Start vertex: {vertex_char_code}");
				println!("Destination\tDistance\tPrevious Vertex");
				for (destination_vertex_index, (distance, previous_vertex)) in
					dijkstra_result.iter().enumerate()
				{
					println!(
						"{}\t\t{:.2}\t\t{}",
						self.vertex[destination_vertex_index],
						distance,
						match previous_vertex {
							Some(index) => String::from(self.vertex[*index]),
							None => String::from("No Path"),
						}
					)
				}
				println!();
			}
			_ => (),
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

fn apply_bellman_ford(weight_matrix: Vec<Vec<f32>>) {
	let mut weight_matrix = weight_matrix.clone();
	for (i, row) in weight_matrix.iter_mut().enumerate() {
		for (j, edge) in row.iter_mut().enumerate() {
			if i != j && *edge == 0f32 {
				*edge = f32::MAX;
			}
		}
	}
	let vertex_count = weight_matrix.len();
	(0..vertex_count).for_each(|k| {
		(0..vertex_count).for_each(|i| {
			(0..vertex_count).for_each(|j| {
				weight_matrix[i][j] =
					weight_matrix[i][j].min(weight_matrix[i][k] + weight_matrix[k][j])
			})
		})
	});
	println!("{weight_matrix:#.2?}");
}

fn apply_dijkstra(start_index: usize, weight_matrix: Vec<Vec<f32>>) -> Vec<(f32, Option<usize>)> {
	let vertex_count = weight_matrix.len();
	let mut distances = vec![f32::INFINITY; vertex_count];
	distances[start_index] = 0f32;
	let mut from: Vec<Option<usize>> = vec![None; vertex_count];
	from[start_index] = Some(start_index);
	let mut visited = vec![false; vertex_count];
	visited[start_index] = true;
	for (dest_index, weight) in weight_matrix[start_index].iter().enumerate() {
		if *weight != 0f32 {
			distances[dest_index] = *weight;
			from[dest_index] = Some(start_index);
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
	distances
		.iter()
		.zip(from.iter())
		.map(|(i, j)| (*i, *j))
		.collect::<Vec<(f32, Option<usize>)>>()
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
