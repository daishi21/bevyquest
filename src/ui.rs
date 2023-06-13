use crate::prelude::*;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_player_ui.in_schedule(OnEnter(GameState::GamePlay)))
            .add_system(spawn_main_menu_ui.in_schedule(OnEnter(GameState::MainMenu)))
            .add_system(despawn_main_menu_ui.in_schedule(OnExit(GameState::MainMenu)))
            .add_system(spawn_game_over_ui.in_schedule(OnEnter(GameState::GameOver)))
            .add_system(despawn_game_over_ui.in_schedule(OnExit(GameState::GameOver)))
            .add_system(about_button_system)
            .add_system(about_back_button_system)
            .add_system(show_about_ui)
            .add_startup_system(spawn_about_ui)
            .add_system(start_button_system)
            .add_system(game_over_button_system)
            .add_system(update_world_text)
            .insert_resource(AboutShown(false))
            .add_system(player_health_ui_sync.in_set(OnUpdate(GameState::GamePlay)));
    }
}

fn start_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (With<Button>, With<StartButtonUI>),
    >,
    about: Res<AboutShown>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if about.0 {
        return;
    }
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = Color::RED.into();
                next_state.set(GameState::GamePlay);
            }
            Interaction::Hovered => {
                *color = Color::GREEN.into();
            }
            Interaction::None => {
                *color = Color::DARK_GREEN.into();
            }
        }
    }
}

fn about_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (With<Button>, With<AboutButtonUI>),
    >,
    mut about: ResMut<AboutShown>,
    _next_state: ResMut<NextState<GameState>>,
) {
    if about.0 {
        return;
    }
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = Color::RED.into();
                about.0 = true;
            }
            Interaction::Hovered => {
                *color = Color::GREEN.into();
            }
            Interaction::None => {
                *color = Color::DARK_GREEN.into();
            }
        }
    }
}

fn about_back_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (With<Button>, With<AboutBackButton>),
    >,
    mut about: ResMut<AboutShown>,
    _next_state: ResMut<NextState<GameState>>,
) {
    if !about.0 {
        return;
    }
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = Color::RED.into();
                about.0 = false;
            }
            Interaction::Hovered => {
                *color = Color::GREEN.into();
            }
            Interaction::None => {
                *color = Color::DARK_GREEN.into();
            }
        }
    }
}

fn show_about_ui(mut about: Query<&mut Visibility, With<AboutUI>>, about_shown: Res<AboutShown>) {
    for mut visible in &mut about {
        if about_shown.0 {
            *visible = Visibility::Visible;
        } else {
            *visible = Visibility::Hidden;
        }
    }
}

fn game_over_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (With<Button>, With<GameOverButtonUI>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = Color::RED.into();
                next_state.set(GameState::MainMenu);
            }
            Interaction::Hovered => {
                *color = Color::GREEN.into();
            }
            Interaction::None => {
                *color = Color::DARK_GREEN.into();
            }
        }
    }
}

fn despawn_main_menu_ui(mut commands: Commands, ui: Query<Entity, With<MainMenuUI>>) {
    for ui in &ui {
        commands.entity(ui).despawn_recursive();
    }
}

fn despawn_game_over_ui(mut commands: Commands, ui: Query<Entity, With<GameOverUI>>) {
    for ui in &ui {
        commands.entity(ui).despawn_recursive();
    }
}

