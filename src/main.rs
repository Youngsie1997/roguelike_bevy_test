use bevy::{
    ecs::{query, reflect},
    input::common_conditions::input_toggle_active,
    math::NormedVectorSpace,
    prelude::*,
};
use bevy_inspector_egui::{quick::WorldInspectorPlugin, InspectorOptions};

use crate::{
    apple::{Apple, ApplePlugin},
    audio::GameAudioPlugin,
    cow::CowPlugin,
    pig::{Pig, PigPlugin},
};
mod apple;
mod audio;
mod cow;
mod pig;
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Basic rogue like game".into(),
                        resolution: (1920.0, 1080.0).into(),
                        resizable: false,

                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        .add_plugins((PigPlugin, CowPlugin))
        .add_plugins(ApplePlugin)
        .add_plugins(GameAudioPlugin)
        .add_systems(Startup, (spawn_camera, spawn_player))
        .add_systems(Update, (player_movement, damage_timer_tick))
        .insert_resource(Money(100.0))
        .register_type::<Player>()
        .register_type::<Money>()
        .register_type::<Pig>()
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn damage_timer_tick(time: Res<Time>, mut player_query: Query<&mut Player>) {
    let mut player = player_query.single_mut();
    player.damage_timer.tick(time.delta());
}

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component)]
struct Player {
    #[inspector(min = 0.0, max = 1000.0)]
    speed: f32,
    health: f32,
    damage_timer: Timer,
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct Money(pub f32);

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle = asset_server.load("character.png");
    commands.spawn((
        Player {
            speed: 500.0,
            health: 100.0,
            damage_timer: Timer::from_seconds(1.0, TimerMode::Once),
        },
        SpriteBundle {
            texture: texture_handle,
            sprite: Sprite {
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..default()
            },
            ..default()
        },
        Name::new("Player"),
    ));
}

fn player_movement(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Player), Without<Apple>>,
    time: Res<Time>,
    apple_query: Query<(Entity, &mut Transform), With<Apple>>,
) {
    for (mut player_transform, mut player) in &mut query {
        let delta = time.delta_seconds();
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }
        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
        }
        player_transform.translation += direction * player.speed * delta;

        for (apple_entity, apple_transform) in &apple_query {
            if apple_transform.translation.x.round() == player_transform.translation.x.round()
                || apple_transform.translation.y.round() == player_transform.translation.y.round()
            {
                player.health += 20.0;
                info!("You ate an apple yum. health is now {:?}", player.health);
                commands.entity(apple_entity).despawn();
            }
        }
    }
}
