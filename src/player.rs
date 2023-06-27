use crate::{attacks::spawn_blade, prelude::*};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_player.in_schedule(OnEnter(GameState::GamePlay)))
            .add_systems(
                (
                    player_movement.after(spawn_player),
                    player_game_over,
                    player_drink_potion.after(player_movement),
                )
                    .in_set(OnUpdate(GameState::GamePlay)),
            );
    }
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    animaitons: Res<Animations>,
) {
    let _window: &Window = window_query.get_single().unwrap();
    let blade = spawn_blade(&mut commands, &asset_server);

    let Some((texture_atlas, animation)) = animaitons.get(Animation::PlayerIdle) else {error!("Failed to find animation: Idle"); return;};
    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas,
                transform: Transform::from_xyz(0.0, 0.0, 2.0),
                sprite: TextureAtlasSprite {
                    index: 0,
                    custom_size: Some(Vec2::new(80.0 * PIXEL_TO_WORLD, 80.0 * PIXEL_TO_WORLD)),
                    ..Default::default()
                },
                ..Default::default()
            },
            Player {
                health: 100.0,
                max_health: 100.0,
                speed: 5.0,
                damage: 5.0,
                facing: Facing::Right,
                state: PlayerState::Idle,
            },
            Name::new("Player"),
            Collider::capsule(Vec2::new(0.0, 0.55), Vec2::new(0.0, -0.50), 0.5),
            GamePlayEntity,
            animation,
            FrameTime(0.0),
        ))
        .add_child(blade);
}

pub fn player_movement(
    mut player: Query<(&mut Transform, &mut Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, mut player) = player.single_mut();
    if input.pressed(KeyCode::W) {
        transform.translation.y += time.delta_seconds() * player.speed;
        player.facing = Facing::Up;
        player.state = PlayerState::Moving;
    } else if input.pressed(KeyCode::S) {
        transform.translation.y -= time.delta_seconds() * player.speed;
        player.facing = Facing::Down;
        player.state = PlayerState::Moving;
    } else if input.pressed(KeyCode::A) {
        transform.translation.x -= time.delta_seconds() * player.speed;
        player.facing = Facing::Left;
        player.state = PlayerState::Moving;
    } else if input.pressed(KeyCode::D) {
        transform.translation.x += time.delta_seconds() * player.speed;
        player.facing = Facing::Right;
        player.state = PlayerState::Moving;
    } else {
        player.state = PlayerState::Idle
    };
}

fn player_game_over(
    player: Query<&Player>,
    mut game_state: ResMut<NextState<GameState>>,
    audio: Res<Audio>,
    assets: Res<AssetServer>,
) {
    let player = player.single();

    if player.health <= 0.0 {
        audio.play_with_settings(
            assets.load("sounds/you_lose.ogg"),
            PlaybackSettings {
                repeat: false,
                volume: 0.9,
                speed: 1.0,
            },
        );
        game_state.set(GameState::GameOver);
    }
}

fn player_drink_potion(
    potion: Query<(&Collider, &GlobalTransform, &Potion)>,
    mut player: Query<&mut Player>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    rapier_context: Res<RapierContext>,
) {
    for (collider, transform, _potion) in &potion {
        rapier_context.intersections_with_shape(
            transform.translation().truncate(),
            0.0,
            collider,
            QueryFilter::new(),
            |entity| {
                if let Ok(mut player) = player.get_mut(entity) {
                    player.health += 10.0;

                    let sound_effect = asset_server.load("sounds/slime_000.ogg");
                    audio.play(sound_effect);
                }
                true
            },
        );
    }
}
