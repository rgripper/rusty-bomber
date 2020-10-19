use bevy::prelude::*;

fn hello_world() {
    println!("hello world!");
}

struct Person;

struct Name(String);

fn add_people(mut commands: Commands) {
    commands
        .spawn((Person, Name("Elaina Proctor".to_string())))
        .spawn((Person, Name("Renzo Hume".to_string())))
        .spawn((Person, Name("Zayna Nieves".to_string())));
}

fn greet_people(_person: &Person, name: &Name) {
    println!("hello {}!", name.0);
}

fn main() {
    App::build()
        .add_startup_system(add_people.system())
        .add_system(hello_world.system())
        .add_system(greet_people.system())
        .run();
}