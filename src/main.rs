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

use ascii_tilemap_plugin::{settings::AsciiTilemapSettings, AsciiTilemapPlugin};
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_egui::EguiPlugin;

mod ascii_tilemap_plugin;
mod flappy_plugin;
mod rusty_dungeon_plugin;

// TODO
// * find a way to control the window dimension from the plugin or update the tilemap size on resize

pub const WIDTH: u32 = 80;
pub const HEIGHT: u32 = 50;

pub const DISPLAY_WIDTH: u32 = WIDTH / 2;
pub const DISPLAY_HEIGHT: u32 = HEIGHT / 2;

pub const TILE_WIDTH: u32 = 32;
pub const TILE_HEIGHT: u32 = 32;

fn main() {
    let settings = AsciiTilemapSettings::builder()
        // .with_tilesheet_path("16x16-sb-ascii.png")
        .with_tilesheet_path("dungeonfont.png")
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(TILE_WIDTH, TILE_HEIGHT)
        .with_chunks(1, 1)
        // map
        .with_layer(0, false, false)
        // entities
        .with_layer(1, true, true)
        // diagnostic
        .with_layer(2, true, false)
        .build();

    App::build()
        .insert_resource(WindowDescriptor {
            // TODO find a way to control this by the plugin
            // if they don't match the map will not be aligned properly
            width: settings.window_width(),
            height: settings.window_height(),
            title: String::from("hands on dungeon crawler"),
            vsync: false,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(AsciiTilemapPlugin)
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .insert_resource(settings)
        // .add_plugin(flappy_plugin::FlappyPlugin)
        .add_plugin(rusty_dungeon_plugin::RustyDungeonPlugin)
        .add_plugin(profiler::ProfilerPlugin)
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
        if keyboard_input.just_pressed(KeyCode::P) {
            profiler_enabled.0 = !profiler_enabled.0;
        }
    }
}
