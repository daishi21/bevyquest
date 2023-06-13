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
    pub use bevy_rapier2d::prelude::*;
    pub use bevy_turborand::prelude::*;
    pub use rand::prelude::*;

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

    #[derive(Resource)]
    pub struct EnemyCount {
        pub value: u32,
    }

    impl Default for EnemyCount {
        fn default() -> EnemyCount {
            EnemyCount { value: 0 }
        }
    }
}
