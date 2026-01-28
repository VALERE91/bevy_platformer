use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use crate::state::BPGameState;

pub struct BPUIPlugin;

impl Plugin for BPUIPlugin {
    fn build(&self, app: &mut App) {
        app
                .add_plugins(InputManagerPlugin::<BPGameOverUiAction>::default())
                .add_systems(OnEnter(BPGameState::GameOver), setup_game_over_ui)
                .add_systems(OnExit(BPGameState::GameOver), cleanup_game_over_ui)
                .add_systems(OnEnter(BPGameState::Victory), setup_victory_ui)
                .add_systems(OnExit(BPGameState::Victory), cleanup_victory_ui)
                .add_systems(Update, ui_update.run_if(in_state(BPGameState::GameOver)
                                                        .or(in_state(BPGameState::Victory))));
    }
}

#[derive(Component)]
pub struct BPUiGameOverMarker;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum BPGameOverUiAction {
    Restart,
    Quit,
}

fn setup_game_over_ui(mut commands: Commands){
    let input_map = InputMap::default()
        .with(BPGameOverUiAction::Restart, KeyCode::KeyR)
        .with(BPGameOverUiAction::Quit, KeyCode::KeyQ);

    commands.spawn((
        BPUiGameOverMarker,
        Node {
            // Take up the whole screen
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            // Center the child (Text) horizontally
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            // Center the child (Text) vertically
            align_items: AlignItems::Center,
            align_content: AlignContent::Center,
            ..default()
        },
        ZIndex(2),
        input_map,
    )).with_children(|parent| {
        parent.spawn((
            Text::new("GAME OVER"),
            TextFont::from_font_size(60.0),
            TextColor(Color::srgb(1.0, 0.0, 0.0)),
        ));
        parent.spawn((
            Text::new("Press R to restart or Q to quit"),
            TextFont::from_font_size(40.0),
            TextColor(Color::srgb(1.0, 0.0, 0.0)),
        ));
    });
}

fn cleanup_game_over_ui(mut commands: Commands,
                        query: Query<Entity, With<BPUiGameOverMarker>>){
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn setup_victory_ui(mut commands: Commands){
    let input_map = InputMap::default()
        .with(BPGameOverUiAction::Restart, KeyCode::KeyR)
        .with(BPGameOverUiAction::Quit, KeyCode::KeyQ);

    commands.spawn((
        BPUiGameOverMarker,
        Node {
            // Take up the whole screen
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            // Center the child (Text) horizontally
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            // Center the child (Text) vertically
            align_items: AlignItems::Center,
            align_content: AlignContent::Center,
            ..default()
        },
        ZIndex(2),
        input_map,
    )).with_children(|parent| {
        parent.spawn((
            Text::new("VICTORY IS YOURS!"),
            TextFont::from_font_size(60.0),
            TextColor(Color::srgb(0.0, 1.0, 0.0)),
        ));
        parent.spawn((
            Text::new("Press R to restart or Q to quit"),
            TextFont::from_font_size(40.0),
            TextColor(Color::srgb(0.0, 1.0, 0.0)),
        ));
    });
}

fn cleanup_victory_ui(mut commands: Commands,
                        query: Query<Entity, With<BPUiGameOverMarker>>){
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn ui_update(query: Query<&ActionState<BPGameOverUiAction>, With<BPUiGameOverMarker>>,
                    mut next_state: ResMut<NextState<BPGameState>>,
                    mut exit_message_writer: MessageWriter<AppExit>){
    for action_state in &query {
        if action_state.just_pressed(&BPGameOverUiAction::Restart) {
            next_state.set(BPGameState::InGame);
            continue;
        }

        if action_state.just_pressed(&BPGameOverUiAction::Quit) {
            exit_message_writer.write(AppExit::Success);
        }
    }
}