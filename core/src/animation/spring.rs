#[derive(Debug, Clone, Copy)]
pub struct Spring {
    pub mass: f32,
    pub stiffness: f32,
    pub damping: f32,
    pub initial_velocity: f32,
}

impl Spring {
    pub fn new(mass: f32, stiffness: f32, damping: f32) -> Self {
        Spring {
            mass,
            stiffness,
            damping,
            initial_velocity: 0.0,
        }
    }

    pub fn with_velocity(mut self, v: f32) -> Self {
        self.initial_velocity = v;
        self
    }

    pub fn critical_damping() -> Self {
        Spring {
            mass: 1.0,
            stiffness: 171.0,
            damping: 26.0,
            initial_velocity: 0.0,
        }
    }

    pub fn under_damped() -> Self {
        Spring {
            mass: 1.0,
            stiffness: 100.0,
            damping: 10.0,
            initial_velocity: 0.0,
        }
    }

    pub fn is_critically_damped(&self) -> bool {
        let c = 2.0 * (self.stiffness * self.mass).sqrt();
        (self.damping - c).abs() < 0.0001
    }

    pub fn solve(&self, dt: f32, position: f32) -> f32 {
        let omega = (self.stiffness / self.mass).sqrt();
        let zeta = self.damping / (2.0 * (self.stiffness * self.mass).sqrt());
        if zeta < 1.0 {
            let wd = omega * (1.0 - zeta * zeta).sqrt();
            let exp = (-zeta * omega * dt).exp();
            let a = position;
            let b = (self.initial_velocity + zeta * omega * position) / wd;
            exp * (a * (wd * dt).cos() + b * (wd * dt).sin())
        } else if zeta == 1.0 {
            (-omega * dt).exp() * (position + (self.initial_velocity + omega * position) * dt)
        } else {
            let wd = omega * (zeta * zeta - 1.0).sqrt();
            let s1 = (-zeta * omega + wd) * dt;
            let s2 = (-zeta * omega - wd) * dt;
            let a = (self.initial_velocity + zeta * omega * position) / (2.0 * wd);
            let p1 = position + a;
            let p2 = position - a;
            p1 * s1.exp() + p2 * s2.exp()
        }
    }

    pub fn duration_to_settle(&self, tolerance: f32) -> f32 {
        let zeta = self.damping / (2.0 * (self.stiffness * self.mass).sqrt());
        let omega = (self.stiffness / self.mass).sqrt();
        if zeta >= 1.0 {
            -tolerance.ln() / omega
        } else {
            -tolerance.ln() / (zeta * omega)
        }
    }
}

impl Default for Spring {
    fn default() -> Self {
        Self::critical_damping()
    }
}
