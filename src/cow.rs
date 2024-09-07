use bevy::{ecs::reflect, prelude::*};
use bevy_inspector_egui::{egui::TextureHandle, prelude::ReflectInspectorOptions};
use bevy_kira_audio::Audio;
use rand::{prelude, Rng};

use crate::Money;

pub struct CowPlugin;

impl Plugin for CowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_cow);
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct Cow {
    speed: f32,
}

fn spawn_cow(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut money: ResMut<Money>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let texture_handle = asset_server.load("cow.png");
    let mut rng = rand::thread_rng();
    let random_x: f32 = rng.gen_range(0.0..640.0);
    let random_y: f32 = rng.gen_range(0.0..320.0);
    if input.just_pressed(KeyCode::KeyC) && money.0 >= 200.0 {
        money.0 -= 50.0;
        commands.spawn((
            Cow { speed: 500.0 },
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
        ));
    }
}
