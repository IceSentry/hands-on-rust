use bevy::prelude::*;

#[derive(Debug)]
pub struct Camera {
    pub left_x: u32,
    pub right_x: u32,
    pub top_y: u32,
    pub bottom_y: u32,
    pub display_width: u32,
    pub display_height: u32,
}

impl Camera {
    pub fn new(player_position: UVec2, display_width: u32, display_height: u32) -> Self {
        Self {
            left_x: player_position.x.saturating_sub(display_width / 2),
            right_x: player_position.x.saturating_add(display_width / 2),
            top_y: player_position.y.saturating_sub(display_height / 2),
            bottom_y: player_position.y.saturating_add(display_height / 2),
            display_height,
            display_width,
        }
    }
    pub fn on_player_move(&mut self, player_position: UVec2) {
        puffin::profile_function!();

        self.left_x = player_position.x.saturating_sub(self.display_width / 2);
        self.right_x = player_position.x.saturating_add(self.display_width / 2);
        self.top_y = player_position.y.saturating_sub(self.display_height / 2);
        self.bottom_y = player_position.y.saturating_add(self.display_height / 2);
    }
}
