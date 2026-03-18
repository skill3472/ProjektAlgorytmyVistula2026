use std::io::{self, Write};

struct Graph {
    edges: Vec<Vec<(usize, usize)>>,
}

impl Graph {
    fn new(num_vertices: usize) -> Graph {
        Graph {
            edges: vec![Vec::new(); num_vertices],
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, weight: usize) {
        self.edges[from].push((to, weight));
    }

    fn dijkstra(&self, start: usize) -> Vec<usize> {
        let num_vertices = self.edges.len();
        let mut distances = vec![usize::MAX; num_vertices];
        let mut visited = vec![false; num_vertices];

        distances[start] = 0;

        for _ in 0..num_vertices {
            let mut min_distance = usize::MAX;
            let mut current_vertex = 0;

            for v in 0..num_vertices {
                if !visited[v] && distances[v] < min_distance {
                    min_distance = distances[v];
                    current_vertex = v;
                }
            }

            if min_distance == usize::MAX {
                break;
            }

            visited[current_vertex] = true;

            for (neighbor, weight) in &self.edges[current_vertex] {
                if distances[current_vertex] + weight < distances[*neighbor] {
                    distances[*neighbor] = distances[current_vertex] + weight;
                }
            }
        }

        return distances;
    }
}

fn read_numbers() -> Vec<usize> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading input");
    input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn main() {
    println!("--- DIJKSTRA'S ALGORITHM ---");
    
    print!("Enter number of vertices: ");
    io::stdout().flush().unwrap();
    let num_vertices = read_numbers()[0];

    print!("Enter number of edges: ");
    io::stdout().flush().unwrap();
    let num_edges = read_numbers()[0];

    let mut graph = Graph::new(num_vertices);

    println!("Enter edges in format: [from] [to] [weight]");
    println!("(Vertices are numbered from 0 to {})", num_vertices - 1);
    
    for i in 0..num_edges {
        print!("Edge {}: ", i + 1);
        io::stdout().flush().unwrap();
        
        let nums = read_numbers();
        if nums.len() >= 3 {
            let from = nums[0];
            let to = nums[1];
            let weight = nums[2];

            if from < num_vertices && to < num_vertices {
                graph.add_edge(from, to, weight);
            } else {
                println!("Error: vertex index out of range");
            }
        } else {
            println!("Error: enter 3 numbers (e.g., 0 1 5)");
        }
    }

    print!("Enter starting vertex: ");
    io::stdout().flush().unwrap();
    let start = read_numbers()[0];

    if start >= num_vertices {
        println!("Error: invalid starting vertex");
        return;
    }

    let distances = graph.dijkstra(start);

    println!("\n--- RESULTS ---");
    println!("Shortest distances from vertex {}:", start);
    for (vertex, &dist) in distances.iter().enumerate() {
        if dist == usize::MAX {
            println!("To vertex {}: No path", vertex);
        } else {
            println!("To vertex {}: {}", vertex, dist);
        }
    }
}