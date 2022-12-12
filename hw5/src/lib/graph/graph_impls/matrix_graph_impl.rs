use std::collections::HashMap;

use petgraph::{
    matrix_graph::{MatrixGraph, Nullable},
    prelude::NodeIndex,
    stable_graph::IndexType,
    visit::IntoNodeIdentifiers,
    EdgeType, Undirected,
};

use crate::{
    drawing_api::{self, DrawingApi},
    graph::traits::{DrawGraph, NodeProjector, ReadGraph},
    utils,
};

use super::{read_graph, ConstructGraph};

impl<N, E, Ty, Ix, Null, NP> DrawGraph<NP> for MatrixGraph<N, E, Ty, Null, Ix>
where
    Ty: EdgeType,
    Ix: IndexType,
    Null: Nullable<Wrapped = E>,
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

impl ConstructGraph<NodeIndex<usize>> for MatrixGraph<(), (), Undirected, Option<()>, usize> {
    fn new(nodes: usize, _edges: usize) -> Self {
        MatrixGraph::with_capacity(nodes)
    }

    fn add_node(&mut self) -> NodeIndex<usize> {
        self.add_node(())
    }

    fn add_edge(&mut self, from: NodeIndex<usize>, to: NodeIndex<usize>) {
        self.add_edge(from, to, ());
    }
}

impl ReadGraph for MatrixGraph<(), (), Undirected, Option<()>, usize> {
    fn read_from_str(s: &String) -> Result<Self, String> {
        read_graph::<
            petgraph::matrix_graph::NodeIndex<usize>,
            MatrixGraph<(), (), Undirected, Option<()>, usize>,
        >(s)
    }
}

#[cfg(test)]
pub mod matrix_graph_tests {
    use petgraph::{
        matrix_graph::{MatrixGraph, NodeIndex},
        Undirected,
    };

    use super::super::common::test::*;
    use crate::graph::graph_impls::read_graph;

    #[test]
    pub fn read_triangle() {
        read_graph::<NodeIndex<usize>, MatrixGraph<(), (), Undirected, Option<()>, usize>>(
            &TRIANGLE.to_string(),
        )
        .unwrap();
    }
}
