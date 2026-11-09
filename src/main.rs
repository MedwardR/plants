pub mod geometry;
pub mod derivation;

use std::ops::{Add, Mul};
use macroquad::prelude::*;
use crate::derivation::LSystem;
use crate::geometry::{Angle, Interpreter, Turtle, TurtleState};

#[macroquad::main("Terrarium")]
async fn main() {

    let mut engine = LSystem::new();
    engine.add_rule('X', "F-[[X]+X]+F[+FX]-X");
    engine.add_rule('F', "FF");

    const AXIOM: &str = "X";
    const MOVEMENT_STEP: f32 = 5.0;
    const ROTATION_STEP: Angle = Angle::from_degrees(22.5);

    let mut iterations = 0;
    let mut drawing_invalid = true;

    let mut lines = Vec::new();
    let mut content_dimensions = Rect::default();

    loop {
        clear_background(BLACK);

        if is_key_pressed(KeyCode::Up) {
            iterations = (iterations + 1).max(0);
            drawing_invalid = true;
        }
        if is_key_pressed(KeyCode::Down) {
            iterations = (iterations - 1).max(0);
            drawing_invalid = true;
        }

        if drawing_invalid {
            lines = generate_drawing(&engine, AXIOM, MOVEMENT_STEP, ROTATION_STEP, iterations);
            content_dimensions = calculate_boundaries(&lines);
            drawing_invalid = false;
        }

        const PADDING: f32 = 50.0;

        let screen_dimensions = Vec2::new(
            screen_width() - PADDING * 2.0,
            screen_height() - PADDING * 2.0,
        );

        let scale =
            screen_dimensions.x.min(screen_dimensions.y) /
            content_dimensions.w.max(content_dimensions.h);

        let offset = Vec2::new(
            PADDING + (screen_dimensions.x - content_dimensions.w * scale) / 2.0 - content_dimensions.x * scale,
            PADDING + (screen_dimensions.y - content_dimensions.h * scale) / 2.0 - content_dimensions.y * scale,
        );

        draw_circle(offset.x, offset.y, 4.0, RED);
        draw_line(offset.x, offset.y, offset.x + 20.0, offset.y - 20.0, 2.0, RED);

        for (start, end) in &lines {
            let a = start.mul(scale).add(offset);
            let b = end.mul(scale).add(offset);

            draw_line(a.x, a.y, b.x, b.y, 4.0, DARKGREEN);
        }

        draw_text(iterations.to_string(), PADDING, PADDING, 24.0, WHITE);
        next_frame().await
    }
}

fn generate_drawing(engine: &LSystem, axiom: impl Into<String>, movement_step: f32, rotation_step: Angle, iterations: i32) -> Vec<(Vec2, Vec2)> {

    let instructions = engine.derive(axiom, iterations);
    let mut lines: Vec<(Vec2, Vec2)> = Vec::new();

    let mut turtle = Turtle::new(TurtleState {
        rotation: Angle::from_degrees(-90.0),
        ..Default::default()
    });

    let mut interpreter = Interpreter::new(movement_step, rotation_step);

    for command in instructions.chars() {
        let start = turtle.state.position;

        interpreter.process_command(&mut turtle, &command);

        if turtle.active && turtle.state.position != start {
            lines.push((start, turtle.state.position));
        }
    }

    lines
}

fn calculate_boundaries(lines: &Vec<(Vec2, Vec2)>) -> Rect {
    let mut min_x = 0.0;
    let mut min_y = 0.0;
    let mut max_x = 0.0;
    let mut max_y = 0.0;

    for (start, end) in lines {
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

    Rect::new(min_x, min_y, max_x - min_x, max_y - min_y)
}
