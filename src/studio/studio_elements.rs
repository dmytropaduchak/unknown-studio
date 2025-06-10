use macroquad::prelude::draw_circle_lines;
use macroquad::prelude::draw_line;
use macroquad::prelude::is_mouse_button_pressed;
use macroquad::prelude::is_mouse_button_released;
use macroquad::prelude::screen_height;
use macroquad::prelude::screen_width;
use macroquad::prelude::MouseButton;
use macroquad::prelude::Vec2;
use macroquad::prelude::DARKGRAY;
use macroquad::prelude::YELLOW;

// use crate::studio::StudioShapes;
const SIZE_RESTRICTION: f32 = 10.0;

use super::StudioButtons;
use super::StudioElement;
use super::StudioState;
use super::StudioValues;
use super::SIZE_POINT;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StudioElements {
    // Arc,
    Line,
    // Poly,
    // PolyLine,
    Circle,
    // CircleLine,
    Ellipse,
    // EllipseLine,
    Rectangle,
    // RectangleLine,
    Triangle,
    // Hexagon,
}

impl StudioElements {
    pub fn text(&self) -> &str {
        match self {
            StudioElements::Line => "LINE",
            // StudioElements::Arc => "ARC",
            // StudioElements::Poly => "POLY",
            StudioElements::Circle => "CIRCLE",
            StudioElements::Ellipse => "ELLIPSE",
            StudioElements::Rectangle => "RECTANGLE",
            StudioElements::Triangle => "TRIANGLE",
            // StudioElements::Hexagon => "HEXAGON",
        }
    }
    fn element(state: &mut StudioState, current: Vec2, position: Vec2) -> StudioElement {
        let element = state.element;
        let element_color = state.element_color;
        let element_value = match element {
            StudioElements::Circle => {
                let radius = current.distance(position);
                let center = current;
                StudioValues::Circle { center, radius }
            }
            StudioElements::Ellipse => {
                let width = (position.x - current.x).abs();
                let height = (position.y - current.y).abs();
                let rotation = 0.0;
                let center = current;
                StudioValues::Ellipse {
                    center,
                    height,
                    width,
                    rotation,
                }
            }
            StudioElements::Line => {
                let point_a = current;
                let point_b = position;
                let thickness = state.element_thickness;
                StudioValues::Line {
                    point_a,
                    point_b,
                    thickness,
                }
            }
            StudioElements::Rectangle => {
                let x = current.x.min(position.x);
                let y = current.y.min(position.y);
                let point = Vec2::new(x, y);
                let width = (position.x - current.x).abs();
                let height = (position.y - current.y).abs();
                let rotation = 0.0;
                StudioValues::Rectangle {
                    point,
                    width,
                    height,
                    rotation,
                }
            }
            StudioElements::Triangle => {
                let point_a = current;
                let point_b = position;
                let point_c = Vec2::new(current.x, current.y * 0.5);
                StudioValues::Triangle {
                    point_a,
                    point_b,
                    point_c,
                }
            }
        };
        StudioElement::new(element, element_value, element_color)
    }

