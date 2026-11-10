use macroquad::math::Vec2;
use crate::angle::Angle;

pub struct Turtle {
    pub state: TurtleState,
    pub active: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct TurtleState {
    pub position: Vec2,
    pub rotation: Angle,
}

impl Default for TurtleState {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            rotation: Angle::ZERO,
        }
    }
}

impl Default for Turtle {
    fn default() -> Self {
        Self {
            state: TurtleState::default(),
            active: true,
        }
    }
}

impl Turtle {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn forward(&mut self, amount: f32) {
        let direction = Vec2::new(
            self.state.rotation.cos().to_radians(),
            self.state.rotation.sin().to_radians(),
        );

        self.state.position += direction * amount;
    }

    pub fn backward(&mut self, amount: f32) {
        let direction = Vec2::new(
            self.state.rotation.cos().to_radians(),
            self.state.rotation.sin().to_radians(),
        );

        self.state.position -= direction * amount;
    }

    pub fn step(&mut self) {
        self.forward(1.0);
    }

    pub fn right(&mut self, amount: Angle) {
        self.state.rotation += amount;
    }

    pub fn left(&mut self, amount: Angle) {
        self.state.rotation -= amount;
    }

    pub fn pen_down(&mut self) {
        self.active = true;
    }

    pub fn pen_up(&mut self) {
        self.active = false;
    }
}
