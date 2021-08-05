use bevy::prelude::*;
use bevy_egui::EguiContext;

struct ProfilerEnabled(bool);
struct ProfilerGuiEnabled(bool);

pub struct ProfilerPlugin;

impl Plugin for ProfilerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_to_stage(bevy::app::CoreStage::First, new_frame.system())
            .add_system(profiler_ui.system())
            .add_system(keyboard.system())
            .insert_resource(ProfilerEnabled(false))
            .insert_resource(ProfilerGuiEnabled(false));
    }
}

fn new_frame(profiler_enabled: Res<ProfilerEnabled>) {
    puffin::set_scopes_on(profiler_enabled.0);
    puffin::GlobalProfiler::lock().new_frame();
}

fn profiler_ui(egui_context: Res<EguiContext>, profiler_gui_enabled: Res<ProfilerGuiEnabled>) {
    puffin::profile_function!();

    if profiler_gui_enabled.0 {
        bevy_egui::egui::Window::new("Profiler")
            .default_size([800., 500.])
            .show(egui_context.ctx(), |ui| puffin_egui::profiler_ui(ui));
    }
}

fn keyboard(
    keyboard_input: Res<Input<KeyCode>>,
    mut profiler_enabled: ResMut<ProfilerEnabled>,
    mut profiler_gui_enabled: ResMut<ProfilerGuiEnabled>,
) {
    if keyboard_input.just_pressed(KeyCode::I) {
        profiler_enabled.0 = !profiler_enabled.0;
    }
    if keyboard_input.just_pressed(KeyCode::K) {
        profiler_gui_enabled.0 = !profiler_gui_enabled.0;
    }
}
