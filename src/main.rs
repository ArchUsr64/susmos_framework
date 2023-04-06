#[derive(Clone, Debug)]
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

fn main() {
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
	println!("{g:?}");
}
