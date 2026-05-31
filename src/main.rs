use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component, Debug)]
struct Name(String);

fn hello_world() {
    println!("Hello!");
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Joe".to_string())));
    commands.spawn((Person, Name("Pop".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("Hello {:?}", name);
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Joe" {
            name.0 = "Joe2".to_string();
            break;
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, (update_people, greet_people).chain()))
        .run();
}
