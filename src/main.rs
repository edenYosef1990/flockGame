use bevy::prelude::*;
use entities_plugin::HelloPlugin;
mod entities_plugin;
mod component_world_entity;
use bevy_prototype_lyon::prelude::*;

fn hello_world() {
    //println!("hello world!123");
}

fn main() {
    App::new()
    .insert_resource(Msaa::Sample4)
    .add_plugins((DefaultPlugins,HelloPlugin))
    .add_plugins(ShapePlugin)
    .add_systems(Update, hello_world)
    .run();
}