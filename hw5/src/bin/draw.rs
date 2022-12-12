use std::fs;

use clap::{arg, Parser, ValueEnum};
use my_lib::{
    apis::{draw::Draw, simple_svg::SimpleSvg},
    drawing_api::DrawingApi,
    graph::{
        node_projectors::{ArchimedeanSpiralProjector, CircularProjector},
        traits::{DrawGraph, NodeProjector, ReadGraph},
    },
};
use petgraph::{matrix_graph::MatrixGraph, Graph, Undirected};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_enum, default_value_t = GraphType::List)]
    graph: GraphType,
    #[arg(short, long, value_enum, default_value_t = NodeProjectorType::Circle)]
    node_projector: NodeProjectorType,
    #[arg(short, long, value_enum)]
    draw_using: DrawApi,
    #[arg(short, long)]
    file: String,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum GraphType {
    Matrix,
    List,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum NodeProjectorType {
    Circle,
    Spiral,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum DrawApi {
    SimpleSVG,
    Draw,
}

// cargo run --bin draw -- -d simple-svg -f "resources/graphs/n_40_pr_0.2" -n circle
fn main() {
    let cli = Cli::parse();

    let node_projector: Box<dyn NodeProjector> = match cli.node_projector {
        NodeProjectorType::Circle => Box::new(CircularProjector),
        NodeProjectorType::Spiral => Box::new(ArchimedeanSpiralProjector::new(10.0, 1.4)),
    };

    let draw_api_get = match cli.draw_using {
        DrawApi::SimpleSVG => || -> Box<dyn DrawingApi> { Box::new(SimpleSvg::new()) },
        DrawApi::Draw => || -> Box<dyn DrawingApi> { Box::new(Draw::new()) },
    };

    let input = fs::read_to_string(cli.file).expect("Should have been able to read the file");

    let graph: Box<dyn DrawGraph<Box<dyn NodeProjector>>> = match cli.graph {
        GraphType::Matrix => Box::new(
            MatrixGraph::<(), (), Undirected, Option<()>, usize>::read_from_str(&input).unwrap(),
        ),
        GraphType::List => {
            Box::new(Graph::<(), (), Undirected, usize>::read_from_str(&input).unwrap())
        }
    };

    let output = "./tmp/file.svg".to_string();
    graph.draw_svg(draw_api_get, &node_projector, &output);
    open::that(&output).unwrap();
}
