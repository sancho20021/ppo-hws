use std::{collections::HashMap, str::FromStr};

use super::traits::ConstructGraph;

pub mod list_graph_impl;
pub mod matrix_graph_impl;

/// Format of parsing:
/// <nodes_number>
/// <edges_number>
/// <edge1>
/// <edge2>
/// ... where <edge_i> = "<from> <to>" (without quotes)
pub fn read_graph<Ix, G>(s: &String) -> Result<G, String>
where
    Ix: Clone,
    G: ConstructGraph<Ix>,
{
    let lines = s.split("\n").filter(|s| !s.is_empty()).collect::<Vec<_>>();
    (|| {
        let n: usize = lines[0].parse()?;
        let m: usize = lines[1].parse()?;
        let mut graph = G::new(n, m);

        let edges = lines
            .into_iter()
            .skip(2)
            .map(|line| {
                line.split(" ")
                    .map(|s| s.parse::<usize>())
                    .collect::<Result<Vec<_>, _>>()
                    .map(|vec| (vec[0], vec[1]))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let mut nodes = HashMap::<usize, _>::default();
        for edge in edges.iter() {
            if !nodes.contains_key(&edge.0) {
                nodes.insert(edge.0, graph.add_node());
            }
            if !nodes.contains_key(&edge.1) {
                nodes.insert(edge.1, graph.add_node());
            }
            graph.add_edge(nodes[&edge.0].clone(), nodes[&edge.1].clone());
        }

        Ok(graph)
    })()
    .map_err(|e: <usize as FromStr>::Err| format!("{}", e))
}

mod common {
    #[cfg(test)]
    pub mod test {
        pub const EMPTY_GRAPH: &str = "0\n0\n";
        pub const ONE_EDGE: &str = "2\n1\n1 2";
        pub const TRIANGLE: &str = "3\n3\n1 2\n2 3\n1 3";
    }
}