    pub fn draw(state: &mut StudioState) {
        let width = screen_width();
        let height = screen_height();
        let position = state.position();

        if state.draw && !state.drag {
            if let Some(current) = state.current {
                let element = StudioElements::element(state, current, position);
                let element_color = DARKGRAY;
                element.draw(Some(element_color));
            }
        }
        if !state.draw && state.drag {
            if let Some(element) = state.stack.iter_mut().find(|i| match i.value {
                StudioValues::Circle { center, radius } => position.distance(center) <= radius,
                _ => false,
            }) {
                if let StudioValues::Circle { ref mut center, .. } = element.value {
                    *center = position - state.drag_offset.unwrap_or(position);
                }
            }
        }

        for element in state.stack.iter() {
            element.draw(None);
        }

        let color = YELLOW.with_alpha(0.2);
        for element in state.stack.iter() {
            match element.value {
                StudioValues::Line {
                    point_a,
                    point_b,
                    thickness,
                } => {
                    if position.distance(point_a) <= SIZE_POINT
                        || position.distance(point_b) <= SIZE_POINT
                    {
                        draw_circle_lines(point_a.x, point_a.y, SIZE_POINT, 1.0, color);
                        draw_circle_lines(point_b.x, point_b.y, SIZE_POINT, 1.0, color);
                    }
                    if state.snap {
                        if (position.x - point_a.x).abs() < SIZE_POINT {
                            draw_line(point_a.x, 0.0, point_a.x, height, 1.0, color);
                        }
                        if (position.y - point_a.y).abs() < SIZE_POINT {
                            draw_line(0.0, point_a.y, width, point_a.y, 1.0, color);
                        }
                        if (position.x - point_b.x).abs() < SIZE_POINT {
                            draw_line(point_b.x, 0.0, point_b.x, height, 1.0, color);
                        }
                        if (position.y - point_b.y).abs() < SIZE_POINT {
                            draw_line(0.0, point_b.y, width, point_b.y, 1.0, color);
                        }
                    }
                }
                StudioValues::Circle { center, radius } => {
                    let point1 = Vec2::new(center.x, center.y + radius);
                    let point2 = Vec2::new(center.x, center.y - radius);
                    let point3 = Vec2::new(center.x + radius, center.y);
                    let point4 = Vec2::new(center.x - radius, center.y);
                    if position.distance(center) <= SIZE_POINT
                        || position.distance(point1) <= SIZE_POINT
                        || position.distance(point2) <= SIZE_POINT
                        || position.distance(point3) <= SIZE_POINT
                        || position.distance(point4) <= SIZE_POINT
                    {
                        draw_circle_lines(center.x, center.y, SIZE_POINT, 1.0, color);
                    }

                    if state.snap {
                        if (position.x - center.x).abs() < SIZE_POINT {
                            draw_line(center.x, 0.0, center.x, height, 1.0, color);
                        }
                        if (position.y - center.y).abs() < SIZE_POINT {
                            draw_line(0.0, center.y, width, center.y, 1.0, color);
                        }
                        if (position.y - point1.y).abs() < SIZE_POINT {
                            draw_line(0.0, point1.y, width, point1.y, 1.0, color);
                        }
                        if (position.y - point2.y).abs() < SIZE_POINT {
                            draw_line(0.0, point2.y, width, point2.y, 1.0, color);
                        }
                        if (position.x - point3.x).abs() < SIZE_POINT {
                            draw_line(point3.x, 0.0, point3.x, height, 1.0, color);
                        }
                        if (position.x - point4.x).abs() < SIZE_POINT {
                            draw_line(point4.x, 0.0, point4.x, height, 1.0, color);
                        }
                    }
                }
                _ => {}
            }
        }
    }
    pub fn actions(state: &mut StudioState) {
        let position = state.position();

        if is_mouse_button_pressed(MouseButton::Left) && state.draw {
            state.current = Some(position);
        }

        if is_mouse_button_released(MouseButton::Left) && state.draw {
            if let Some(current) = state.current.take() {
                if current.distance(position) > SIZE_RESTRICTION {
                    let element = StudioElements::element(state, current, position);
                    state.save();
                    state.stack.push(element);
                }
            }
        }

        if is_mouse_button_pressed(MouseButton::Left) && !state.draw {
            if let Some(element) = state.stack.iter().find(|i| match i.value {
                StudioValues::Circle { center, radius } => position.distance(center) <= radius,
                _ => false,
            }) {
                if let StudioValues::Circle { center, .. } = element.value {
                    state.drag_offset = Some(position - center);
                    state.drag = true;
                }
            }
        }

        if is_mouse_button_released(MouseButton::Left) && !state.draw {
            state.drag = false;
        }
    }
}

impl From<StudioElements> for StudioButtons {
    fn from(i: StudioElements) -> Self {
        match i {
            // StudioElements::Arc => StudioButtons::Arc,
            StudioElements::Line => StudioButtons::Line,
            // StudioElements::Poly => StudioButtons::Poly,
            StudioElements::Circle => StudioButtons::Circle,
            // StudioShapes::CircleLine => StudioButtons::Circle,
            StudioElements::Ellipse => StudioButtons::Ellipse,
            StudioElements::Rectangle => StudioButtons::Rectangle,
            StudioElements::Triangle => StudioButtons::Triangle,
            // StudioElements::Hexagon => StudioButtons::Hexagon,
        }
    }
}
