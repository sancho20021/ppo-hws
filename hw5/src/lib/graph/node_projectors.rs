use std::f32::consts::PI;

use super::traits::NodeProjector;

pub struct CircularProjector;
impl NodeProjector for CircularProjector {
    fn project(&self, i: usize, nodes: usize) -> (f32, f32) {
        let angle: f32 = 2.0 * PI * i as f32 / nodes as f32;
        (angle.cos(), angle.sin())
    }
}

pub struct ArchimedeanSpiralProjector {
    pub a: f32,
    pub angle_multiplier: f32,
    radius_divisor: f32,
}

impl ArchimedeanSpiralProjector {
    pub fn new(a: f32, angle_multiplier: f32) -> ArchimedeanSpiralProjector {
        ArchimedeanSpiralProjector {
            a,
            angle_multiplier,
            radius_divisor: a + 2.0 * PI * angle_multiplier,
        }
    }
}

impl NodeProjector for ArchimedeanSpiralProjector {
    fn project(&self, i: usize, nodes: usize) -> (f32, f32) {
        let angle: f32 = 2.0 * PI * i as f32 / nodes as f32 * self.angle_multiplier;
        let radius = (self.a + angle) / self.radius_divisor;
        (radius * angle.cos(), radius * angle.sin())
    }
}
