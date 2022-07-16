use std::time::Duration;
use bevy::app::App;
use bevy::DefaultPlugins;
use bevy::prelude::{Color, Commands, default, Entity, Input, KeyCode, Mut, OrthographicCameraBundle, Query, Res, ResMut, Sprite, SpriteBundle, Time, Timer, Transform, Vec2, Visibility, With};
use bevy::ecs::component::Component;
use bevy::math::Vec3;

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct UpdateTimer(Timer);

struct SnakeLength(u32);

struct EntityWithSpriteBundle(Vec<Entity>);

#[derive(Component)]
struct SnakeHead;

#[derive(Component, Copy, Clone)]
struct SnakePartInfo(u32, Direction);

#[derive(Component, Copy, Clone)]
struct Pos(i32, i32); // x, y

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(UpdateTimer(Timer::new(Duration::from_secs(1), true)))
        .insert_resource(SnakeLength(1))
        .insert_resource(EntityWithSpriteBundle(vec![]))
        .add_startup_system(setup)
        .add_system(update_head_direction)
        .add_system(update)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle((SnakeHead, SnakePartInfo(0, Direction::Right), Pos(0, 0)));
}

fn update_head_direction(mut snake_head: Query<&mut SnakePartInfo, With<SnakeHead>>, keyboard_input: Res<Input<KeyCode>>) {
    let mut snake_head = snake_head.iter_mut().next().unwrap();
    if keyboard_input.pressed(KeyCode::Up) { snake_head.1 = Direction::Up; println!("Up pressed"); }
    if keyboard_input.pressed(KeyCode::Left) { snake_head.1 = Direction::Left; println!("Left pressed"); }
    if keyboard_input.pressed(KeyCode::Down) { snake_head.1 = Direction::Down; println!("Down pressed"); }
    if keyboard_input.pressed(KeyCode::Right) { snake_head.1 = Direction::Right; println!("Right pressed"); }
}

fn update(mut entities: ResMut<EntityWithSpriteBundle>, mut update_timer: ResMut<UpdateTimer>, time: Res<Time>, mut commands: Commands, mut snake_parts: Query<(&mut SnakePartInfo, &mut Pos)>, mut visibles: Query<&mut Visibility>) {
    if update_timer.0.tick(time.delta()).just_finished() {
        println!("Start Update");
        for (mut snake_part_info, mut pos) in snake_parts.iter_mut() {
            let (mut snake_part_info, mut pos): (Mut<SnakePartInfo>, Mut<Pos>) = (snake_part_info, pos);
            match snake_part_info.1 {
                Direction::Up => { pos.1 += 1 }
                Direction::Down => { pos.1 -= 1 }
                Direction::Left => { pos.0 -= 1 }
                Direction::Right => { pos.0 += 1}
            }
        }
    }
    for mut visibility in visibles.iter_mut() {
        visibility.is_visible = false;
    }
    for (snake_part_info, pos) in snake_parts.iter() {
        let (snake_part_info, pos): (&SnakePartInfo, &Pos) = (snake_part_info, pos);
        entities.0.push(commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(16.0, 16.0)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new((pos.0 * 16) as f32, (pos.1 * 16) as f32, 0.0),
                ..default()
            },
            ..default()
        }).id());
    }
}