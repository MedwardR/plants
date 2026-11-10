use crate::derivation::ViewConfig;
use crate::turtle::{Turtle, TurtleState};

pub struct Interpreter {
    pub view: ViewConfig,
    stack: Vec<TurtleState>,
}

impl Interpreter {
    pub fn new(view: &ViewConfig) -> Self {
        Self {
            view: view.clone(),
            stack: Vec::new(),
        }
    }

    pub fn process_command(&mut self, turtle: &mut Turtle, command: &char) {
        match command {
            'F' | 'G' => {
                turtle.pen_down();
                turtle.step();
            },
            '+' => {
                turtle.left(self.view.delta);
            },
            '-' => {
                turtle.right(self.view.delta);
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
