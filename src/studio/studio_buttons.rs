use macroquad::prelude::draw_text;
use macroquad::prelude::is_key_down;
use macroquad::prelude::is_key_pressed;
use macroquad::prelude::is_key_released;
use macroquad::prelude::is_mouse_button_pressed;
use macroquad::prelude::measure_text;
use macroquad::prelude::mouse_position;
use macroquad::prelude::KeyCode;
use macroquad::prelude::MouseButton;
use macroquad::prelude::TextDimensions;
use macroquad::prelude::Vec2;
use macroquad::prelude::DARKGRAY;
use macroquad::prelude::GRAY;
use macroquad::prelude::GREEN;
use macroquad::prelude::LIGHTGRAY;

use super::StudioButton;
use super::StudioElements;
use super::StudioState;

pub const BUTTON_SIZE: f32 = 21.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StudioButtons {
    Undo,
    Redo,
    Help,
    Grid,
    Snap,
    Color,
    // Thickness,
    // Zoom,
    // ZoomIn,
    // ZoonOut,
    Line,
    Arc,
    Poly,
    // PolyLine,
    Circle,
    // CircleLine,
    Ellipse,
    // EllipseLine,
    Rectangle,
    // RectangleLine,
    Triangle,
    Hexagon,
}

impl StudioButtons {
    pub fn text(&self) -> &str {
        match self {
            StudioButtons::Undo => "UNDO",
            StudioButtons::Redo => "REDO",
            StudioButtons::Help => "HELP",
            StudioButtons::Grid => "GRID",
            StudioButtons::Snap => "SNAP",
            StudioButtons::Color => "COLOR",
            // StudioButtons::ZoomIn => "ZOOM_IN",
            // StudioButtons::ZoomOut => "ZOOM_OUT",
            StudioButtons::Line => "LINE",
            StudioButtons::Arc => "ARC",
            StudioButtons::Poly => "POLY",
            StudioButtons::Circle => "CIRCLE",
            StudioButtons::Ellipse => "ELLIPSE",
            StudioButtons::Rectangle => "RECTANGLE",
            StudioButtons::Triangle => "TRIANGLE",
            StudioButtons::Hexagon => "HEXAGON",
        }
    }
    pub fn dimensions(&self) -> TextDimensions {
        let text = self.text();
        measure_text(text, None, BUTTON_SIZE as u16, 1.0)
    }
    pub fn draw(state: &mut StudioState) {
        let position: Vec2 = mouse_position().into();
        let buttons: Vec<StudioButton> = StudioButton::list();

        for i in buttons {
            let text = i.button.text();
            let text_dimensions = i.button.dimensions();
            let is_position = position.x >= i.x
                && position.x <= i.x + text_dimensions.width
                && position.y >= i.y - text_dimensions.height
                && position.y <= i.y;
            let color = match i.button {
                StudioButtons::Undo => {
                    if state.stack_undo.is_empty() {
                        DARKGRAY
                    } else if is_position {
                        LIGHTGRAY
                    } else {
                        GRAY
                    }
                }
                StudioButtons::Redo => {
                    if state.stack_redo.is_empty() {
                        DARKGRAY
                    } else if is_position {
                        LIGHTGRAY
                    } else {
                        GRAY
                    }
                }
                StudioButtons::Help => {
                    if is_position || state.help {
                        GREEN
                    } else {
                        GRAY
                    }
                }
                StudioButtons::Snap => {
                    if is_position || state.snap {
                        GREEN
                    } else {
                        GRAY
                    }
                }
                StudioButtons::Grid => {
                    if is_position || state.grid >= 1 {
                        GREEN
                    } else {
                        GRAY
                    }
                }
                StudioButtons::Circle
                | StudioButtons::Ellipse
                | StudioButtons::Line
                | StudioButtons::Arc
                | StudioButtons::Rectangle
                | StudioButtons::Triangle
                | StudioButtons::Hexagon
                | StudioButtons::Poly => {
                    if is_position || i.button == StudioButtons::from(state.element) && state.draw {
                        GREEN
                    } else {
                        GRAY
                    }
                }
                _ => {
                    if is_position {
                        LIGHTGRAY
                    } else {
                        GRAY
                    }
                }
            };
            draw_text(text, i.x, i.y, i.size, color);
        }
    }
    pub fn actions(state: &mut StudioState) {
        if is_key_pressed(KeyCode::Z) && is_key_down(KeyCode::LeftSuper) {
            state.undo();
        }
        if is_key_pressed(KeyCode::Y) && is_key_down(KeyCode::LeftSuper) {
            state.redo();
        }

        if is_key_pressed(KeyCode::S) && is_key_down(KeyCode::LeftSuper) {
            state.snap = !state.snap;
        }
        if is_key_pressed(KeyCode::G) && is_key_down(KeyCode::LeftSuper) {
            if state.grid > 2 {
                state.grid = 0;
            } else {
                state.grid += 1;
            }
        }

        if is_key_pressed(KeyCode::Key1) && is_key_down(KeyCode::LeftSuper) {
            state.element = StudioElements::Line;
        }

        if is_key_down(KeyCode::H) {
            state.help = true;
        }
        if is_key_released(KeyCode::H) {
            state.help = false;
        }
        if is_key_pressed(KeyCode::E) {
            state.export();
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            if let Some(button) = StudioButton::find().take() {
                if [
                    StudioButtons::Ellipse,
                    StudioButtons::Line,
                    StudioButtons::Triangle,
                    StudioButtons::Rectangle,
                    StudioButtons::Circle,
                ]
                .contains(&button.button)
                {
                    if state.button == Some(button.button) {
                        state.draw = !state.draw;
                    } else {
                        state.draw = true;
                    }
                }
                match button.button {
                    StudioButtons::Undo => {
                        state.button = Some(StudioButtons::Undo);
                        state.undo();
                    }
                    StudioButtons::Redo => {
                        state.button = Some(StudioButtons::Redo);
                        state.redo();
                    }
                    StudioButtons::Help => {
                        state.button = Some(StudioButtons::Help);
                        state.help = !state.help;
                    }
                    StudioButtons::Arc => {
                        // state.button = Some(StudioButtons::Arc);
                        // state.element = StudioElements::Arc;
                    }
                    StudioButtons::Poly => {
                        // state.button = Some(StudioButtons::Poly);
                        // state.element = StudioElements::Poly;
                    }
                    StudioButtons::Ellipse => {
                        state.button = Some(StudioButtons::Ellipse);
                        state.element = StudioElements::Ellipse;
                    }
                    StudioButtons::Rectangle => {
                        state.button = Some(StudioButtons::Rectangle);
                        state.element = StudioElements::Rectangle;
                    }
                    StudioButtons::Triangle => {
                        state.button = Some(StudioButtons::Triangle);
                        state.element = StudioElements::Triangle;
                    }
                    StudioButtons::Hexagon => {
                        // state.button = Some(StudioButtons::Hexagon);
                        // state.element = StudioElements::Hexagon;
                    }
                    StudioButtons::Line => {
                        state.button = Some(StudioButtons::Line);
                        state.element = StudioElements::Line;
                    }
                    StudioButtons::Circle => {
                        state.button = Some(StudioButtons::Circle);
                        state.element = StudioElements::Circle;
                        // state.element_lines = !state.element_lines;
                    }
                    // Some(StudioButtons::CircleLine) => {
                    //     self.state.button = Some(StudioButtons::CircleLine);
                    //     self.state.element_shape = StudioElements::CircleLine;
                    // }
                    StudioButtons::Grid => {
                        state.button = Some(StudioButtons::Grid);
                        if state.grid > 2 {
                            state.grid = 0;
                        } else {
                            state.grid += 1;
                        }
                    }
                    StudioButtons::Snap => {
                        state.button = Some(StudioButtons::Snap);
                        state.snap = !state.snap;
                    }
                    StudioButtons::Color => {
                        // state.button = Some(StudioButtons::Grid);
                        // if state.grid > 2 {
                        //     state.grid = 0;
                        // } else {
                        //     state.grid += 1;
                        // }
                    }
                }
            }
        }
    }
}
