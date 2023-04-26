use bevy::{
    input::mouse::MouseButtonInput,
    prelude::*,
    sprite::MaterialMesh2dBundle,
    window::{Window},
};

use crate::main_camera::MainCamera;

pub struct CellsPlugin;

#[derive(Component)]
struct MovingShape {
    moving_vect: Vec3,
}

impl Plugin for CellsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_shape_system)
            .add_system(mouse_button_events);
    }
}

fn move_shape_system(
    mut commands: Commands,
    mut query: Query<(&mut Transform, &mut MovingShape), With<MovingShape>>,
    time: Res<Time>,
) {
    for mut transform in query.iter_mut() {
        let moving_vect = transform.1.moving_vect.clone();
        transform.0.translation = transform.0.translation + moving_vect;
    }
}

fn mouse_button_events(
    mut commands: Commands,
    mut events: EventReader<MouseButtonInput>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    use bevy::input::ButtonState;

    for ev in events.iter() {
        match ev.state {
            ButtonState::Pressed => {
                let (camera, camera_transform) = camera_q.single();
                let window = windows.single();

                if let Some(world_position) = window
                    .cursor_position()
                    .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                    .map(|ray| ray.origin.truncate())
                {
                    commands.spawn((
                        MaterialMesh2dBundle {
                            mesh: meshes.add(shape::Circle::new(15.).into()).into(),
                            material: materials.add(ColorMaterial::from(Color::PURPLE)),
                            transform: Transform::from_xyz(world_position.x, world_position.y, 1.),
                            ..default()
                        },
                        MovingShape {
                            moving_vect: Vec3::new(-1., 1., 0.),
                        },
                    ));
                }
            }
            ButtonState::Released => {}
        }
    }
}
