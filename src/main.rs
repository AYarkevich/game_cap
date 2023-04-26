use bevy::{
    prelude::*,
    window::{PresentMode, Window, WindowPlugin},
};
use bevy_pancam::*;

mod debug;
use debug::DebugPlugin;

mod info_board;
use info_board::InfoBoardPlugin;

mod cells;
use cells::CellsPlugin;

mod main_camera;
use main_camera::MainCamera;

struct ViewPortMetrics {
    width: f32,
    height: f32,
}

const MAIN_VIEW_PORT_METRICS: ViewPortMetrics = ViewPortMetrics {
    width: 500.0,
    height: 500.0,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Game_cap!".into(),
                resolution: (900., 600.).into(),
                present_mode: PresentMode::AutoVsync,
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(DebugPlugin)
        .add_plugin(InfoBoardPlugin)
        .add_plugin(CellsPlugin)
        .add_plugin(PanCamPlugin::default())
        .insert_resource(ClearColor(Color::MAROON))
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((Camera2dBundle::default(), MainCamera))
    .insert(PanCam {
        grab_buttons: vec![MouseButton::Middle], // which buttons should drag the camera
        enabled: true, // when false, controls are disabled. See toggle example.
        zoom_to_cursor: true, // whether to zoom towards the mouse or the center of the screen
        min_scale: 1., // prevent the camera from zooming too far in
        max_scale: Some(40.),
        ..default()// prevent the camera from zooming too far out
    });

    // Border
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(
                MAIN_VIEW_PORT_METRICS.width,
                MAIN_VIEW_PORT_METRICS.height,
            )),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..default()
    });
}
