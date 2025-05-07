// TODO: understand what prelude is
// (I've learned this before but forgot what it means)
use bevy::prelude::*;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Person;

struct HelloPlugin;

#[derive(Resource)]
struct GreetTimer(Timer);

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_people);
        app.add_systems(Update, (update_people, greet_people).chain());
    }
}

fn add_people(mut commands: Commands) {
    // TODO: understand what commands are
    commands.spawn((Person, Name("Bruno".to_string())));
    commands.spawn((Person, Name("Bia".to_string())));
    commands.spawn((Person, Name("Rodrigo".to_string())));
    commands.spawn((Person, Name("Rosane".to_string())));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("Hello {}!", name.0)
        }
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Bruno" {
            name.0 = "Bruno Mello".to_string();
            break;
        }
    }
}

fn main() {
    App::new()
        // TODO: revisar essa estrutura de  resource com o timer
        .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .run();
}
