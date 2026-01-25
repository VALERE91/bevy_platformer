use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::{EguiContext, EguiPlugin, EguiPrimaryContextPass, PrimaryEguiContext};
use bevy_inspector_egui::egui;
use leafwing_input_manager::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct BPDebugPlugin;

impl Plugin for BPDebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(RapierDebugRenderPlugin::default())
            .add_plugins(EguiPlugin::default())
            .add_plugins(bevy_inspector_egui::DefaultInspectorConfigPlugin)
            .add_plugins(InputManagerPlugin::<DebugAction>::default())
            .add_systems(Startup, setup_debug)
            .add_systems(Update, toggle_rapier_debug_system)
            .add_systems(EguiPrimaryContextPass, toggle_egui_debug_system);
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum DebugAction {
    TogglePhysicsLines,
    ToggleInspector,
}

#[derive(Component)]
pub struct BPDebugMarker;

#[derive(Component)]
pub struct BPDebugState {
    pub show_inspector: bool,
    pub show_rapier_debug: bool,
}

fn setup_debug(mut commands: Commands){
    let input_map = InputMap::default()
        .with(DebugAction::ToggleInspector, KeyCode::F1)
        .with(DebugAction::TogglePhysicsLines, KeyCode::F2);

    commands.spawn((
        BPDebugState { show_inspector: false, show_rapier_debug: false },
        BPDebugMarker,
        input_map
    ));
}

fn toggle_rapier_debug_system(mut query: Query<(&ActionState<DebugAction>, &mut BPDebugState), With<BPDebugMarker>>,
                mut rapier_debug: ResMut<DebugRenderContext>){
    if let Ok(mut action) = query.single_mut() {
        if action.0.just_pressed(&DebugAction::TogglePhysicsLines) {
            rapier_debug.enabled = !rapier_debug.enabled;
            action.1.show_rapier_debug = rapier_debug.enabled;
        }
    }
}

fn toggle_egui_debug_system(world: &mut World){
    let Ok(egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryEguiContext>>()
        .single(world)
    else {
        return;
    };
    let mut egui_context = egui_context.clone();

    let Ok(mut debug_context) = world
        .query_filtered::<(&ActionState<DebugAction>, &mut BPDebugState), With<BPDebugMarker>>()
        .single_mut(world)
    else {
        return;
    };

    if debug_context.0.just_pressed(&DebugAction::ToggleInspector) {
        debug_context.1.show_inspector = !debug_context.1.show_inspector;
    }

    if !debug_context.1.show_inspector{
        return;
    }

    egui::Window::new("UI").show(egui_context.get_mut(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            // equivalent to `WorldInspectorPlugin`
            bevy_inspector_egui::bevy_inspector::ui_for_world(world, ui);

            egui::CollapsingHeader::new("Materials").show(ui, |ui| {
                bevy_inspector_egui::bevy_inspector::ui_for_assets::<StandardMaterial>(world, ui);
            });

            ui.heading("Entities");
            bevy_inspector_egui::bevy_inspector::ui_for_entities(world, ui);
        });
    });
}