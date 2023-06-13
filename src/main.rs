#![allow(clippy::type_complexity)]
//use bevy::app::AppExit;
use bevy::input::common_conditions::input_toggle_active;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevyquest::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "BevyQuest".into(),
                        resolution: (WIDTH, HEIGHT).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugin(WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::I)))
        .insert_resource(SpawnManager {
            global_time: Stopwatch::new(),
            waves: vec![
                Respawn {
                    next_spawn: Timer::from_seconds(1.0, TimerMode::Repeating),
                    respawn_size: 2,
                    to_spawn: Enemy {
                        speed: 1.3,
                        health: 10.0,
                        asset: "sprites/Kobold.png".to_string(),
                        damage: 1.0,
                    },
                },
                Respawn {
                    next_spawn: Timer::from_seconds(5.0, TimerMode::Repeating),
                    respawn_size: 2,
                    to_spawn: Enemy {
                        speed: 2.2,
                        health: 5.0,
                        asset: "sprites/Skeleton.png".to_string(),
                        damage: 2.0,
                    },
                },
                Respawn {
                    next_spawn: Timer::from_seconds(10.0, TimerMode::Repeating),
                    respawn_size: 2,
                    to_spawn: Enemy {
                        speed: 0.8,
                        health: 30.0,
                        asset: "sprites/Orc.png".to_string(),
                        damage: 10.0,
                    },
                },
                Respawn {
                    next_spawn: Timer::from_seconds(15.0, TimerMode::Repeating),
                    respawn_size: 2,
                    to_spawn: Enemy {
                        speed: 2.5,
                        health: 15.0,
                        asset: "sprites/Naga.png".to_string(),
                        damage: 10.0,
                    },
                },
            ],
        })
        .insert_resource(PotionManager {
            potion_time: Stopwatch::new(),
            potion_waves: vec![RePotion {
                next_potion_spawn: Timer::from_seconds(5.0, TimerMode::Repeating),
                potion_count: 1,
                to_spawn_potion: Potion {
                    health: 1.0,
                    asset: "sprites/health_potion_small.png".to_string(),
                },
            }],
        })
        .add_plugin(RngPlugin::default())
        .add_state::<GameState>()
        .add_state::<SimulationState>()
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(GameCameraPlugin)
        .add_plugin(GameUiPlugin)
        .add_plugin(PotionsPlugin)
        .add_plugin(AttackPlugin)
        .add_plugin(AnimationPlugin)
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(50.0))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        .add_startup_system(spawn_background)
        .add_startup_system(start_music)
        .add_system(exit_game.in_set(OnUpdate(GameState::GamePlay)))
        .add_system(despawn_game_play.in_schedule(OnEnter(GameState::GameOver)))
        .run();
}

fn start_music(audio: Res<Audio>, assets: Res<AssetServer>) {
    audio.play_with_settings(
        assets.load("sounds/nightmare-on-imaginationland-8040.ogg"),
        PlaybackSettings {
            repeat: true,
            volume: 0.5,
            speed: 1.0,
        },
    );
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        game_state.set(GameState::GameOver);
    }
}

fn spawn_background(mut commands: Commands, assets: Res<AssetServer>) {
    let size = 1080.0 * PIXEL_TO_WORLD;
    for i in -7..7 {
        for j in -7..7 {
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(i as f32 * size, j as f32 * size, 1.0),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(size, size)),
                        ..default()
                    },
                    texture: assets.load("map.png"),
                    ..default()
                },
                Name::new("Background"),
            ));
        }
    }
}

fn despawn_game_play(mut commands: Commands, entities: Query<Entity, With<GamePlayEntity>>) {
    for entity in &entities {
        commands.entity(entity).despawn_recursive();
    }
}
