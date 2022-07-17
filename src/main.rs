use std::process::id;
use std::time::Duration;
use bevy::app::App;
use bevy::DefaultPlugins;
use bevy::prelude::{Color, Commands, default, Entity, Input, KeyCode, Mut, OrthographicCameraBundle, Query, Res, ResMut, Sprite, SpriteBundle, Time, Timer, Transform, Vec2, Visibility, With, Without};
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

struct SnakePartEntities(Vec<Entity>);

struct DirectionBuffer(Vec<Direction>);

struct ApplePos((i32, i32));

struct LastPartInfo(Pos, Direction);

#[derive(Component)]
struct Apple;

#[derive(Component)]
struct SnakeHead;

#[derive(Component, Copy, Clone)]
struct SnakePartInfo(usize, Direction);

#[derive(Component, Copy, Clone)]
struct Pos(i32, i32); // x, y

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(UpdateTimer(Timer::new(Duration::from_millis(500), true)))
        .insert_resource(SnakeLength(1))
        .insert_resource(SnakePartEntities(vec![]))
        .insert_resource(DirectionBuffer(vec![]))
        .insert_resource(LastPartInfo(Pos(0, 0), Direction::Right))
        .insert_resource(ApplePos((0, 0)))
        .add_startup_system(setup)
        .add_system(update_head_direction)
        .add_system(update)
        .run();
}

fn setup(mut last_part_info: ResMut<LastPartInfo>, mut commands: Commands, mut snake_part_entities: ResMut<SnakePartEntities>, mut direction_buffer: ResMut<DirectionBuffer>, mut apple_pos: ResMut<ApplePos>, mut snake_length: ResMut<SnakeLength>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn().insert_bundle((Apple, Pos(10, 10))).insert_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::YELLOW,
            custom_size: Some(Vec2::new(16.0, 16.0)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..default()
        },
        ..default()
    });
    apple_pos.0.0 = 10;
    apple_pos.0.1 = 10;
    snake_part_entities.0.push(commands.spawn().insert_bundle((SnakeHead, SnakePartInfo(0, Direction::Right), Pos(0, 0))).insert_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::RED,
            custom_size: Some(Vec2::new(16.0, 16.0)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..default()
        },
        ..default()
    }).id()); // Registering head entity
    direction_buffer.0.push(Direction::Right);
    snake_part_entities.0.push(commands.spawn().insert_bundle((SnakePartInfo(1, Direction::Right), Pos(-1, 0))).insert_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::RED,
            custom_size: Some(Vec2::new(16.0, 16.0)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..default()
        },
        ..default()
    }).id()); // Registering body
    direction_buffer.0.push(Direction::Right);
    snake_part_entities.0.push(commands.spawn().insert_bundle((SnakePartInfo(2, Direction::Right), Pos(-2, 0))).insert_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::RED,
            custom_size: Some(Vec2::new(16.0, 16.0)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..default()
        },
        ..default()
    }).id()); // Registering body
    direction_buffer.0.push(Direction::Right);
    last_part_info.0 = Pos(-2, 0);
    last_part_info.1 = Direction::Right;
    snake_length.0 = 3;
}

fn update_head_direction(mut snake_head: Query<&mut SnakePartInfo, With<SnakeHead>>, keyboard_input: Res<Input<KeyCode>>) {
    let mut snake_head = snake_head.iter_mut().next().unwrap();
    if keyboard_input.pressed(KeyCode::Up) { snake_head.1 = Direction::Up; println!("Up pressed"); }
    if keyboard_input.pressed(KeyCode::Left) { snake_head.1 = Direction::Left; println!("Left pressed"); }
    if keyboard_input.pressed(KeyCode::Down) { snake_head.1 = Direction::Down; println!("Down pressed"); }
    if keyboard_input.pressed(KeyCode::Right) { snake_head.1 = Direction::Right; println!("Right pressed"); }
}

fn update(mut last_part_info: ResMut<LastPartInfo>, mut snake_part_entities: ResMut<SnakePartEntities>, mut update_timer: ResMut<UpdateTimer>, time: Res<Time>, mut commands: Commands, mut snake_parts: Query<(&mut SnakePartInfo, &mut Pos, &mut Transform)>, mut direction_buffer: ResMut<DirectionBuffer>, mut apple: Query<(&mut Pos, &mut Transform, &mut Apple), Without<SnakePartInfo>>, mut apple_pos: ResMut<ApplePos>, mut snake_length: ResMut<SnakeLength>) {
    if update_timer.0.tick(time.delta()).just_finished() {
        println!("Start Update");
        for (mut snake_part_info, mut pos, mut transform) in snake_parts.iter_mut() {
            let (mut snake_part_info, mut pos, mut transform): (Mut<SnakePartInfo>, Mut<Pos>, Mut<Transform>) = (snake_part_info, pos, transform);
            match snake_part_info.1 {
                Direction::Up => { pos.1 += 1 }
                Direction::Down => { pos.1 -= 1 }
                Direction::Left => { pos.0 -= 1 }
                Direction::Right => { pos.0 += 1 }
            }
            direction_buffer.0[snake_part_info.0] = snake_part_info.1;
            if snake_part_info.0 == (snake_length.0 - 1) as usize {
                last_part_info.0 = Pos(pos.0, pos.1);
            }
            if snake_part_info.0 != 0 {
                snake_part_info.1 = direction_buffer.0[snake_part_info.0 - 1];
            } else {
                if pos.0 == apple_pos.0.0 && pos.1 == apple_pos.0.1 {
                    let index = snake_length.0 as usize;
                    let direction = direction_buffer.0.clone().last().unwrap().clone();
                    let new_pos = match direction_buffer.0.last().unwrap() {
                        Direction::Up => { Pos(last_part_info.0.0, last_part_info.0.1) }
                        Direction::Down => { Pos(last_part_info.0.0, last_part_info.0.1) }
                        Direction::Left => { Pos(last_part_info.0.0, last_part_info.0.1) }
                        Direction::Right => { Pos(last_part_info.0.0, last_part_info.0.1) }
                    };
                    snake_part_entities.0.push(commands.spawn().insert_bundle((SnakePartInfo(index, direction), new_pos)).insert_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::RED,
                            custom_size: Some(Vec2::new(16.0, 16.0)),
                            ..default()
                        },
                        transform: Transform {
                            translation: Vec3::new(0.0, 0.0, 0.0),
                            ..default()
                        },
                        ..default()
                    }).id()); // Registering body
                    direction_buffer.0.push(direction);
                    snake_length.0 += 1;
                }
            }
        }
    }
    println!("{}", snake_parts.iter().count());
    for (snake_part_info, pos, mut transform) in snake_parts.iter_mut() {
        let (snake_part_info, pos, mut transform): (Mut<SnakePartInfo>, Mut<Pos>, Mut<Transform>) = (snake_part_info, pos, transform);
        transform.translation = Vec3::new((pos.0 * 16) as f32, (pos.1 * 16) as f32, 0.0);
    }
    let (mut pos, mut transform, mut apple): (Mut<Pos>, Mut<Transform>, Mut<Apple>) = apple.iter_mut().next().unwrap();
    transform.translation = Vec3::new((pos.0 * 16) as f32, (pos.1 * 16) as f32, 0.0);
}