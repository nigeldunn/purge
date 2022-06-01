use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(hello)
        .run();
}

fn hello() {
    println!("Hello, world!");
}