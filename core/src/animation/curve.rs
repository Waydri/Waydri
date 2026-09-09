#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Curve {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    CubicBezier { p1: (f32, f32), p2: (f32, f32) },
    Bounce,
    Elastic,
    Spring { mass: f32, stiffness: f32, damping: f32 },
}

impl Curve {
    pub fn apply(self, t: f32) -> f32 {
        match self {
            Curve::Linear => t,
            Curve::EaseIn => t * t,
            Curve::EaseOut => 1.0 - (1.0 - t) * (1.0 - t),
            Curve::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
                }
            }
            Curve::CubicBezier { p1, p2 } => cubic_bezier(t, p1, p2),
            Curve::Bounce => bounce(t),
            Curve::Elastic => elastic(t),
            Curve::Spring { mass, stiffness, damping } => {
                let omega = (stiffness / mass).sqrt();
                let zeta = damping / (2.0 * (stiffness * mass).sqrt());
                if zeta < 1.0 {
                    let wd = omega * (1.0 - zeta * zeta).sqrt();
                    1.0 - (-zeta * omega * t).exp() * (wd * t).cos()
                } else {
                    1.0 - (-omega * t).exp()
                }
            }
        }
    }
}

impl Default for Curve {
    fn default() -> Self {
        Curve::EaseInOut
    }
}

fn cubic_bezier(t: f32, p1: (f32, f32), p2: (f32, f32)) -> f32 {
    let u = 1.0 - t;
    let c0 = u * u * u;
    let c1 = 3.0 * u * u * t;
    let c2 = 3.0 * u * t * t;
    let c3 = t * t * t;
    c0 * 0.0 + c1 * p1.1 + c2 * p2.1 + c3 * 1.0
}

fn bounce(t: f32) -> f32 {
    let n1 = 7.5625;
    let d1 = 2.75;
    if t < 1.0 / d1 {
        n1 * t * t
    } else if t < 2.0 / d1 {
        let t = t - 1.5 / d1;
        n1 * t * t + 0.75
    } else if t < 2.5 / d1 {
        let t = t - 2.25 / d1;
        n1 * t * t + 0.9375
    } else {
        let t = t - 2.625 / d1;
        n1 * t * t + 0.984375
    }
}

fn elastic(t: f32) -> f32 {
    let c4 = (2.0 * std::f32::consts::PI) / 3.0;
    if t == 0.0 {
        0.0
    } else if t == 1.0 {
        1.0
    } else {
        -(2.0f32.powf(10.0 * t - 10.0) * ((t * 10.0 - 10.75) * c4).sin())
    }
}
