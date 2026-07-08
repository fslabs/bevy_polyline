use bevy::{camera::Hdr, prelude::*};
use bevy_polyline::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PolylinePlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
    mut polylines: ResMut<Assets<Polyline>>,
) {
    commands.spawn(PolylineBundle {
        polyline: PolylineHandle(polylines.add(Polyline {
            vertices: vec![Vec3::new(-1.0, -1.0, 0.0), Vec3::new(1.0, 1.0, 0.0)],
        })),
        material: PolylineMaterialHandle(polyline_materials.add(PolylineMaterial {
            width: 100.0,
            color: LinearRgba::new(1.0, 0.0, 0.0, 1.0),
            perspective: false,
            ..default()
        })),
        ..default()
    });

    commands.spawn(PolylineBundle {
        polyline: PolylineHandle(polylines.add(Polyline {
            vertices: vec![Vec3::new(-1.0, 1.0, 0.1), Vec3::new(1.0, -1.0, 0.1)],
        })),
        material: PolylineMaterialHandle(polyline_materials.add(PolylineMaterial {
            width: 100.0,
            color: LinearRgba::new(0.0, 1.0, 0.0, 0.5),
            perspective: false,
            ..default()
        })),
        ..default()
    });

    // camera
    commands.spawn((
        Camera3d::default(),
        Camera::default(),
        Hdr,
        Msaa::Sample4,
        Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
