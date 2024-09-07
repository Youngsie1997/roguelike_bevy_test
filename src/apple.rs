use bevy::{ecs::define_label, prelude::*, transform::commands};
use rand::prelude::*;
pub struct ApplePlugin;

impl Plugin for ApplePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (tick_respawn_timer, spawn_power_up));
        app.add_systems(Startup, spawn_timer);
    }
}

#[derive(Component)]
pub struct Apple {
    restore_amount: f32,
}

#[derive(Component)]
struct PowerUpTimer {
    respawn_time: Timer,
}

fn tick_respawn_timer(time: Res<Time>, mut query: Query<&mut PowerUpTimer>) {
    let mut powerup_timer = query.single_mut();
    powerup_timer.respawn_time.tick(time.delta());
}

fn spawn_timer(mut commands: Commands) {
    commands.spawn(PowerUpTimer {
        respawn_time: Timer::from_seconds(25.0, TimerMode::Once),
    });
}

fn spawn_power_up(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<&mut PowerUpTimer>,
) {
    let mut powerup_timer = query.single_mut();
    if powerup_timer.respawn_time.finished() {
        let texture_handle = asset_server.load("apple.png");
        let mut rng = rand::thread_rng();
        let random_x = rng.gen_range(0.0..200.0);
        let random_y = rng.gen_range(0.0..200.0);
        commands.spawn((
            SpriteBundle {
                texture: texture_handle,
                transform: Transform {
                    translation: Vec3::new(random_x, random_y, 0.0),
                    ..default()
                },
                sprite: Sprite {
                    custom_size: Some(Vec2::new(100.0, 100.0)),
                    ..default()
                },
                ..default()
            },
            Apple {
                restore_amount: 20.0,
            },
            Name::new("Apple"),
        ));
        powerup_timer.respawn_time.reset();
    }
}