fn spawn_main_menu_ui(mut commands: Commands, assets: Res<AssetServer>) {
    let font = assets.load("fonts/dos.ttf");

    let menu_parent = (
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(38.0), Val::Percent(32.0)),
                position: UiRect {
                    left: Val::Percent(47.0),
                    right: Val::Auto,
                    top: Val::Percent(45.0),
                    bottom: Val::Auto,
                },
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            },
            ..default()
        },
        MainMenuUI,
    );

    let button = (
        ButtonBundle {
            style: Style {
                size: Size::new(Val::Percent(70.0), Val::Percent(30.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                ..default()
            },

            background_color: Color::CRIMSON.into(),
            ..default()
        },
        StartButtonUI,
    );

    let button_text = TextBundle::from_section(
        "Start Game!",
        TextStyle {
            font: font.clone(),
            font_size: 40.0,
            color: Color::rgb(0.9, 0.9, 0.9),
        },
    );

    let button_2 = (
        ButtonBundle {
            style: Style {
                size: Size::new(Val::Percent(70.0), Val::Percent(30.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                ..default()
            },

            background_color: Color::CRIMSON.into(),
            ..default()
        },
        AboutButtonUI,
    );

    let button_text_2 = TextBundle::from_section(
        "About",
        TextStyle {
            font,
            font_size: 40.0,
            color: Color::rgb(0.9, 0.9, 0.9),
        },
    );

    commands.spawn(menu_parent).with_children(|commands| {
        commands.spawn(button).with_children(|commands| {
            commands.spawn(button_text);
        });
        commands.spawn(button_2).with_children(|commands| {
            commands.spawn(button_text_2);
        });
    });

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 100.0),
            texture: assets.load("backgroundColorGrassWithTitle.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(
                    333.0 * PIXEL_TO_WORLD * 3.86,
                    186.0 * PIXEL_TO_WORLD * 3.86,
                )),
                ..default()
            },
            ..default()
        },
        MainMenuUI,
    ));
}

fn spawn_game_over_ui(mut commands: Commands, assets: Res<AssetServer>) {
    let font = assets.load("fonts/dos.ttf");

    let menu_parent = (
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                padding: UiRect::left(Val::Percent(3.0)),
                ..default()
            },
            ..default()
        },
        GameOverUI,
    );

    let menu_title = NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(70.0), Val::Percent(60.0)),
            position_type: PositionType::Relative,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceAround,
            ..default()
        },
        background_color: Color::DARK_GRAY.into(),
        ..default()
    };

    let button = (
        ButtonBundle {
            style: Style {
                size: Size::new(Val::Percent(50.0), Val::Percent(15.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                ..default()
            },

            background_color: Color::CRIMSON.into(),
            ..default()
        },
        GameOverButtonUI,
    );

    let title_text = TextBundle::from_section(
        "Game Over!",
        TextStyle {
            font: font.clone(),
            font_size: 64.0,
            color: Color::rgb(0.9, 0.9, 0.9),
        },
    );

    let button_text = TextBundle::from_section(
        "Back to Menu",
        TextStyle {
            font,
            font_size: 40.0,
            color: Color::rgb(0.9, 0.9, 0.9),
        },
    );

    commands.spawn(menu_parent).with_children(|commands| {
        commands.spawn(menu_title).with_children(|commands| {
            commands.spawn(title_text);
            commands.spawn(button).with_children(|commands| {
                commands.spawn(button_text);
            });
        });
    });
}

fn spawn_about_ui(mut commands: Commands, assets: Res<AssetServer>) {
    let font = assets.load("fonts/dos.ttf");

    let about_parent = (
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            visibility: Visibility::Hidden,
            z_index: ZIndex::Global(100),
            ..default()
        },
        AboutUI,
    );

    let about_box = NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(95.0), Val::Percent(95.0)),
            position_type: PositionType::Relative,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceAround,
            ..default()
        },
        background_color: Color::DARK_GRAY.into(),
        ..default()
    };

    let button = (
        ButtonBundle {
            style: Style {
                size: Size::new(Val::Percent(50.0), Val::Percent(15.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                ..default()
            },

            background_color: Color::CRIMSON.into(),
            ..default()
        },
        AboutBackButton,
    );

    let font_size = 24.0;

    let text = vec![
        TextSection {
            value: "BevyQuest".to_string(),
            style: TextStyle {
                font: font.clone(),
                font_size,
                color: Color::DARK_GREEN,
            },
        },
        TextSection {
            value: " is a action rougelike game designed for my wife".to_string(),
            style: TextStyle {
                font: font.clone(),
                font_size,
                color: Color::CRIMSON,
            },
        },
        TextSection {
            value: " Trisha".to_string(),
            style: TextStyle {
                font: font.clone(),
                font_size,
                color: Color::TEAL,
            },
        },

        TextSection {
            value: "\n\nAll assets were sourced from kenney.nl or purchased for use.\n\nMuch of the code is structured around work done by  the Matthew Bryant (LogicProjects on Youtube).".to_string(),
            style: TextStyle {
                font: font.clone(),
                font_size,
                color: Color::CRIMSON,
            },
        },
    ];

    let mut title_text = TextBundle::from_sections(text);

    // https://github.com/bevyengine/bevy/issues/1490
    title_text.style.size.width = Val::Px(WIDTH * 0.90);

    let button_text = TextBundle::from_section(
        "Back to Menu",
        TextStyle {
            font: font.clone(),
            font_size: 40.0,
            color: Color::rgb(0.9, 0.9, 0.9),
        },
    );

    let main_title_text = TextBundle::from_section(
        "About BevyQuest",
        TextStyle {
            font: font.clone(),
            font_size: 40.0,
            color: Color::rgb(0.9, 0.9, 0.9),
        },
    );

    commands.spawn(about_parent).with_children(|commands| {
        commands.spawn(about_box).with_children(|commands| {
            commands.spawn(main_title_text);
            commands.spawn(title_text);
            commands.spawn(button).with_children(|commands| {
                commands.spawn(button_text);
            });
        });
    });
}

fn player_health_ui_sync(mut ui: Query<&mut Style, With<HealthUI>>, player: Query<&Player>) {
    let mut style = ui.single_mut();
    let player = player.single();

    let percent = player.health / player.max_health;
    style.size.width = Val::Percent(percent * 100.0);
}

fn spawn_player_ui(mut commands: Commands) {
    let parent_node = (
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(5.0), Val::Percent(2.0)),
                position: UiRect {
                    left: Val::Percent(47.5),
                    right: Val::Auto,
                    top: Val::Percent(60.0),
                    bottom: Val::Auto,
                },
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                flex_direction: FlexDirection::Row,
                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        },
        GamePlayEntity,
        PlayerUI,
        Name::new("Player UI"),
    );

    let health_node = (
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(0.0), Val::Percent(100.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::RED),
            ..default()
        },
        HealthUI,
        Name::new("Health UI"),
    );

    commands.spawn(parent_node).with_children(|commands| {
        commands.spawn(health_node);
    });
}

