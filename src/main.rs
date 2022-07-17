use std::time::Duration;

use bevy::app::App;
use bevy::core::Time;
use bevy::ecs::component::Component;
use bevy::input::Input;
use bevy::math::Vec2;
use bevy::prelude::{
    Color, Commands, KeyCode, Mut, OrthographicCameraBundle, Query, Res, ResMut, Sprite,
    SpriteBundle, Timer, Transform, Vec3, With, Entity, Without,
};
use bevy::utils::default;
use bevy::DefaultPlugins;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct UpdateTimer(Timer);

struct SnakeLength(usize);

#[derive(Component)]
struct SnakeTail;

#[derive(Component)]
struct SnakeHead {
    direction: Direction,
}

#[derive(Component, Debug)]
struct SnakePart {
    index: usize,
    life_counter: u32,
}

#[derive(Component)]
struct Apple;

#[derive(Component, Clone, Copy, Debug)]
struct Pos {
    x: i32,
    y: i32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(UpdateTimer(Timer::new(Duration::from_millis(500), true)))
        .insert_resource(SnakeLength(1))
        .add_startup_system(setup)
        .add_system(draw)
        .add_system(update)
        .add_system(handle_input)
        .run();
}

fn draw(mut drawables: Query<(&Pos, &mut Transform)>) {
    for (pos, transform) in drawables.iter_mut() {
        let (pos, mut transform): (&Pos, Mut<Transform>) = (pos, transform);
        transform.translation = Vec3::new((pos.x * 16) as f32, (pos.y * 16) as f32, 0.0);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Add Apple entity
    let x = fastrand::i32(-10..11);
    let y = fastrand::i32(-10..11);
    commands
        .spawn()
        .insert_bundle((Pos { x, y }, Apple))
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::YELLOW,
                custom_size: Some(Vec2::new(16.0, 16.0)),
                ..default()
            },
            ..default()
        });

    // Add Snake head
    commands
        .spawn()
        .insert_bundle((
            Pos { x: 0, y: 0 },
            SnakePart {
                index: 0,
                life_counter: 1,
            },
            SnakeHead {
                direction: Direction::Right,
            },
        ))
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(16.0, 16.0)),
                ..default()
            },
            ..default()
        });
}

fn handle_input(keyboard_input: Res<Input<KeyCode>>, mut snake_head: Query<&mut SnakeHead>) {
    let mut snake_head = snake_head.iter_mut().next().unwrap();
    if keyboard_input.pressed(KeyCode::Up) {
        snake_head.direction = Direction::Up
    }
    if keyboard_input.pressed(KeyCode::Down) {
        snake_head.direction = Direction::Down
    }
    if keyboard_input.pressed(KeyCode::Left) {
        snake_head.direction = Direction::Left
    }
    if keyboard_input.pressed(KeyCode::Right) {
        snake_head.direction = Direction::Right
    }
}

// Managing moving the snake and decreasing life_counter
fn update(
    mut commands: Commands,
    mut update_timer: ResMut<UpdateTimer>,
    time: Res<Time>,
    snake_head_info: Query<(Entity, &SnakeHead, &Pos)>,
    mut snake_length: ResMut<SnakeLength>,
    mut snake_parts: Query<(&mut SnakePart, Entity)>,
    mut apple: Query<&mut Pos, (With<Apple>, Without<SnakeHead>)>,
) {
    if update_timer.0.tick(time.delta()).just_finished() {
        println!("Starting new update sequence");
        let (entity, snake_head, head_pos) = snake_head_info.iter().next().unwrap();

        // Calculating next pos of the head
        let next_pos = match snake_head.direction {
            Direction::Up => Pos {
                x: head_pos.x,
                y: head_pos.y + 1,
            },
            Direction::Down => Pos {
                x: head_pos.x,
                y: head_pos.y - 1,
            },
            Direction::Left => Pos {
                x: head_pos.x - 1,
                y: head_pos.y,
            },
            Direction::Right => Pos {
                x: head_pos.x + 1,
                y: head_pos.y,
            },
        };
        // Removing SnakeHead component of the current head before creating a new one
        commands
            .entity(entity)
            .remove::<SnakeHead>();

        // Spawning new head entity, WARNING, from here all previous data about the head (head_pos, entity) changed
        let new_head_entity = commands
            .spawn()
            .insert_bundle((next_pos, SnakePart { index: 0, life_counter: snake_length.0 as u32 }, SnakeHead { direction: snake_head.direction }))
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(16.0, 16.0)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new((next_pos.x * 16) as f32, (next_pos.y * 16) as f32, 0.0),
                    ..default()
                },
                ..default()
            }).id();

        println!("Decreasing all life_counter and increasing index");
        // Decrease all life_counter and increasing all index
        for (mut snake_part, _) in snake_parts.iter_mut() {
            snake_part.index += 1;
            snake_part.life_counter -= 1; // Can overflow, we have to remove entity with a life_counter of 0
            println!("index: {:?}, life_counter: {:?}", snake_part.index, snake_part.life_counter)
        }

        let mut apple_pos = apple.iter_mut().next().unwrap();

        // Check if snake head is on the apple, if yes we increase by 1 each life_counter to prevent last snake part to be despawned
        if next_pos.x == apple_pos.x && next_pos.y == apple_pos.y {
            snake_length.0 += 1;
            // Calculating new pos for apple
            let x = fastrand::i32(-10..11);
            let y = fastrand::i32(-10..11);
            apple_pos.x = x;
            apple_pos.y = y;
            println!("Detected head on the apple, increasing life_counter");
            for (mut snake_part, _) in snake_parts.iter_mut() { // ERROR : New head life counter is not increased !!
                snake_part.life_counter += 1;
                println!("index: {:?}, life_counter: {:?}", snake_part.index, snake_part.life_counter)
            }
            commands.entity(new_head_entity).remove::<SnakePart>().insert(SnakePart { index: 0, life_counter: snake_length.0 as u32 });
        }

        // Remove each part with a life_counter of 0
        for (snake_part, entity) in snake_parts.iter() {
            if snake_part.life_counter == 0 {
                commands
                    .entity(entity)
                    .despawn();
            }
        }

        

    }
}
