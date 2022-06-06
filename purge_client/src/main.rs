use purge_common;
use bevy::prelude::*;

fn main() {
    println!("{}", purge_common::hello_world("Purge".to_string()));
    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}