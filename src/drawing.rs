use macroquad::prelude::*;

pub struct Drawing {
    pub lines: Vec<Line>,
    pub polygons: Vec<Polygon>,
}

#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub start: Vec2,
    pub end: Vec2,
    pub color: Color,
    pub thickness: f32,
}

#[derive(Debug, Clone)]
pub struct Polygon {
    pub vertices: Vec<Vec2>,
    pub color: Color,
}

impl Drawing {
    pub const fn new() -> Self {
        Self {
            lines: Vec::new(),
            polygons: Vec::new(),
        }
    }

    pub fn vertices(&self) -> Vec<Vec2> {
        self.lines
            .iter()
            .flat_map(|line| [line.start, line.end])
            .chain(
                self.polygons
                    .iter()
                    .flat_map(|polygon| polygon.vertices.iter().copied()),
            )
            .collect()
    }
}

impl Line {
    pub fn new(start: Vec2, end: Vec2, color: Color, thickness: f32) -> Self {
        Self {
            start,
            end,
            color,
            thickness,
        }
    }
}

impl Polygon {
    pub fn new(vertices: Vec<Vec2>, color: Color) -> Self {
        Self { vertices, color }
    }
}

pub fn draw_polygon(points: impl AsRef<[Vec2]>, color: Color) -> bool {
    if let Some(triangles) = triangulate_polygon(points.as_ref()) {
        for triangle in triangles {
            draw_triangle(triangle[0], triangle[1], triangle[2], color);
        }
        true
    } else {
        eprintln!("Could not triangulate polygon");
        false
    }
}

fn cross(a: Vec2, b: Vec2, c: Vec2) -> f32 {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}

fn point_in_triangle(p: Vec2, a: Vec2, b: Vec2, c: Vec2, orientation: f32) -> bool {
    let eps = 0.00001;

    cross(a, b, p) * orientation >= -eps
        && cross(b, c, p) * orientation >= -eps
        && cross(c, a, p) * orientation >= -eps
}

fn triangulate_polygon(points: &[Vec2]) -> Option<Vec<[Vec2; 3]>> {
    let n = points.len();

    if n < 3 {
        return None;
    }

    let mut area = 0.0;

    for i in 0..n {
        let a = points[i];
        let b = points[(i + 1) % n];

        area += a.x * b.y - b.x * a.y;
    }

    if area.abs() < 0.00001 {
        return None;
    }

    let orientation = if area > 0.0 { 1.0 } else { -1.0 };
    let mut remaining: Vec<usize> = (0..n).collect();
    let mut triangles = Vec::with_capacity(n - 2);

    while remaining.len() > 3 {
        let len = remaining.len();
        let mut ear_found = false;

        for i in 0..len {
            let prev = remaining[(i + len - 1) % len];
            let curr = remaining[i];
            let next = remaining[(i + 1) % len];

            let a = points[prev];
            let b = points[curr];
            let c = points[next];

            if cross(a, b, c) * orientation <= 0.00001 {
                continue;
            }

            let mut contains_point = false;

            for &idx in &remaining {
                if idx == prev || idx == curr || idx == next {
                    continue;
                }

                if point_in_triangle(points[idx], a, b, c, orientation) {
                    contains_point = true;
                    break;
                }
            }

            if contains_point {
                continue;
            }

            triangles.push([a, b, c]);
            remaining.remove(i);

            ear_found = true;
            break;
        }

        if !ear_found {
            return None;
        }
    }

    triangles.push([
        points[remaining[0]],
        points[remaining[1]],
        points[remaining[2]],
    ]);

    Some(triangles)
}
