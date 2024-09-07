use crate::{audio::PlaySFXEvent, Money, Player};
use bevy::{ecs::query, prelude::*, render::extract_resource::ExtractResource};
use bevy_kira_audio::{Audio, AudioControl};
use rand::prelude::*;

pub struct PigPlugin;

impl Plugin for PigPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_pig, pig_lifetime));
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Pig {
    pub lifetime: Timer,
}

fn pig_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut pigs: Query<(&mut Transform, Entity, &mut Pig), Without<Player>>,
    mut money: ResMut<Money>,
    mut player_query: Query<(&Transform, &mut Player), With<Player>>,
) {
    for (mut transform, pig_entity, mut pig) in &mut pigs {
        pig.lifetime.tick(time.delta());

        let mut rng = rand::thread_rng();
        let random_direction = rng.gen_range(0..=4);
        let mut direction = Vec3::ZERO;
        // match random_direction {
        //     0 => direction.y += 1.0,
        //     1 => direction.y -= 1.0,
        //     2 => direction.x += 1.0,
        //     4 => direction.x -= 1.0,
        //     _ => return,
        // }
        //
        for (player_transform, mut player) in &mut player_query {
            if transform.translation.x > player_transform.translation.x {
                direction.x -= 1.0;
            }
            if transform.translation.x < player_transform.translation.x {
                direction.x += 1.0;
            }

            if transform.translation.y > player_transform.translation.y {
                direction.y -= 1.0;
            }

            if transform.translation.y < player_transform.translation.y {
                direction.y += 1.0;
            }
            transform.translation += direction * 100.0 * time.delta_seconds();

            if transform.translation.x.round() == player_transform.translation.x.round()
                && transform.translation.y.round() == player_transform.translation.y.round()
                && player.damage_timer.finished()
            {
                player.health -= 10.0;
                info!("Ouch you got bit, Player health is now {:?}", player.health);
                player.damage_timer.reset();
            }
        }

        if pig.lifetime.finished() {
            money.0 += 15.0;
            commands.entity(pig_entity).despawn();
            info!("Pig sold for £15 Current Money: ${:?}", money.0);
        }
    }
}

fn spawn_pig(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<ButtonInput<KeyCode>>,
    mut money: ResMut<Money>,
    mut ev_oink: EventWriter<PlaySFXEvent>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }
    let mut rng = rand::thread_rng();
    let random_x: f32 = rng.gen_range(0.0..640.0);
    let random_y: f32 = rng.gen_range(0.0..320.0);
    if money.0 >= 10.0 {
        money.0 -= 10.0;
        info!("Spent £10 on a pig, remaining money: £{:?}", money.0);
        let texture_handle = asset_server.load("pig.png");

        commands.spawn((
            SpriteBundle {
                texture: texture_handle,
                sprite: Sprite {
                    custom_size: Some(Vec2::new(100.0, 100.0)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(random_x, random_y, 0.0),
                    ..default()
                },
                ..default()
            },
            Pig {
                lifetime: Timer::from_seconds(5.0, TimerMode::Once),
            },
            Name::new("Pig"),
        ));
        ev_oink.send(PlaySFXEvent {
            sound_effect: asset_server.load("oink.ogg"),
        });
    }
}
