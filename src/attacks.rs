use crate::{prelude::*, ui::spawn_world_text};
use std::{time::Duration,};

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((player_attack, blade_attack, blade_attack_facing).in_set(OnUpdate(GameState::GamePlay)));
    }
}

fn damage_enemy(
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

fn damage_potion(
    _commands: &mut Commands,
    potion: &mut Potion,
    _position: &Transform,
    damage: f32,
) {
    potion.health -= damage;
}

fn player_attack(
    mut commands: Commands,
    mut potion: Query<(&mut Potion, &Transform)>,
    player: Query<(&Collider, &GlobalTransform, &Player)>,
    rapier_context: Res<RapierContext>,
) {
    for (collider, transform, player) in &player {
        rapier_context.intersections_with_shape(
            transform.translation().truncate(),
            0.0,
            collider,
            QueryFilter::new(),
            |entity| {
                /*

                if let Ok((mut enemy, transform)) = enemy.get_mut(entity) {
                    damage_enemy(&mut commands, &assets, &mut enemy, transform, player.damage);
                    let sound_effect = assets.load("sounds/knifeSlice.ogg");
                    audio.play(sound_effect);
                }

                */

                if let Ok((mut potion, transform)) = potion.get_mut(entity) {
                    damage_potion(&mut commands, &mut potion, transform, player.damage);
                }

                true
            },
        );
    }
}

pub fn spawn_blade(commands: &mut Commands, assets: &AssetServer) -> Entity {
    let mut timer = Timer::from_seconds(1.0, TimerMode::Repeating);
    timer.set_elapsed(Duration::from_secs(1));

    commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_xyz(2.0, 0.0, 0.0),
                texture: assets.load("sprites/GuttyKreum_1.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(40.0 * PIXEL_TO_WORLD, 40.0 * PIXEL_TO_WORLD)),
                    flip_x: true,
                    ..default()
                },
                ..default()
            },
            GamePlayEntity,
            Name::new("Blade"),
           Blade { timer, damage: 5.0 },
            TwoFrameAnimation {
                frame_1: assets.load("sprites/GuttyKreum_2.png"),
                frame_2: assets.load("sprites/GuttyKreum_1.png"),
                current_frame: false,
                timer: Timer::from_seconds(10000.0, TimerMode::Repeating),
            },
            Sensor,
            Collider::cuboid(41.0 * PIXEL_TO_WORLD / 2.0, 41.0 * PIXEL_TO_WORLD / 2.0),
        ))
        .id()
}

pub fn blade_attack_facing(
    mut blade_query: Query<(&mut Transform, &mut Sprite), With<Blade>>,
    player: Query<&Player>,
) {
    let player = player.single();

    if let Ok((mut blade, mut sprite,)) = blade_query.get_single_mut() {
        blade.translation = match player.facing {
            Facing::Left => {
                sprite.flip_x = false;
                Vec3::new(-2.0, 0.0, 0.0)
            }
            Facing::Right => {
                sprite.flip_x = true;
                Vec3::new(2.0, 0.0, 0.0)
            }
        };
    }
}

fn blade_attack(
    mut commands: Commands,
    assets: Res<AssetServer>,
    audio: Res<Audio>,
    mut blades: Query<(
        &Collider,
        &GlobalTransform,
        &mut Blade,
        &mut TwoFrameAnimation,
        &mut Visibility,
    )>,
    mut enemy: Query<(&mut Enemy, &Transform)>,
    rapier_context: Res<RapierContext>,
    time: Res<Time>,
) {
    for (collider, transform, mut blade, mut animation, mut visibility) in &mut blades {
        blade.timer.tick(time.delta());

        *visibility = if blade.timer.percent() < 0.2 || blade.timer.percent() > 0.9 {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };

        if blade.timer.percent() > 0.5 {
            animation.current_frame = true;
        }

        if blade.timer.just_finished() {
            animation.current_frame = false;
            rapier_context.intersections_with_shape(
                transform.translation().truncate(),
                0.0,
                collider,
                QueryFilter::new(),
                |entity| {
                    if let Ok((mut enemy, transform)) = enemy.get_mut(entity) {
                        damage_enemy(&mut commands, &assets, &mut enemy, transform, blade.damage);
                        let sound_effect = assets.load("sounds/knifeSlice.ogg");
                        audio.play(sound_effect);
                    }
                    true
                },
            );
        }
    }
}
