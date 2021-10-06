use bevy::prelude::*;

#[derive(Debug)]
pub struct Camera {
    pub left_x: i32,
    pub right_x: i32,
    pub top_y: i32,
    pub bottom_y: i32,
    pub display_width: i32,
    pub display_height: i32,
}

impl Camera {
    pub fn new(player_position: IVec2, display_width: i32, display_height: i32) -> Self {
        Self {
            left_x: player_position.x - (display_width / 2),
            right_x: player_position.x + (display_width / 2),
            top_y: player_position.y - (display_height / 2),
            bottom_y: player_position.y + (display_height / 2),
            display_width,
            display_height,
        }
    }

    pub fn on_player_move(&mut self, player_position: IVec2) {
        self.left_x = player_position.x - (self.display_width / 2);
        self.right_x = player_position.x + (self.display_width / 2);
        self.top_y = player_position.y - (self.display_height / 2);
        self.bottom_y = player_position.y + (self.display_height / 2);
    }
}
