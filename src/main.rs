use macroquad::prelude::*;

#[derive(Clone)]
pub struct Line {
    pub start: Vec2,
    pub end: Vec2,
}

pub struct EditorState {
    pub lines: Vec<Line>,
    pub current_start: Option<Vec2>,
    pub sticky_radius: f32,

    pub dragging_point: Option<(usize, bool)>, // (line index, is_start)
    pub drag_start_mouse_pos: Option<Vec2>,    // Mouse pos when drag started
    pub drag_original_positions: Vec<(usize, bool, Vec2)>, // All matching points original positions

    pub undo_stack: Vec<Vec<Line>>,
    pub redo_stack: Vec<Vec<Line>>,

    pub show_points: bool,
}

impl EditorState {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            current_start: None,
            sticky_radius: 10.0,

            dragging_point: None,
            drag_start_mouse_pos: None,
            drag_original_positions: Vec::new(),

            undo_stack: Vec::new(),
            redo_stack: Vec::new(),

            show_points: true,
        }
    }
    pub fn save_history(&mut self) {
        self.undo_stack.push(self.lines.clone());
        self.redo_stack.clear(); // Clear redo stack on new action
    }

    pub fn undo(&mut self) {
        if let Some(previous) = self.undo_stack.pop() {
            self.redo_stack.push(self.lines.clone());
            self.lines = previous;
        }
    }

    pub fn redo(&mut self) {
        if let Some(next) = self.redo_stack.pop() {
            self.undo_stack.push(self.lines.clone());
            self.lines = next;
        }
    }
}

pub fn handle_undo_redo_input(state: &mut EditorState) {
    if is_key_pressed(KeyCode::U) {
        state.undo();
    }
    if is_key_pressed(KeyCode::R) {
        state.redo();
    }
}

fn get_all_matching_points(
    lines: &[Line],
    target_point: Vec2,
    epsilon: f32,
) -> Vec<(usize, bool, Vec2)> {
    let mut result = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        if line.start.distance(target_point) < epsilon {
            result.push((i, true, line.start));
        }
        if line.end.distance(target_point) < epsilon {
            result.push((i, false, line.end));
        }
    }
    result
}

pub fn handle_toggle_points(state: &mut EditorState) {
    if is_key_pressed(KeyCode::P) {
        state.show_points = !state.show_points;
    }
}

/// Handle line creation with sticky snapping and minimum length check
pub fn handle_sticky_line_drawing(state: &mut EditorState, mouse_pos: Vec2) {
    // Skip if dragging a point
    if state.dragging_point.is_some() {
        return;
    }
    // Start line on mouse press, snap start point
    if is_mouse_button_pressed(MouseButton::Left) {
        let snap_start = find_closest_point(mouse_pos, &state.lines, state.sticky_radius);
        state.current_start = Some(snap_start.unwrap_or(mouse_pos));
    }

    // On release, snap end point and add line only if line long enough
    if is_mouse_button_released(MouseButton::Left) {
        if let Some(start) = state.current_start.take() {
            let snap_end = find_closest_point(mouse_pos, &state.lines, state.sticky_radius);
            let end = snap_end.unwrap_or(mouse_pos);
            if start.distance(end) > 2.0 * state.sticky_radius {
                state.save_history();
                state.lines.push(Line { start, end });
            }
        }
    }
}

pub fn handle_point_dragging(state: &mut EditorState, mouse_pos: Vec2) {
    if is_mouse_button_pressed(MouseButton::Left) && state.dragging_point.is_none() {
        if let Some((line_idx, is_start)) =
            find_closest_point_for_drag(mouse_pos, &state.lines, 5.0)
        {
            state.save_history();
            let original_point = if is_start {
                state.lines[line_idx].start
            } else {
                state.lines[line_idx].end
            };
            state.dragging_point = Some((line_idx, is_start));
            state.drag_start_mouse_pos = Some(mouse_pos);
            state.drag_original_positions =
                get_all_matching_points(&state.lines, original_point, 0.01);
        }
    }

    if let (Some((_, _)), Some(drag_start_mouse_pos)) =
        (state.dragging_point, state.drag_start_mouse_pos)
    {
        if is_mouse_button_down(MouseButton::Left) {
            let delta = mouse_pos - drag_start_mouse_pos;

            // Compute tentative new positions with delta, then snap if close to other points
            for &(line_idx, is_start, orig_pos) in &state.drag_original_positions {
                let mut new_pos = orig_pos + delta;

                // Snap to closest point (excluding all original points)
                if let Some(snap) = find_closest_point_excluding_multiple(
                    &state.lines,
                    new_pos,
                    state.sticky_radius,
                    &state.drag_original_positions,
                ) {
                    // new_pos = snap;
                    new_pos = snap_to_screen_edge(snap, 5.0);
                }

                if let Some(line) = state.lines.get_mut(line_idx) {
                    if is_start {
                        line.start = new_pos;
                    } else {
                        line.end = new_pos;
                    }
                }
            }
        }
    }

    if is_mouse_button_released(MouseButton::Left) {
        state.dragging_point = None;
        state.drag_start_mouse_pos = None;
        state.drag_original_positions.clear();
    }
}

fn snap_to_screen_edge(pos: Vec2, snap_threshold: f32) -> Vec2 {
    let screen_w = screen_width();
    let screen_h = screen_height();

    let mut snapped = pos;

    if (pos.x - 0.0).abs() < snap_threshold {
        snapped.x = 0.0;
    } else if (pos.x - screen_w).abs() < snap_threshold {
        snapped.x = screen_w;
    }

    if (pos.y - 0.0).abs() < snap_threshold {
        snapped.y = 0.0;
    } else if (pos.y - screen_h).abs() < snap_threshold {
        snapped.y = screen_h;
    }

    snapped
}

