use crate::{player::spawn_player, prelude::*};

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (two_frame_animation, animate_sprite, change_player_animation)
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
    input: Res<Input<KeyCode>>,
    animaitons: Res<Animations>,
) {
    let (mut atlas, mut animation, mut sprite) = player.single_mut();

    let set = if input.any_pressed([KeyCode::D, KeyCode::Right]) {
        Animation::PlayerRight
    } else if input.any_pressed([KeyCode::W, KeyCode::Up]) {
        Animation::PlayerUp
    } else if input.any_pressed([KeyCode::A, KeyCode::Left]) {
        Animation::PlayerLeft
    } else if input.any_pressed([KeyCode::S, KeyCode::Down]) {
        Animation::PlayerDown
    } else {
        Animation::PlayerIdle
    };

    let Some((new_atlas, new_animaiton)) = animaitons.get(set) else {error!("No Animation Jump Loaded"); return;};
    *atlas = new_atlas;
    sprite.index %= new_animaiton.len;
    *animation = new_animaiton;
}
