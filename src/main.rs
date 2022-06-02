use bevy::{
    input::{keyboard::KeyboardInput, ElementState},
    input::mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    window::CursorMoved,
    ecs::system::Res,
    prelude::*,
};
use bevy_config_cam::ConfigCam;
use bevy_egui::{egui, EguiContext, EguiPlugin};

use leafwing_input_manager::prelude::*;
use leafwing_input_manager::{errors::NearlySingularConversion, orientation::Direction};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(InputManagerPlugin::<Action>::default())
        .add_plugin(EguiPlugin)
        .add_plugin(ConfigCam)
        .add_startup_system(spawn_arena)
        .add_startup_system(spawn_player)
        .add_system(ui_example)
        .add_system(jump)
        .run();
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Action {
    Run,
    Jump,
}

#[derive(Component)]
struct Player;

fn spawn_player(mut commands: Commands) {
    commands
        .spawn()
        .insert(Player)
        .insert_bundle(InputManagerBundle::<Action> {
            // Stores "which actions are currently pressed"
            action_state: ActionState::default(),
            // Describes how to convert from player inputs into those actions
            input_map: InputMap::new([(Action::Jump, KeyCode::Space)]),
        });
}

// Query for the `ActionState` component in your game logic systems!
fn jump(query: Query<&ActionState<Action>, With<Player>>) {
    let action_state = query.single();
    // Each action has a button-like state of its own that you can check
    if action_state.just_pressed(Action::Jump) {
        println!("I'm jumping!");
    }
}

fn spawn_arena(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 10. })),
            material: materials.add(StandardMaterial {
                base_color: Color::rgba(0.5, 0.5, 0.5, 1.0),
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..default()
            }),
            transform: Transform::from_translation(Vec3::new(1., 1., 1.)),
            ..Default::default()
        });
}

fn ui_example(mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("Hello").show(egui_context.ctx_mut(), |ui| {
        ui.label("world");
    });
}
