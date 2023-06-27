use crate::prelude::*;
use crate::ui::spawn_world_text;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                enemy_death_check,
                enemy_movement,
                spawn_enemy,
                despawn_enemy,
                enemy_damage_player.after(enemy_movement),
            )
                .in_set(OnUpdate(GameState::GamePlay)),
        );
    }
}

fn despawn_enemy(
    mut commands: Commands,
    player: Query<&Transform, (With<Player>, Without<Enemy>)>,
    enemy: Query<(Entity, &Transform), With<Enemy>>,
) {
    let player = player.single();

    for (enemy, transform) in &enemy {
        if Vec2::distance(
            player.translation.truncate(),
            transform.translation.truncate(),
        ) > 30.0
        {
            commands.entity(enemy).despawn_recursive();
        }
    }
}

fn spawn_enemy(
    mut commands: Commands,
    mut spawn_manager: ResMut<SpawnManager>,
    player: Query<&Transform, With<Player>>,
    assets: Res<AssetServer>,
    mut global_rng: ResMut<GlobalRng>,
    time: Res<Time>,
) {
    let player_transform = player.single();

    spawn_manager.global_time.tick(time.delta());

    let current_wave = (spawn_manager.global_time.elapsed_secs() / 20.0) as usize;
    let wave_index = current_wave % spawn_manager.waves.len();
    let wave_buf = current_wave / spawn_manager.waves.len();

    let wave = &mut spawn_manager.waves[wave_index];
    let size = (wave.respawn_size as f32 * 1.3_f32.powf(wave_buf as f32)) as i32;

    wave.next_spawn.tick(time.delta());

    if wave.next_spawn.just_finished() {
        for _i in 0..size {
            let target_direction = 22.0
                * Vec2::new(global_rng.f32_normalized(), global_rng.f32_normalized()).normalize();

            let mut target_translation = target_direction.extend(100.0)
                + Vec3::new(
                    global_rng.f32_normalized(),
                    global_rng.f32_normalized(),
                    0.0,
                );

            let mut enemy = wave.to_spawn.clone();
            enemy.speed *= 1.3_f32.powf(wave_buf as f32);
            enemy.health *= 1.3_f32.powf(wave_buf as f32);

            target_translation += player_transform.translation.truncate().extend(0.0);
            commands.spawn((
                SpriteBundle {
                    texture: assets.load(&wave.to_spawn.asset),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(69.0 * PIXEL_TO_WORLD, 80.0 * PIXEL_TO_WORLD)),
                        ..default()
                    },
                    transform: Transform::from_translation(target_translation),
                    ..default()
                },
                enemy,
                Name::new("Enemy"),
                RngComponent::from(&mut global_rng),
                RigidBody::Dynamic,
                LockedAxes::ROTATION_LOCKED_Z,
                Damping {
                    linear_damping: 100.0,
                    angular_damping: 1.0,
                },
                GamePlayEntity,
                Collider::capsule(Vec2::new(0.0, 0.55), Vec2::new(0.0, -0.55), 0.8),
            ));
        }
    }
}

fn enemy_movement(
    player: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut enemy: Query<(&mut Transform, &mut Sprite, &Enemy)>,
    time: Res<Time>,
) {
    let player_transform = player.single();

    for (mut transform, mut sprite, enemy) in &mut enemy {
        let direction = (transform.translation.truncate()
            - player_transform.translation.truncate())
        .normalize();
        sprite.flip_x = direction.x > 0.0;
        transform.translation -= (direction * time.delta_seconds() * enemy.speed).extend(0.);
    }
}

fn damage_player(
    commands: &mut Commands,
    assets: &AssetServer,
    enemy: &mut Enemy,
    position: &Transform,
    damage: f32,
) {
    spawn_world_text(
        commands,
        assets,
        position.translation.truncate(),
        &format!("{:?}", damage as i32),
    );

    enemy.health -= damage;
}

fn enemy_damage_player(
    enemies: Query<(&Collider, &GlobalTransform, &Enemy)>,
    mut player: Query<&mut Player>,
    rapier_context: Res<RapierContext>,
    time: Res<Time>,
    _assets: Res<AssetServer>,
    _audio: Res<Audio>,
) {
    for (collider, transform, enemy) in &enemies {
        rapier_context.intersections_with_shape(
            transform.translation().truncate(),
            0.0,
            collider,
            QueryFilter::new(),
            |entity| {
                if let Ok(mut player) = player.get_mut(entity) {
                    player.health -= enemy.damage * time.delta_seconds();
                }
                true
            },
        );
    }
}

fn enemy_death_check(mut commands: Commands, mut enemies: Query<(Entity, &Transform, &Enemy)>) {
    for (entity, _transform, enemy) in &mut enemies {
        if enemy.health <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
