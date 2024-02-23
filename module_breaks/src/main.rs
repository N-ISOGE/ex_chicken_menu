use bevy::math;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, move_paddle)
        .run();
}

// 길이 mm, 시간 s
const PADDLE_START_Y: f32 = 0.0;
const PADDLE_SIZE: Vec2 = Vec2::new(120.0, 20.0);
const PADDLE_COLOR: Color = Color::rgb(0.3, 0.2, 0.5);
const PADDLE_SPEED: f32 = 700.0;

#[derive(Component)]
struct Paddle;

fn setup(mut commands: Commands) {
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