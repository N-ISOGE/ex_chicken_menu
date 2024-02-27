use bevy::{math, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                move_paddle,
                apply_velocity,
                check_ball_collisions.after(apply_velocity),
            ),
        )
        .run();
}

// 길이 px인가 mm인가, 시간 s

// PADDLE
const PADDLE_START_Y: f32 = 0.0;
const PADDLE_SIZE: Vec3 = Vec3::new(120.0, 20., 0.);
const PADDLE_COLOR: Color = Color::rgb(0.3, 0.2, 0.5);
const PADDLE_SPEED: f32 = 700.0;

// BALL
const BALL_DIAMETER: f32 = 20.;
const BALL_SIZE: Vec2 = Vec2::splat(BALL_DIAMETER);
const BALL_COLOR: Color = Color::rgb(0.9, 0.1, 0.2);
const BALL_SPEED: f32 = 500.0;
const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.);
const BALL_INITIAL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);

// WALL
const WALL_LEFT: f32 = -450.;
const WALL_RIGHT: f32 = 450.;
const WALL_TOP: f32 = 300.;
const WALL_BOTTOM: f32 = -300.;

const WALL_THICKNESS: f32 = 10.;
const WALL_BLOCK_WIDTH: f32 = WALL_RIGHT - WALL_LEFT;
const WALL_BLOCK_HEIGHT: f32 = WALL_TOP - WALL_BOTTOM;
const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Ball;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider;

#[derive(Bundle)]
struct WallBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // camera
    commands.spawn(Camera2dBundle::default());

    // paddle
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: math::vec3(0., PADDLE_START_Y, 0.),
                scale: PADDLE_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: PADDLE_COLOR,
                ..default()
            },
            ..default()
        },
        Paddle,
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
        Ball {
            size: Circle::new(BALL_DIAMETER / 2.),
        },
        Velocity(BALL_SPEED * BALL_INITIAL_DIRECTION),
    ));

    // walls
    {
        let vertical_wall_size = math::vec2(WALL_THICKNESS, WALL_BLOCK_HEIGHT + WALL_THICKNESS);
        let horizontal_wall_size = math::vec2(WALL_BLOCK_WIDTH + WALL_THICKNESS, WALL_THICKNESS);
        // left wall
        commands.spawn(WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: math::vec3(WALL_LEFT, 0., 0.),
                    scale: vertical_wall_size.extend(0.),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        });

        // right wall
        commands.spawn(WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: math::vec3(WALL_RIGHT, 0., 0.),
                    scale: vertical_wall_size.extend(0.),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        });

        // bottom wall
        commands.spawn(WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: math::vec3(0., WALL_BOTTOM, 0.),
                    scale: horizontal_wall_size.extend(0.),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        });

        // top wall
        commands.spawn(WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: math::vec3(0., WALL_TOP, 0.),
                    scale: horizontal_wall_size.extend(0.),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        });
    }
}

fn move_paddle(
    input: Res<ButtonInput<KeyCode>>,
    time_step: Res<Time<Fixed>>,
    mut query: Query<&mut Transform, With<Paddle>>,
) {
    let mut paddle_transform = query.single_mut();

    let mut direction = 0.0;
    if input.pressed(KeyCode::KeyA) {
        direction -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) {
        direction += 1.0;
    }
    let new_x =
        paddle_transform.translation.x + direction * PADDLE_SPEED * time_step.delta_seconds();

    let new_x = new_x.min(WALL_RIGHT - (WALL_THICKNESS + PADDLE_SIZE.x) * 0.5);
    let new_x = new_x.max(WALL_LEFT + (WALL_THICKNESS + PADDLE_SIZE.x) * 0.5);

    paddle_transform.translation.x = new_x;
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time_step: Res<Time<Fixed>>) {
    let delta_time = time_step.delta_seconds();
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * delta_time;
        transform.translation.y += velocity.y * delta_time;
    }
}

fn check_ball_collisions(
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    collider_query: Query<(&Transform, &Collider)>,
) {
    let (mut ball_velocity, ball_transform) = ball_query.single_mut();

    for (transform, _) in &collider_query {
        use math::bounding::*;

        let ball_bound =
            BoundingCircle::new(ball_transform.translation.truncate(), BALL_DIAMETER / 2.);
        let other_bound = Aabb2d::new(
            transform.translation.truncate(),
            transform.scale.truncate() / 2.,
        );
        let collision = ball_bound.intersects(&other_bound);

        if collision {
            let closest = other_bound.closest_point(ball_bound.center());
            let offset = closest - ball_bound.center();

            if offset.x.abs() > offset.y.abs() {
                // x 축과 가까운 곳에서 충돌
                ball_velocity.x = -ball_velocity.x;
            } else {
                // y 축과 가까운 곳에서 충돌
                ball_velocity.y = -ball_velocity.y;
            }
        }
    }
}