fn find_closest_point_excluding_multiple(
    lines: &[Line],
    target: Vec2,
    radius: f32,
    exclude_points: &[(usize, bool, Vec2)],
) -> Option<Vec2> {
    let mut closest = None;
    let mut closest_dist = radius;

    'outer: for line in lines {
        for &point in &[line.start, line.end] {
            // Skip all original dragged points
            for &(_, _, excl_pos) in exclude_points {
                if point.distance(excl_pos) < 0.01 {
                    continue 'outer;
                }
            }

            let dist = target.distance(point);
            if dist <= closest_dist {
                closest = Some(point);
                closest_dist = dist;
            }
        }
    }

    closest
}

fn export_lines(lines: &[Line]) {
    println!("\n// Exported Line Segments");
    println!("const POLYLINES: &[([f32; 2], [f32; 2])] = &[");
    let mut unique: Vec<Line> = Vec::new();
    for line in lines {
        let is_duplicate = unique.iter().any(|l| {
            (l.start == line.start && l.end == line.end)
                || (l.start == line.end && l.end == line.start)
        });
        if !is_duplicate {
            unique.push(line.clone());
        }
    }

    for line in unique {
        println!(
            "    ([{:.1}, {:.1}], [{:.1}, {:.1}]),",
            line.start.x, line.start.y, line.end.x, line.end.y
        );
    }

    println!("];");
}

pub fn draw_help_text() {
    draw_text(
        "C: Clear | E: Export | U: Undo | R: Redo | P: Points",
        10.0,
        20.0,
        20.0,
        GRAY,
    );
}

/// Find closest point to start dragging
fn find_closest_point_for_drag(target: Vec2, lines: &[Line], radius: f32) -> Option<(usize, bool)> {
    let mut closest = None;
    let mut closest_dist = radius;

    for (i, line) in lines.iter().enumerate() {
        let dist_start = target.distance(line.start);
        if dist_start <= closest_dist {
            closest = Some((i, true));
            closest_dist = dist_start;
        }

        let dist_end = target.distance(line.end);
        if dist_end <= closest_dist {
            closest = Some((i, false));
            closest_dist = dist_end;
        }
    }

    closest
}

/// Find closest point (for snapping when creating lines)
fn find_closest_point(target: Vec2, lines: &[Line], radius: f32) -> Option<Vec2> {
    let mut closest = None;
    let mut closest_dist = radius;

    for line in lines {
        for &point in &[line.start, line.end] {
            let dist = target.distance(point);
            if dist < closest_dist {
                closest = Some(point);
                closest_dist = dist;
            }
        }
    }

    closest
}

mod config;
use config::*;

mod studio;
use studio::*;

#[macroquad::main(default)]
async fn main() {
    let mut studio = Studio::new();
    studio.run().await;
}

// #[macroquad::main("Editor with Point Moving")]
// async fn main() {
//     let mut state = EditorState::new();

//     loop {
//         clear_background(WHITE);

//         draw_help_text();

//         handle_undo_redo_input(&mut state);
//         handle_toggle_points(&mut state);

//         if is_key_pressed(KeyCode::C) {
//             state.lines.clear();
//         }

//         // Export
//         if is_key_pressed(KeyCode::E) {
//             export_lines(&state.lines);
//         }

//         let mouse_pos: Vec2 = mouse_position().into();

//         // Dragging points first (highest priority)
//         handle_point_dragging(&mut state, mouse_pos);

//         // Then line drawing if not dragging
//         if state.dragging_point.is_none() {
//             handle_sticky_line_drawing(&mut state, mouse_pos);
//         }

//         // Draw lines and points as before
//         for (i, line) in state.lines.iter().enumerate() {
//             // Draw start point
//             let start_color = if let Some((drag_idx, is_start_flag)) = state.dragging_point {
//                 if drag_idx == i && is_start_flag {
//                     GREEN
//                 } else {
//                     DARKBLUE
//                 }
//             } else {
//                 DARKBLUE
//             };
//             if state.show_points {
//                 draw_circle(line.start.x, line.start.y, 5.0, start_color);
//             }
//             // Draw end point
//             let end_color = if let Some((drag_idx, is_start_flag)) = state.dragging_point {
//                 if drag_idx == i && !is_start_flag {
//                     GREEN
//                 } else {
//                     DARKBLUE
//                 }
//             } else {
//                 DARKBLUE
//             };

//             if state.show_points {
//                 draw_circle(line.end.x, line.end.y, 5.0, end_color);
//             }
//             // Draw line
//             draw_line(
//                 line.start.x,
//                 line.start.y,
//                 line.end.x,
//                 line.end.y,
//                 2.0,
//                 BLACK,
//             );
//         }

//         // Preview line during creation
//         if let Some(start) = state.current_start {
//             draw_line(start.x, start.y, mouse_pos.x, mouse_pos.y, 1.0, LIGHTGRAY);

//             if let Some(closest) = find_closest_point(mouse_pos, &state.lines, state.sticky_radius)
//             {
//                 if state.show_points {
//                     draw_circle(closest.x, closest.y, 5.0, ORANGE);
//                 }
//             }
//         }

//         next_frame().await;
//     }
// }
