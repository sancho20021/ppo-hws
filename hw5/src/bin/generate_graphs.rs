use std::{fmt::Display, fs};

struct Graph {
    nodes: usize,
    edges: Vec<(usize, usize)>,
}

impl Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = "".to_string();
        out.push_str(format!("{}\n", self.nodes).as_str());
        out.push_str(format!("{}\n", self.edges.len()).as_str());
        for (a, b) in self.edges.iter() {
            out.push_str(format!("{} {}\n", a, b).as_str());
        }
        f.write_str(out.as_str())
    }
}

fn random_graph(size: usize, pr: f32) -> String {
    let mut graph = Graph {
        nodes: size,
        edges: vec![],
    };
    for i in 0..size {
        for j in 0..size {
            let coin = rand::random::<f32>();
            if coin < pr {
                graph.edges.push((i, j));
            }
        }
    }

    graph.to_string()
}

fn main() {
    for size in (5..50).filter(|n| n % 2 == 0) {
        for pr in [0.005, 0.01, 0.03, 0.05, 0.1, 0.2, 0.4, 0.8, 1.0] {
            let graph = random_graph(size, pr);
            let name = format!("n_{}_pr_{}", size, pr);
            fs::write(format!("resources/graphs/{}", name), graph).unwrap();
        }
    }
}
