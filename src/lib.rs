mod animation;
mod attacks;
mod camera;
mod enemy;
mod player;
mod potions;
mod ui;
pub mod prelude {

    pub use crate::animation::AnimationPlugin;
    pub use crate::attacks::AttackPlugin;
    pub use crate::camera::GameCameraPlugin;
    pub use crate::enemy::EnemyPlugin;
    pub use crate::player::PlayerPlugin;
    pub use crate::potions::PotionsPlugin;
    pub use crate::ui::GameUiPlugin;

    pub use bevy::prelude::*;
    pub use bevy::time::Stopwatch;
    pub use bevy::window::PrimaryWindow;
    pub use bevy_editor_pls::prelude::*;
    pub use bevy_rapier2d::prelude::*;
    pub use bevy_turborand::prelude::*;
    pub use rand::prelude::*;
    pub use std::collections::HashMap;

    pub const PLAYER_SIZE: f32 = 64.0;
    pub const NUMBER_OF_ENEMIES: usize = 1;
    pub const ENEMY_SIZE: f32 = 64.0;
    pub const NUMBER_OF_POTIONS: usize = 30;
    pub const POTION_SIZE: f32 = 20.0;
    pub const SPAWN_TIME: f32 = 1.0;
    pub const WIDTH: f32 = 948.0;
    pub const HEIGHT: f32 = 533.0;
    pub const RENDER_WIDTH: f32 = 960.;
    pub const RENDER_HEIGHT: f32 = 540.;
    pub const PIXEL_TO_WORLD: f32 = 30. / 960.;

    #[derive(States, PartialEq, Eq, Default, Debug, Clone, Hash)]
    pub enum GameState {
        #[default]
        MainMenu,
        GamePlay,
        GameOver,
    }

    #[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
    pub enum SimulationState {
        #[default]
        Running,
        Paused,
    }

    #[derive(Component)]
    pub struct Player {
        pub speed: f32,
        pub health: f32,
        pub max_health: f32,
        pub damage: f32,
        pub facing: Facing,
    }

    //  #[derive(Component, Clone)]
    pub enum Facing {
        Left,
        Right,
    }

    #[derive(Component, Clone)]
    pub struct Enemy {
        pub speed: f32,
        pub health: f32,
        pub damage: f32,
        pub asset: String,
        //       pub facing: Facing,
    }

    #[derive(Component)]
    pub struct Blade {
        pub timer: Timer,
        pub damage: f32,
    }

    #[derive(Component, Clone)]
    pub struct Potion {
        pub health: f32,
        pub asset: String,
    }

    #[derive(Component)]
    pub struct TwoFrameAnimation {
        pub frame_1: Handle<Image>,
        pub frame_2: Handle<Image>,
        pub current_frame: bool,
        pub timer: Timer,
    }

    #[derive(Resource)]
    pub struct AboutShown(pub bool);

    #[derive(Resource)]
    pub struct MainRender(pub Handle<Image>);

    #[derive(Component)]
    pub struct HeaderBarUI;

    #[derive(Component)]
    pub struct PlayerUI;

    #[derive(Component)]
    pub struct HealthUI;

    #[derive(Component)]
    pub struct MainMenuUI;

    #[derive(Component)]
    pub struct GameOverUI;

    #[derive(Component)]
    pub struct PauseUI;

    #[derive(Component)]
    pub struct StartButtonUI;

    #[derive(Component)]
    pub struct AboutButtonUI;

    #[derive(Component)]
    pub struct GamePlayEntity;

    #[derive(Component)]
    pub struct GameOverButtonUI;

    #[derive(Component)]
    pub struct AboutUI;

    #[derive(Component)]
    pub struct AboutBackButton;

    #[derive(Component)]
    pub struct MainCamera;

    #[derive(Component)]
    pub struct FinalCamera;

    #[derive(Component)]
    pub struct WorldTextUI {
        pub lifetime: Timer,
        pub velocity: Vec2,
        pub position: Vec2,
    }

    pub const POTION_SPAWN_TIME: f32 = 1.0;

    #[derive(Resource)]
    pub struct PotionSpawnTimer {
        pub timer: Timer,
    }

    impl Default for PotionSpawnTimer {
        fn default() -> PotionSpawnTimer {
            PotionSpawnTimer {
                timer: Timer::from_seconds(POTION_SPAWN_TIME, TimerMode::Repeating),
            }
        }
    }

