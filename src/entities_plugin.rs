
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use crate::component_world_entity::WorldEntity;

pub struct HelloPlugin;

fn move_entities(
    mut soldiers_query: Query<&mut Transform, With<WorldEntity>>,
){
    for mut soldier_transform in soldiers_query.iter_mut()  {
        soldier_transform.translation += Vec3::new(5.0, 0.0, 0.0);
    }

}

fn load_entities(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
){
    commands.spawn(Camera2dBundle::default());

    let texture_handle: Handle<Image> = asset_server.load("Sprites/sheet.png");
    let texture_atlas =
    TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 1, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn((SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform{
                translation: Vec3 { x: 100.0, y: 10.0, z: 1.0 },
                ..Default::default()
            },
            ..default()
        }, WorldEntity)
    );

    let shape = shapes::RegularPolygon {
        sides: 6,
        feature: shapes::RegularPolygonFeature::Radius(200.0),
        ..shapes::RegularPolygon::default()
    };

    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            ..default()
        },
        Fill::color(Color::CYAN),
        Stroke::new(Color::BLACK, 10.0),
    ));

    println!("hello there!");

}

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, load_entities)
        .add_systems(Update, move_entities);
    }
}