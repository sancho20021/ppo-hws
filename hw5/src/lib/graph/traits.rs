use crate::drawing_api::DrawingApi;

pub trait DrawGraph<NP: NodeProjector> {
    fn draw_svg(&self, get_api: fn() -> Box<dyn DrawingApi>, node_projector: &NP, file: &String);
}

/// Projects nodes on float plane with axis of length 1.0
pub trait NodeProjector {
    fn project(&self, i: usize, nodes: usize) -> (f32, f32);
}

impl NodeProjector for Box<dyn NodeProjector> {
    fn project(&self, i: usize, nodes: usize) -> (f32, f32) {
        self.as_ref().project(i, nodes)
    }
}

pub trait ConstructGraph<Ix: Clone> {
    fn new(nodes: usize, edges: usize) -> Self;
    fn add_node(&mut self) -> Ix;
    fn add_edge(&mut self, from: Ix, to: Ix);
}

pub trait ReadGraph: Sized {
    fn read_from_str(s: &String) -> Result<Self, String>;
}
