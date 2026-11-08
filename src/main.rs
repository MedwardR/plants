pub mod geometry;
pub mod derivation;

use std::ops::{Add, Mul};
use macroquad::prelude::*;
use crate::derivation::LSystem;
use crate::geometry::{Angle, Interpreter, Turtle, TurtleState};

#[macroquad::main("Terrarium")]
async fn main() {
    const AXIOM: &str = "F";
    const ITERATIONS: i32 = 4;

    let mut lsys = LSystem::new();
    lsys.add_rule('F', "F-F++F-F");

    let instructions = lsys.derive(AXIOM, ITERATIONS);
    let mut lines: Vec<(Vec2, Vec2)> = Vec::new();

    let mut turtle = Turtle::new(TurtleState {
        rotation: Angle::from_degrees(-90.0),
        ..Default::default()
    });

    let mut interpreter = Interpreter::new(
        50.0,
        Angle::from_degrees(45.0),
    );

    for command in instructions.chars() {
        let start = turtle.state.position;

        interpreter.process_command(&mut turtle, &command);

        if turtle.active && turtle.state.position != start {
            lines.push((start, turtle.state.position));
        }
    }

    let mut min_x = 0.0;
    let mut min_y = 0.0;
    let mut max_x = 0.0;
    let mut max_y = 0.0;

    for (start, end) in &lines {
        if start.x < min_x {
            min_x = start.x;
        }
        if start.y < min_y {
            min_y = start.y;
        }
        if end.x < min_x {
            min_x = end.x;
        }
        if end.y < min_y {
            min_y = end.y;
        }
        if start.x > max_x {
            max_x = start.x;
        }
        if start.y > max_y {
            max_y = start.y;
        }
        if end.x > max_x {
            max_x = end.x;
        }
        if end.y > max_y {
            max_y = end.y;
        }
    }

    let content_dimensions = Vec2::new(max_x - min_x, max_y - min_y);

    loop {
        clear_background(BLACK);

        const PADDING: f32 = 50.0;

        let screen_dimensions = Vec2::new(
            screen_width() - PADDING * 2.0,
            screen_height() - PADDING * 2.0,
        );

        let scale =
            screen_dimensions.x.min(screen_dimensions.y) /
            content_dimensions.x.max(content_dimensions.y);

        let offset = Vec2::new(
            PADDING + (screen_dimensions.x - content_dimensions.x * scale) / 2.0 - min_x * scale,
            PADDING + (screen_dimensions.y - content_dimensions.y * scale) / 2.0 - min_y * scale,
        );

        draw_circle(offset.x, offset.y, 4.0, RED);
        draw_line(offset.x, offset.y, offset.x + 20.0, offset.y - 20.0, 2.0, RED);

        for (start, end) in &lines {
            let a = start.mul(scale).add(offset);
            let b = end.mul(scale).add(offset);

            draw_line(a.x, a.y, b.x, b.y, 4.0, DARKGREEN);
        }

        next_frame().await
    }
}
