use std::mem;
use macroquad::math::Vec2;
use crate::angle::Angle;
use crate::derivation::ViewConfig;
use crate::turtle::Turtle;

pub struct Interpreter {
    pub view: ViewConfig,
    pub color_index: usize,
}

impl Interpreter {
    pub fn new(view: &ViewConfig) -> Self {
        Self {
            view: view.clone(),
            color_index: 0,
        }
    }

    pub fn process_command(&mut self, turtle: &mut Turtle, command: &char) -> Vec<Vec2> {
        let start = turtle.state.position;

        match command {
            'F' => {
                turtle.pen_down();
                turtle.step();
                turtle.record_vertex();
            },
            'f' => {
                turtle.pen_up();
                turtle.step();
                turtle.record_vertex();
            },
            'G' => {
                turtle.pen_down();
                turtle.step();
            },
            'g' => {
                turtle.pen_up();
                turtle.step();
            }
            '+' => {
                turtle.left(self.view.delta);
            },
            '-' => {
                turtle.right(self.view.delta);
            },
            '|' => {
                turtle.right(Angle::PI);
            },
            '[' => {
                turtle.start_branch();
            },
            ']' => {
                turtle.end_branch();
            },
            '{' => {
                turtle.start_polygon();
            },
            '}' => {
                turtle.end_polygon();
            },
            '.' => {
                turtle.record_vertex();
            },
            '!' => {
                turtle.state.thinness += 1;
            },
            '\'' => {
                self.color_index = (self.color_index + 1) % self.view.colors.len();
                turtle.state.color = self.view.colors[self.color_index];
            },
            _ => { }
        }

        if turtle.pen_active && turtle.state.position != start {
            vec![start, turtle.state.position]
        }
        else if !turtle.polygon_active && turtle.polygon.len() > 0 {
            mem::take(&mut turtle.polygon)
        }
        else {
            Vec::new()
        }
    }
}
