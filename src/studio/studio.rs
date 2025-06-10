use macroquad::prelude::clear_background;
use macroquad::prelude::next_frame;
use macroquad::prelude::Color;
use macroquad::prelude::BLACK;

use super::StudioButtons;
use super::StudioElements;
use super::StudioHelps;
use super::StudioState;

pub struct Studio {
    color: Color,
    state: StudioState,
}

impl Studio {
    pub fn new() -> Self {
        let color = BLACK;
        let state = StudioState::new();
        Studio { color, state }
    }

    pub async fn run(&mut self) {
        loop {
            clear_background(self.color);

            StudioButtons::actions(&mut self.state);
            StudioButtons::draw(&mut self.state);

            StudioElements::actions(&mut self.state);
            StudioElements::draw(&mut self.state);

            StudioHelps::actions(&mut self.state);
            StudioHelps::draw(&mut self.state);

            next_frame().await;
        }
    }
}
