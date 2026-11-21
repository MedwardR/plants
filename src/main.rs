pub mod angle;
pub mod derivation;
pub mod drawing;
pub mod interpreter;
pub mod turtle;

use crate::derivation::LSystem;
use crate::drawing::{Drawing, Line, Polygon, draw_polygon};
use macroquad::prelude::*;
use std::fs;
use std::ops::{Add, Mul};

#[macroquad::main("Terrarium")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let toml = fs::read_to_string("./examples/kelp.toml")?;
    let mut engine = LSystem::from_toml(toml)?;

    let mut drawing = Drawing::new();
    let mut content_dimensions = Rect::default();
    let mut drawing_invalid = true;

    loop {
        clear_background(BLACK);

        if is_key_pressed(KeyCode::Up) {
            engine.state.n += 1;
            drawing_invalid = true;
        }
        if is_key_pressed(KeyCode::Down) && engine.state.n > 0 {
            engine.state.n -= 1;
            drawing_invalid = true;
        }

        if drawing_invalid {
            drawing = generate_drawing(&engine);
            content_dimensions = calculate_boundaries(&drawing);
            drawing_invalid = false;
        }

        const PADDING: f32 = 50.0;

        let screen_dimensions = Vec2::new(
            screen_width() - PADDING * 2.0,
            screen_height() - PADDING * 2.0,
        );

        let scale = screen_dimensions.x.min(screen_dimensions.y)
            / content_dimensions.w.max(content_dimensions.h);

        let offset = Vec2::new(
            PADDING + (screen_dimensions.x - content_dimensions.w * scale) / 2.0
                - content_dimensions.x * scale,
            PADDING + (screen_dimensions.y - content_dimensions.h * scale) / 2.0
                - content_dimensions.y * scale,
        );

        for line in &drawing.lines {
            let a = line.start.mul(scale).add(offset);
            let b = line.end.mul(scale).add(offset);

            draw_line(a.x, a.y, b.x, b.y, line.thickness, line.color);
        }
        for polygon in &drawing.polygons {
            let transformed: Vec<_> = polygon
                .vertices
                .iter()
                .map(|v| v.mul(scale).add(offset))
                .collect();

            draw_polygon(transformed, polygon.color);
        }

        draw_text(engine.state.n.to_string(), PADDING, PADDING, 24.0, WHITE);
        next_frame().await
    }
}

fn generate_drawing(engine: &LSystem) -> Drawing {
    let instructions = engine.derive();

    let mut lines: Vec<Line> = Vec::new();
    let mut lines_thinness: Vec<usize> = Vec::new();
    let mut polygons: Vec<Polygon> = Vec::new();

    let mut turtle = engine.turtle();
    let mut interpreter = engine.interpreter();

    for command in instructions.chars() {
        let vertices = interpreter.process_command(&mut turtle, &command);

        if vertices.len() == 2 {
            let value = Line::new(
                vertices[0],
                vertices[1],
                turtle.state.color,
                engine.view.thickness,
            );
            lines.push(value);
            lines_thinness.push(turtle.state.thinness);
        } else if vertices.len() > 2 {
            let value = Polygon::new(vertices, turtle.state.color);
            polygons.push(value);
        }
    }

    if let Some(denominator) = lines_thinness.iter().copied().max().map(|v| v + 1) {
        for (line, thinness) in lines.iter_mut().zip(lines_thinness) {
            let ratio = if denominator != 0 {
                1.0 - thinness as f32 / denominator as f32
            } else {
                1.0
            };

            line.thickness = engine.view.thickness * ratio;
        }
    }

    Drawing { lines, polygons }
}

fn calculate_boundaries(drawing: &Drawing) -> Rect {
    let mut min_x = 0.0;
    let mut min_y = 0.0;
    let mut max_x = 0.0;
    let mut max_y = 0.0;

    for v in drawing.vertices() {
        if v.x < min_x {
            min_x = v.x;
        }
        if v.y < min_y {
            min_y = v.y;
        }
        if v.x > max_x {
            max_x = v.x;
        }
        if v.y > max_y {
            max_y = v.y;
        }
    }

    Rect::new(min_x, min_y, max_x - min_x, max_y - min_y)
}
