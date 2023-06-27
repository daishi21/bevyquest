use crate::{player::spawn_player, prelude::*};

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                two_frame_animation,
                animate_sprite,
                change_player_animation,
                //change_slime_animation,
            )
                .after(spawn_player)
                .in_set(OnUpdate(GameState::GamePlay)),
        );
    }
}

pub fn two_frame_animation(
    mut animated: Query<(&mut TwoFrameAnimation, &mut Handle<Image>)>,
    time: Res<Time>,
) {
    for (mut animation, mut image) in &mut animated {
        animation.timer.tick(time.delta());
        if animation.current_frame {
            *image = animation.frame_2.clone();
        } else {
            *image = animation.frame_1.clone();
        }

        if animation.timer.just_finished() {
            if animation.current_frame {
                animation.current_frame = false;
            } else {
                animation.current_frame = true;
            }
        }
    }
}
pub fn animate_sprite(
    mut animations: Query<(&mut TextureAtlasSprite, &SpriteAnimation, &mut FrameTime)>,
    time: Res<Time>,
) {
    for (mut sprite, animation, mut frame_time) in animations.iter_mut() {
        frame_time.0 += time.delta_seconds();
        if frame_time.0 > animation.frame_time {
            let frames = (frame_time.0 / animation.frame_time) as usize;
            sprite.index += frames;
            if sprite.index >= animation.len {
                sprite.index %= animation.len;
            }
            frame_time.0 -= animation.frame_time;
        }
    }
}

pub fn change_player_animation(
    mut player: Query<
        (
            &mut Handle<TextureAtlas>,
            &mut SpriteAnimation,
            &mut TextureAtlasSprite,
        ),
        With<Player>,
    >,
    player_face: Query<&Player>,
    input: Res<Input<KeyCode>>,
    animaitons: Res<Animations>,
) {
    let (mut atlas, mut animation, mut sprite) = player.single_mut();
    let check_face = player_face.single();

    let set = if check_face.facing == Facing::Right && check_face.state == PlayerState::Moving {
        Animation::PlayerRight
    } else if check_face.facing == Facing::Up && check_face.state == PlayerState::Moving {
        Animation::PlayerUp
    } else if check_face.facing == Facing::Left && check_face.state == PlayerState::Moving {
        Animation::PlayerLeft
    } else if check_face.facing == Facing::Down && check_face.state == PlayerState::Moving {
        Animation::PlayerDown
    } else if check_face.state == PlayerState::Idle {
        Animation::PlayerIdle
    } else {
        Animation::PlayerIdle
    };

    let Some((new_atlas, new_animaiton)) = animaitons.get(set) else {error!("No Animation Jump Loaded"); return;};
    *atlas = new_atlas;
    sprite.index %= new_animaiton.len;
    *animation = new_animaiton;
}

/*

pub fn change_slime_animation(
    mut enemy: Query<
        (
            &mut Handle<TextureAtlas>,
            &mut SpriteAnimation,
            &mut TextureAtlasSprite,
        ),
        With<Enemy>,
    >,
    enemy_face: Query<&Enemy>,
    animaitons: Res<Animations>,
) {
    let (mut atlas, mut animation, mut sprite) = enemy.single_mut();
    let check_face = enemy_face.single();

    let set = if check_face.facing == Facing::Right {
        Animation::SlimeRight
    } else if check_face.facing == Facing::Up {
        Animation::SlimeUp
    } else if check_face.facing == Facing::Left {
        Animation::SlimeLeft
    } else if check_face.facing == Facing::Down {
        Animation::SlimeDown
    } else {
        Animation::SlimeIdle
    };

    let Some((new_atlas, new_animaiton)) = animaitons.get(set) else {error!("No Animation Jump Loaded"); return;};
    *atlas = new_atlas;
    sprite.index %= new_animaiton.len;
    *animation = new_animaiton;
}
*/
