use std::collections::HashMap;

use petgraph::{
    prelude::NodeIndex, stable_graph::IndexType, visit::IntoNodeIdentifiers, EdgeType, Graph,
    Undirected,
};

use crate::{
    drawing_api::{self, DrawingApi},
    graph::traits::{DrawGraph, NodeProjector, ReadGraph},
    utils,
};

use super::{ConstructGraph, read_graph};

impl<N, E, Ty, Ix, NP> DrawGraph<NP> for Graph<N, E, Ty, Ix>
where
    Ty: EdgeType,
    Ix: IndexType,
    NP: NodeProjector,
{
    fn draw_svg(&self, get_api: fn() -> Box<dyn DrawingApi>, node_projector: &NP, file: &String) {
        let mut draw = get_api();
        let x_limits = (0, draw.get_area_width());
        let y_limits = (0, draw.get_area_height());

        let node_radius = std::cmp::min(x_limits.1, y_limits.1) / 4 / (self.node_count() as u32);

        let usize_indices = utils::argsort(self.node_identifiers().clone());
        let points = usize_indices
            .into_iter()
            .map(|(k, v)| {
                let (i, small) = (k, node_projector.project(v, self.node_count()));
                let big = drawing_api::scale(small.0, small.1, &x_limits, &y_limits);
                (i, big)
            })
            .collect::<HashMap<_, _>>();

        for (node, pos) in points.iter() {
            for neighbor in self.neighbors(*node) {
                let start = &points[node];
                let end = &points[&neighbor];
                draw.draw_line(start, end);
            }
            draw.draw_circle(pos, node_radius);
        }

        draw.export_svg(file);
    }
}

impl ConstructGraph<NodeIndex<usize>> for Graph<(), (), Undirected, usize> {
    fn new(nodes: usize, edges: usize) -> Self {
        Graph::with_capacity(nodes, edges)
    }

    fn add_node(&mut self) -> NodeIndex<usize> {
        self.add_node(())
    }

    fn add_edge(&mut self, from: NodeIndex<usize>, to: NodeIndex<usize>) {
        self.add_edge(from, to, ());
    }
}

impl ReadGraph for Graph<(), (), Undirected, usize> {
    fn read_from_str(s: &String) -> Result<Self, String> {
       read_graph::<petgraph::prelude::NodeIndex<usize>, Graph<(), (), Undirected, usize>>(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::Graph;

    use crate::graph::graph_impls::{
        common::test::{EMPTY_GRAPH, ONE_EDGE, TRIANGLE},
        read_graph,
    };

    #[test]
    pub fn read_empty() {
        read_graph::<NodeIndex<usize>, Graph<(), (), Undirected, usize>>(&EMPTY_GRAPH.to_string())
            .unwrap();
    }

    #[test]
    fn read_one() {
        read_graph::<NodeIndex<usize>, Graph<(), (), Undirected, usize>>(&ONE_EDGE.to_string())
            .unwrap();
    }

    #[test]
    fn read_triangle() {
        read_graph::<NodeIndex<usize>, Graph<(), (), Undirected, usize>>(&TRIANGLE.to_string())
            .unwrap();
    }
}
