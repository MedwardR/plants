use crate::angle::Angle;
use macroquad::math::Vec2;

pub struct Turtle {
    pub state: TurtleState,
    pub stack: Vec<TurtleState>,
    pub polygon: Vec<Vec2>,
    pub pen_active: bool,
    pub polygon_active: bool,
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
            stack: Vec::new(),
            polygon: Vec::new(),
            pen_active: true,
            polygon_active: false,
        }
    }
}

impl Turtle {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn forward(&mut self, amount: f32) {
        let direction = Vec2::new(
            self.state.rotation.cos().radians(),
            self.state.rotation.sin().radians(),
        );

        self.state.position += direction * amount;
    }

    pub fn backward(&mut self, amount: f32) {
        self.forward(-amount);
    }

    pub fn step(&mut self) {
        self.forward(1.0);
    }

    pub fn left(&mut self, amount: Angle) {
        let value = self.state.rotation - amount;
        self.state.rotation = value % Angle::TAU;
    }

    pub fn right(&mut self, amount: Angle) {
        let value = self.state.rotation + amount;
        self.state.rotation += value % Angle::TAU;
    }

    pub fn pen_down(&mut self) {
        self.pen_active = true;
    }

    pub fn pen_up(&mut self) {
        self.pen_active = false;
    }

    pub fn start_branch(&mut self) {
        self.stack.push(self.state);
    }

    pub fn end_branch(&mut self) {
        if let Some(previous) = self.stack.pop() {
            self.state = previous;
        }
    }

    pub fn start_polygon(&mut self) {
        self.polygon = Vec::new();
        self.polygon_active = true;
    }

    pub fn end_polygon(&mut self) {
        self.polygon_active = false;
    }
    
    pub fn record_vertex(&mut self) {
        if self.polygon_active {
            self.polygon.push(self.state.position);
        }
    }
}
