use crate::prelude::*;

pub struct PotionsPlugin;

impl Plugin for PotionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PotionSpawnTimer>().add_systems(
            (
                // tick_potion_spawn_timer,
                potion_death_check,
                // spawn_potions_over_time,
                spawn_potions,
            )
                .in_set(OnUpdate(GameState::GamePlay)),
        );
    }
}

pub fn spawn_potions(
    mut commands: Commands,
    player: Query<&Transform, With<Player>>,
    mut spawn_manager: ResMut<PotionManager>,
    mut global_rng: ResMut<GlobalRng>,
    time: Res<Time>,
    assets: Res<AssetServer>,
) {
    let player_transform = player.single();

    spawn_manager.potion_time.tick(time.delta());

    let current_wave = (spawn_manager.potion_time.elapsed_secs() / 20.0) as usize;
    let wave_index = current_wave % spawn_manager.potion_waves.len();
    let wave_buf = current_wave / spawn_manager.potion_waves.len();

    let wave = &mut spawn_manager.potion_waves[wave_index];
    let size = wave.potion_count;

    wave.next_potion_spawn.tick(time.delta());

    if wave.next_potion_spawn.just_finished() {
        for _i in 0..size {
            let target_direction = 22.0
                * Vec2::new(global_rng.f32_normalized(), global_rng.f32_normalized()).normalize();

            let mut target_translation = target_direction.extend(100.0)
                + Vec3::new(
                    global_rng.f32_normalized(),
                    global_rng.f32_normalized(),
                    0.0,
                );

            let mut potion = wave.to_spawn_potion.clone();
            potion.health *= 1.3_f32.powf(wave_buf as f32);

            target_translation += player_transform.translation.truncate().extend(0.0);
            commands.spawn((
                SpriteBundle {
                    texture: assets.load(&wave.to_spawn_potion.asset),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(32.0 * PIXEL_TO_WORLD, 32.0 * PIXEL_TO_WORLD)),
                        ..default()
                    },
                    transform: Transform::from_translation(target_translation),
                    ..default()
                },
                potion,
                Name::new("Potion"),
                RngComponent::from(&mut global_rng),
                RigidBody::Dynamic,
                LockedAxes::ROTATION_LOCKED_Z,
                Damping {
                    linear_damping: 100.0,
                    angular_damping: 1.0,
                },
                Collider::ball(0.9),
                GamePlayEntity,
            ));
        }
    }
}
/*
pub fn tick_potion_spawn_timer(mut potion_spawn_timer: ResMut<PotionSpawnTimer>, time: Res<Time>) {
    potion_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_potions_over_time(
    mut commands: Commands,
    player: Query<&Transform, With<Player>>,
    mut global_rng: ResMut<GlobalRng>,
    asset_server: Res<AssetServer>,
    potion_spawn_timer: Res<PotionSpawnTimer>,
) {
    if potion_spawn_timer.timer.finished() {
        let target_direction =
            22.0 * Vec2::new(global_rng.f32_normalized(), global_rng.f32_normalized()).normalize();

        let player_transform = player.single();

        let mut target_translation = target_direction.extend(100.0)
            + Vec3::new(
                global_rng.f32_normalized(),
                global_rng.f32_normalized(),
                0.0,
            );

        target_translation += player_transform.translation.truncate().extend(0.0);

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(32.0 * PIXEL_TO_WORLD, 32.0 * PIXEL_TO_WORLD)),
                    ..default()
                },
                transform: Transform::from_translation(target_translation),
                texture: asset_server.load("sprites/health_potion_small.png"),
                ..default()
            },
            Potion { health: 1.0 },
            Name::new("Potion"),
            Collider::ball(0.9),
            GamePlayEntity,
        ));
    }
}

*/

fn potion_death_check(mut commands: Commands, mut potions: Query<(Entity, &Transform, &Potion)>) {
    for (entity, _transform, potion) in &mut potions {
        if potion.health <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