fn update_world_text(
    mut commands: Commands,
    mut text: Query<(Entity, &mut Style, &mut WorldTextUI)>,
    main_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    render_camera: Query<&Camera, With<FinalCamera>>,
    time: Res<Time>,
) {
    //AHHH
    let (camera, transform) = main_camera.single();
    let final_camera = render_camera.single();

    for (entity, mut style, mut world_ui) in &mut text {
        world_ui.lifetime.tick(time.delta());
        if world_ui.lifetime.just_finished() {
            commands.entity(entity).despawn_recursive();
        }

        world_ui.position = world_ui.position + world_ui.velocity * time.delta_seconds();

        if let Some(coords) = camera.world_to_viewport(transform, world_ui.position.extend(0.0)) {
            let mut coords = coords / Vec2::new(RENDER_WIDTH, RENDER_HEIGHT)
                * final_camera.logical_viewport_size().unwrap();
            coords.y = final_camera.logical_viewport_size().unwrap().y - coords.y;

            style.position = UiRect {
                top: Val::Px(coords.y),
                left: Val::Px(coords.x),
                bottom: Val::Px(coords.y),
                right: Val::Px(coords.x),
            }
        }
    }
}

pub fn spawn_world_text(commands: &mut Commands, assets: &AssetServer, position: Vec2, text: &str) {
    let font = assets.load("fonts/dos.ttf");

    let position = position + Vec2::new(-0.2, 1.4);

    let parent = (
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(20.0), Val::Percent(20.0)),
                position_type: PositionType::Absolute,
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexStart,
                ..default()
            },
            z_index: ZIndex::Global(-100),
            ..default()
        },
        WorldTextUI {
            lifetime: Timer::from_seconds(0.5, TimerMode::Once),
            velocity: Vec2::new(0.15, 1.5),
            position,
        },
    );

    let text = TextBundle::from_section(
        text,
        TextStyle {
            font,
            font_size: 32.0,
            color: Color::rgb(0.95, 0.2, 0.2),
        },
    );

    commands.spawn(parent).with_children(|commands| {
        commands.spawn(text);
    });
}
