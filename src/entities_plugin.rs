
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use crate::component_world_entity::WorldEntity;

pub struct HelloPlugin;

fn move_entities(
    mut soldiers_query: Query<&mut Transform, With<WorldEntity>>,
){
    for mut soldier_transform in soldiers_query.iter_mut()  {
        soldier_transform.translation += Vec3::new(10.0, 0.0, 0.0);
    }

}

fn load_camera( mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn load_path( mut commands: Commands) {
    let line_shape = shapes::Line (
        Vec2::new(-100. , -35.),
        Vec2::new(-200. , -35.)
    );

    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&line_shape),
            ..default()
        },
        Fill::color(Color::CYAN),
        Stroke::new(Color::RED, 10.0),
    ));
}

fn load_entities( mut commands: Commands) {

    let shape = shapes::RegularPolygon {
        sides: 3,
        feature: shapes::RegularPolygonFeature::Radius(50.0),
        ..shapes::RegularPolygon::default()
    };

    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            ..default()
        },
        Fill::color(Color::DARK_GREEN),
        Stroke::new(Color::BLACK, 2.0),
        WorldEntity
    ));

    println!("hello there!");

}

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, load_camera)
        .add_systems(Startup, load_entities)
        .add_systems(Startup, load_path)
        .add_systems(Update, move_entities);
    }
}