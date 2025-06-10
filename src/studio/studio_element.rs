use macroquad::prelude::draw_circle;
use macroquad::prelude::draw_ellipse;
use macroquad::prelude::draw_line;
use macroquad::prelude::draw_rectangle_ex;
use macroquad::prelude::draw_triangle;
use macroquad::prelude::Color;
use macroquad::prelude::DrawRectangleParams;
use macroquad::prelude::Vec2;

use super::StudioElements;

#[derive(Debug, Clone, Copy)]
pub enum StudioValues {
    Line {
        point_a: Vec2,
        point_b: Vec2,
        thickness: f32,
    },
    Circle {
        center: Vec2,
        radius: f32,
    },
    Ellipse {
        center: Vec2,
        width: f32,
        height: f32,
        rotation: f32,
    },
    Rectangle {
        point: Vec2,
        width: f32,
        height: f32,
        rotation: f32,
    },
    Triangle {
        point_a: Vec2,
        point_b: Vec2,
        point_c: Vec2,
    },
}

#[derive(Debug, Clone, Copy)]
pub struct StudioElement {
    pub color: Color,
    pub value: StudioValues,
    pub element: StudioElements,
    // use super::StudioShapes;

    // pub struct StudioElement {
    //     color: Color,
    //     shape: StudioShapes,
    //     element: StudioElements,
    //     // pub id: usize,
    //     // pub a: Vec2,
    //     // pub b: Vec2,
    //     // // pub c: Option<Vec2>,
    //     // // pub rotation: Option<Vec2>,
    //     // pub color: Color,
    //     // pub shape: StudioElements,
    //     // pub props: StudioElementProps,
    //     // pub sides: Option<usize>,
    // }
}

impl StudioElement {
    pub fn new(element: StudioElements, value: StudioValues, color: Color) -> Self {
        Self {
            element,
            color,
            value,
        }
    }
    pub fn draw(&self, color: Option<Color>) {
        let color = color.unwrap_or(self.color);
        match self.value {
            StudioValues::Line {
                point_a,
                point_b,
                thickness,
            } => {
                let a_x = point_a.x;
                let a_y = point_a.y;
                let b_x = point_b.x;
                let b_y = point_b.y;
                draw_line(a_x, a_y, b_x, b_y, thickness, color);
            }
            // StudioValues::Poly => {
            //     let radius = current.distance(position);
            //     let sides = (radius / 10.0).clamp(3.0, 12.0) as u8;
            //     let dx = position.x - current.x;
            //     let dy = position.y - current.y;
            //     let rotation = dy.atan2(dx);
            //     draw_poly(current.x, current.y, sides, radius, rotation, color);
            // }
            StudioValues::Circle { center, radius } => {
                let x = center.x;
                let y = center.y;
                draw_circle(x, y, radius, color);
            }
            // StudioElements::Arc => {
            //     let radius = current.distance(position);
            //     let sides = (radius / 4.0).clamp(12.0, 64.0) as u8;
            //     let dx = position.x - current.x;
            //     let dy = position.y - current.y;
            //     let angle = dy.atan2(dx).to_degrees();
            //     let arc = 180.0;
            //     draw_arc(current.x, current.y, sides, arc, angle, 1.0, radius, color);
            // }
            StudioValues::Ellipse {
                center,
                width,
                height,
                rotation,
            } => {
                let x = center.x;
                let y = center.y;
                draw_ellipse(x, y, width, height, rotation, color);
            }
            StudioValues::Rectangle {
                point,
                width,
                height,
                rotation,
            } => {
                let x = point.x;
                let y = point.y;
                let offset = Vec2::new(0.0, 0.0);
                draw_rectangle_ex(
                    x,
                    y,
                    width,
                    height,
                    DrawRectangleParams {
                        color,
                        rotation,
                        offset,
                    },
                );
            }
            StudioValues::Triangle {
                point_a,
                point_b,
                point_c,
            } => {
                draw_triangle(point_a, point_b, point_c, color);
            } // StudioElements::Hexagon => {
              //     let radius = current.distance(position);
              //     draw_hexagon(
              //         current.x,
              //         current.y,
              //         radius,
              //         0.0,
              //         current.x < position.x,
              //         color,
              //         color,
              //     );
              // }
        }
    }
}
