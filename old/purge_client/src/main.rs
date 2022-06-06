use bevy::{
    input::{keyboard::KeyboardInput, ElementState},
    input::mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    render::camera::{ActiveCamera, Camera3d},
    window::CursorMoved,
    ecs::system::Res,
    prelude::*,
};
// use bevy_config_cam::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
//
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::{errors::NearlySingularConversion, orientation::Direction};

use bevy_config_cam::next_enum;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

// Used in queries when you want filter between cameras
#[derive(Clone, Eq, PartialEq, Debug, Hash, EnumIter, Component)]
enum SwitchableCameras {
    ForwardCam,
    ReverseCam,
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(InputManagerPlugin::<Action>::default())
        .add_plugin(EguiPlugin)
        // .add_plugin(ConfigCam)
        // .insert_resource(Msaa { samples: 4 })
        // .insert_resource(MovementSettings {
        //     sensitivity: 0.00015, // default: 0.00012
        //     speed: 12.0,          // default: 12.0
        //     ..Default::default()
        // })
        // .insert_resource(PlayerSettings {
        //     pos: Vec3::new(2., 0., 0.),
        //     player_asset: "models/shuttle.gltf#Scene0",
        //     ..Default::default()
        // })
        .add_state(SwitchableCameras::ForwardCam)
        .add_startup_system(setup)
        .add_system(cycle_camera_state)
        .add_system(switch_camera)

        // .add_startup_system(spawn_arena)
        // .add_startup_system(spawn_player)
        .add_system(ui_example)
        // .add_system(jump)
        .run();
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Action {
    Run,
    Jump,
}
//
// #[derive(Component)]
// struct Player;

// fn spawn_player(mut commands: Commands) {
//     commands
//         .spawn()
//         .insert(Player)
//         .insert_bundle(InputManagerBundle::<Action> {
//             // Stores "which actions are currently pressed"
//             action_state: ActionState::default(),
//             // Describes how to convert from player inputs into those actions
//             input_map: InputMap::new([(Action::Jump, KeyCode::Space)]),
//         });
// }

// Query for the `ActionState` component in your game logic systems!
// fn jump(query: Query<&ActionState<Action>, With<Player>>) {
//     let action_state = query.single();
//     // Each action has a button-like state of its own that you can check
//     if action_state.just_pressed(Action::Jump) {
//         println!("I'm jumping!");
//     }
// }

// fn spawn_arena(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     mut cl: ResMut<CamLogic>,
// ) {
//     // plane
//     commands.spawn_bundle(PbrBundle {
//         mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
//         material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
//         ..Default::default()
//     });
//
//     // cube, set as target
//     cl.target = Some(
//         commands
//             .spawn_bundle(PbrBundle {
//                 mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
//                 material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
//                 transform: Transform::from_xyz(0.0, 0.5, 0.0),
//                 ..Default::default()
//             })
//             .id(),
//     );
//
//     // light
//     commands.spawn_bundle(PointLightBundle {
//         transform: Transform::from_xyz(4.0, 8.0, 4.0),
//         ..Default::default()
//     });
// }
//
fn ui_example(mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("Hello").show(egui_context.ctx_mut(), |ui| {
        ui.label("world");
    });
}


/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });
    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(StandardMaterial {
            base_color: Color::rgba(0.8, 0.7, 0.6, 0.1).into(),
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    });
    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    // forward camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(SwitchableCameras::ForwardCam);

    // reverse camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 10.0, 0.1).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(SwitchableCameras::ReverseCam);
}

fn switch_camera(
    mut active_cams: ResMut<ActiveCamera<Camera3d>>,
    cam_state: ResMut<State<SwitchableCameras>>,
    mut query: Query<(&SwitchableCameras, Entity), With<Camera3d>>,
) {
    // find the camera with the current state, set its name to the 3d camera name
    query
        .iter_mut()
        .filter(|(switchable_cams, _)| cam_state.current().eq(switchable_cams))
        .for_each(|(_, camera_entity): (&SwitchableCameras, Entity)| {
            active_cams.set(camera_entity);
        });
}

fn cycle_camera_state(
    mut selected_cam: ResMut<State<SwitchableCameras>>,
    mut keys: ResMut<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::E) {
        let new_cam_state = next_enum!(SwitchableCameras, selected_cam);
        println!("New camera: {:?}", new_cam_state);
        selected_cam.set(new_cam_state).unwrap();

        keys.reset(KeyCode::E);
    }
}