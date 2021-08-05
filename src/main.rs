#![warn(clippy::pedantic)]
#![allow(
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::needless_pass_by_value,
    clippy::default_trait_access,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::module_name_repetitions
)]

use ascii_tilemap_plugin::AsciiTilemapPlugin;
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_egui::EguiPlugin;

use crate::ascii_tilemap_plugin::{LayerBuilderData, TilemapBuilder};

mod ascii_tilemap_plugin;
mod flappy_plugin;
mod rusty_dungeon_plugin;

pub const WIDTH: u32 = 80;
pub const HEIGHT: u32 = 50;

pub const DISPLAY_WIDTH: u32 = WIDTH / 2;
pub const DISPLAY_HEIGHT: u32 = HEIGHT / 2;

pub const TILE_WIDTH: u32 = 32;
pub const TILE_HEIGHT: u32 = 32;

pub const WINDOW_WIDTH: f32 = DISPLAY_WIDTH as f32 * TILE_WIDTH as f32;
pub const WINDOW_HEIGHT: f32 = DISPLAY_HEIGHT as f32 * TILE_HEIGHT as f32;

pub enum LayerId {
    Map = 0,
    Entities = 1,
    Hud = 2,
    Diagnostic = 3,
}

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            // TODO find a way to control this by the plugin
            // if they don't match the map will not be aligned properly
            // or update the tilemap size on resize
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            title: String::from("hands on dungeon crawler"),
            vsync: false,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::PINK))
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(profiler::ProfilerPlugin)
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_plugin(AsciiTilemapPlugin)
        .insert_resource(TilemapBuilder {
            layers: vec![
                LayerBuilderData {
                    id: LayerId::Map as u16,
                    texture_path: Some("dungeonfont.png".to_string()),
                    is_background_transparent: false,
                    is_transparent: false,
                    size: Some(UVec2::new(DISPLAY_WIDTH, DISPLAY_HEIGHT)),
                    tile_size: Some(Vec2::new(TILE_WIDTH as f32, TILE_HEIGHT as f32)),
                    tilesheet_size: Some(Vec2::new(16., 16.)),
                },
                LayerBuilderData {
                    id: LayerId::Entities as u16,
                    texture_path: Some("dungeonfont.png".to_string()),
                    is_background_transparent: true,
                    is_transparent: true,
                    size: Some(UVec2::new(DISPLAY_WIDTH, DISPLAY_HEIGHT)),
                    tile_size: Some(Vec2::new(TILE_WIDTH as f32, TILE_HEIGHT as f32)),
                    tilesheet_size: Some(Vec2::new(16., 16.)),
                },
                LayerBuilderData {
                    id: LayerId::Hud as u16,
                    texture_path: Some("16x16-sb-ascii.png".to_string()),
                    is_background_transparent: false,
                    is_transparent: true,
                    size: Some(UVec2::new(0, 0)),
                    tile_size: Some(Vec2::new(16., 16.)),
                    tilesheet_size: Some(Vec2::new(16., 16.)),
                },
                LayerBuilderData {
                    id: LayerId::Diagnostic as u16,
                    texture_path: Some("16x16-sb-ascii.png".to_string()),
                    is_background_transparent: false,
                    is_transparent: true,
                    size: Some(UVec2::new(DISPLAY_WIDTH * 2, DISPLAY_HEIGHT * 2)),
                    tile_size: Some(Vec2::new(16., 16.)),
                    tilesheet_size: Some(Vec2::new(16., 16.)),
                },
            ],
        })
        // .add_plugin(flappy_plugin::FlappyPlugin)
        .add_plugin(rusty_dungeon_plugin::RustyDungeonPlugin)
        .run();
}

mod profiler {
    use bevy::prelude::*;
    use bevy_egui::EguiContext;

    struct ProfilerEnabled(bool);

    pub struct ProfilerPlugin;

    impl Plugin for ProfilerPlugin {
        fn build(&self, app: &mut AppBuilder) {
            app.add_system_to_stage(bevy::app::CoreStage::First, new_frame.system())
                .add_system(profiler_ui.system())
                .add_system(keyboard.system())
                .insert_resource(ProfilerEnabled(false));
        }
    }

    fn new_frame(profiler_enabled: Res<ProfilerEnabled>) {
        puffin::set_scopes_on(profiler_enabled.0);
        puffin::GlobalProfiler::lock().new_frame();
    }

    fn profiler_ui(egui_context: Res<EguiContext>, profiler_enabled: Res<ProfilerEnabled>) {
        puffin::profile_function!();

        if profiler_enabled.0 {
            bevy_egui::egui::Window::new("Profiler")
                .default_size([800., 500.])
                .show(egui_context.ctx(), |ui| puffin_egui::profiler_ui(ui));
        }
    }

    fn keyboard(
        keyboard_input: Res<Input<KeyCode>>,
        mut profiler_enabled: ResMut<ProfilerEnabled>,
    ) {
        if keyboard_input.just_pressed(KeyCode::I) {
            profiler_enabled.0 = !profiler_enabled.0;
        }
    }
}