    #[derive(Resource)]
    pub struct SpawnManager {
        pub global_time: Stopwatch,
        pub waves: Vec<Respawn>,
    }

    pub struct Respawn {
        pub next_spawn: Timer,
        pub respawn_size: i32,
        pub to_spawn: Enemy,
    }

    #[derive(Resource)]
    pub struct PotionManager {
        pub potion_time: Stopwatch,
        pub potion_waves: Vec<RePotion>,
    }

    pub struct RePotion {
        pub next_potion_spawn: Timer,
        pub potion_count: i32,
        pub to_spawn_potion: Potion,
    }

    #[derive(Resource, Default)]
    pub struct EnemyCount {
        pub value: u32,
    }

    #[derive(Debug, Hash, PartialEq, Eq)]
    pub enum Animation {
        PlayerIdle,
        PlayerUp,
        PlayerDown,
        PlayerLeft,
        PlayerRight,
    }

    #[derive(Component)]
    pub struct FrameTime(pub f32);

    #[derive(Component, Clone, Copy)]
    pub struct SpriteAnimation {
        pub len: usize,
        pub frame_time: f32,
    }

    #[derive(Resource)]
    pub struct Animations {
        pub map: HashMap<Animation, (Handle<TextureAtlas>, SpriteAnimation)>,
    }

    impl Animations {
        pub fn add(
            &mut self,
            id: Animation,
            handle: Handle<TextureAtlas>,
            animation: SpriteAnimation,
        ) {
            self.map.insert(id, (handle, animation));
        }
        pub fn get(&self, id: Animation) -> Option<(Handle<TextureAtlas>, SpriteAnimation)> {
            self.map.get(&id).cloned()
        }
    }

    impl FromWorld for Animations {
        fn from_world(world: &mut World) -> Self {
            let mut map = Animations {
                map: HashMap::new(),
            };
            world.resource_scope(|world, mut texture_atles: Mut<Assets<TextureAtlas>>| {
                let asset_server = world.resource::<AssetServer>();
                let idel_atlas = TextureAtlas::from_grid(
                    asset_server.load("sprites/succubus_player.png"),
                    Vec2::new(69., 80.),
                    1,
                    1,
                    Some(Vec2::splat(2.0)),
                    None,
                );
                map.add(
                    Animation::PlayerIdle,
                    texture_atles.add(idel_atlas),
                    SpriteAnimation {
                        len: 1,
                        frame_time: 1. / 5.,
                    },
                );

                let left_atlas = TextureAtlas::from_grid(
                    asset_server.load("sprites/succubus_left.png"),
                    Vec2::new(69., 80.),
                    3,
                    1,
                    Some(Vec2::splat(2.0)),
                    None,
                );
                map.add(
                    Animation::PlayerLeft,
                    texture_atles.add(left_atlas),
                    SpriteAnimation {
                        len: 3,
                        frame_time: 1. / 10.,
                    },
                );

                let right_atlas = TextureAtlas::from_grid(
                    asset_server.load("sprites/succubus_right.png"),
                    Vec2::new(69., 80.),
                    3,
                    1,
                    Some(Vec2::splat(2.0)),
                    None,
                );
                map.add(
                    Animation::PlayerRight,
                    texture_atles.add(right_atlas),
                    SpriteAnimation {
                        len: 3,
                        frame_time: 1. / 10.,
                    },
                );

                let up_atlas = TextureAtlas::from_grid(
                    asset_server.load("sprites/succubus_up.png"),
                    Vec2::new(69., 80.),
                    3,
                    1,
                    Some(Vec2::splat(2.0)),
                    None,
                );
                map.add(
                    Animation::PlayerUp,
                    texture_atles.add(up_atlas),
                    SpriteAnimation {
                        len: 3,
                        frame_time: 1. / 10.,
                    },
                );
                let down_atlas = TextureAtlas::from_grid(
                    asset_server.load("sprites/succubus_down.png"),
                    Vec2::new(69., 80.),
                    3,
                    1,
                    Some(Vec2::splat(2.0)),
                    None,
                );
                map.add(
                    Animation::PlayerDown,
                    texture_atles.add(down_atlas),
                    SpriteAnimation {
                        len: 3,
                        frame_time: 1. / 10.,
                    },
                );
            });

            map
        }
    }
}
