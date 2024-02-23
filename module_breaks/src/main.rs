use bevy::math;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (move_paddle, apply_velocity))
        .run();
}

// 길이 px인가 mm인가, 시간 s

// PADDLE
const PADDLE_START_Y: f32 = 0.0;
const PADDLE_SIZE: Vec2 = Vec2::new(120.0, 20.0);
const PADDLE_COLOR: Color = Color::rgb(0.3, 0.2, 0.5);
const PADDLE_SPEED: f32 = 700.0;

// BALL
const BALL_SIZE: Vec2 = Vec2::new(20.0, 20.0);
const BALL_COLOR: Color = Color::rgb(0.9, 0.1, 0.2);
const BALL_SPEED: f32 = 500.0;
const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.);
const BALL_INITIAL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);


#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Ball;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // camera
    commands.spawn(Camera2dBundle::default());

    // paddle
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: math::vec3(0., PADDLE_START_Y, 0.),
                ..default()
            },
            sprite: Sprite {
                color: PADDLE_COLOR,
                custom_size: Some(PADDLE_SIZE),
                ..default()
            },
            ..default()
        },
        Paddle
    ));

    // ball
    let ball_tex = asset_server.load("textures/ball.png");
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: BALL_STARTING_POSITION,
                ..default()
            },
            sprite: Sprite {
                color: BALL_COLOR,
                custom_size: Some(BALL_SIZE),
                ..default()
            },
            texture: ball_tex,
            ..default()
        },
        Ball,
        Velocity(BALL_SPEED * BALL_INITIAL_DIRECTION),
    ));
}

fn move_paddle(input: Res<ButtonInput<KeyCode>>,
               time_step: Res<Time<Fixed>>,
               mut query: Query<&mut Transform, With<Paddle>>, ) {
    let mut paddle_transform = query.single_mut();

    let mut direction = 0.0;
    if input.pressed(KeyCode::KeyA) {
        direction -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) {
        direction += 1.0;
    }

    let new_x = paddle_transform.translation.x + direction * PADDLE_SPEED * time_step.delta_seconds();

    paddle_transform.translation.x = new_x;
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time_step: Res<Time<Fixed>>) {
    let delta_time = time_step.delta_seconds();
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * delta_time;
        transform.translation.y += velocity.y * delta_time;
    }
}