use std::ops::{Add, AddAssign, Sub, SubAssign};
use macroquad::math::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct Angle {
    value: f32
}

#[derive(Debug, Clone, Copy)]
pub struct TurtleState {
    pub position: Vec2,
    pub rotation: Angle,
}

pub struct Turtle {
    pub state: TurtleState,
    pub active: bool,
}

pub struct Interpreter {
    pub movement_step: f32,
    pub rotation_step: Angle,
    stack: Vec<TurtleState>,
}

impl Add for Angle {
    type Output = Angle;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: self.value + rhs.value
        }
    }
}

impl Sub for Angle {
    type Output = Angle;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: self.value - rhs.value
        }
    }
}

impl AddAssign for Angle {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
    }
}

impl SubAssign for Angle {
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value;
    }
}

impl Angle {
    pub const ZERO: Angle = Angle { value: 0.0 };

    pub const fn from_degrees(degrees: f32) -> Self {
        Self {
            value: degrees.to_radians(),
        }
    }

    pub const fn from_radians(radians: f32) -> Self {
        Self {
            value: radians,
        }
    }

    pub fn to_degrees(&self) -> f32 {
        self.value.to_degrees()
    }

    pub fn to_radians(&self) -> f32 {
        self.value
    }

    pub fn sin(&self) -> Self {
        Self {
            value: self.value.sin()
        }
    }

    pub fn cos(&self) -> Self {
        Self {
            value: self.value.cos()
        }
    }
}

impl Default for TurtleState {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            rotation: Angle::ZERO,
        }
    }
}

impl Turtle {
    pub const fn new(state: TurtleState) -> Self {
        Self {
            state,
            active: true,
        }
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

impl Interpreter {
    pub const fn new(movement_step: f32, rotation_step: Angle) -> Self {
        Self {
            movement_step,
            rotation_step,
            stack: Vec::new(),
        }
    }

    pub fn process_command(&mut self, turtle: &mut Turtle, command: &char) {
        match command {
            'F' => {
                turtle.pen_down();
                turtle.forward(self.movement_step);
            },
            '+' => {
                turtle.left(self.rotation_step);
            },
            '-' => {
                turtle.right(self.rotation_step);
            },
            '[' => {
                self.stack.push(turtle.state);
            }
            ']' => {
                if let Some(previous) = self.stack.pop() {
                    turtle.pen_up();
                    turtle.state = previous;
                }
            }
            _ => { }
        }
    }
}
