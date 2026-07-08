/// Tests back-to-front sorting of translucent polylines.
use bevy::{camera::Hdr, prelude::*};
use bevy_polyline::prelude::*;
use std::f32::consts::FRAC_PI_2;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PolylinePlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, orbit_camera)
        .run();
}

fn setup(
    mut commands: Commands,
    mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
    mut polylines: ResMut<Assets<Polyline>>,
) {
    for (z, color) in [
        (-1.0, LinearRgba::new(1.0, 0.0, 0.0, 0.5)),
        (0.0, LinearRgba::new(0.0, 1.0, 0.0, 0.5)),
        (1.0, LinearRgba::new(0.0, 0.0, 1.0, 0.5)),
    ] {
        commands.spawn(PolylineBundle {
            polyline: PolylineHandle(polylines.add(Polyline {
                vertices: vec![Vec3::new(-1.0, -1.0, 0.0), Vec3::new(1.0, 1.0, 0.0)],
            })),
            material: PolylineMaterialHandle(polyline_materials.add(PolylineMaterial {
                width: 30.0,
                color,
                perspective: false,
                ..default()
            })),
            transform: Transform::from_xyz(0.0, 0.0, z),
            ..default()
        });
    }

    commands.spawn((
        Camera3d::default(),
        Camera::default(),
        Hdr,
        Msaa::Sample4,
        Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn orbit_camera(time: Res<Time>, mut cameras: Query<&mut Transform, With<Camera3d>>) {
    // Make sure the initial view rotation is not close to identity to expose sorting errors.
    let angle = time.elapsed_secs() * 0.5 + FRAC_PI_2;
    for mut transform in &mut cameras {
        let position = Vec3::new(angle.sin() * 5.0, 1.5, angle.cos() * 5.0);
        *transform = Transform::from_translation(position).looking_at(Vec3::ZERO, Vec3::Y);
    }
}
