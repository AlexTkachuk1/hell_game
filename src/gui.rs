use bevy::app::Plugin;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

use crate::animation::AnimationTimer;
use crate::castle::Castle;
use crate::enemy::Enemy;
use crate::player::{GoldCount, Health, Player};
use crate::state::GameState;
use crate::world::GameEntity;
use crate::{GlobalTextureAtlas, MENU_SPRITE_SCALE_FACTOR};

#[derive(Component)]
struct DebugText;

#[derive(Component)]
struct CoinText;

#[derive(Component)]
pub struct MenuBG;

#[derive(Component)]
struct MainMenuItem;
#[derive(Component)]
struct MenuImage;
pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin)
            .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(OnExit(GameState::MainMenu), despawn_main_menu)
            .add_systems(
                Update,
                handle_main_menu_buttons.run_if(in_state(GameState::MainMenu)),
            )
            .add_systems(OnEnter(GameState::InGame), (spawn_debug_text, spawn_res_ui))
            .add_systems(
                Update,
                (update_debug_text, update_res_text).run_if(in_state(GameState::InGame)),
            );
    }
}

fn spawn_debug_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::Start,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            GameEntity,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(440.0),
                        height: Val::Px(165.0),
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        padding: UiRect::all(Val::Px(8.0)),
                        margin: UiRect::px(10.0, 10.0, 10.0, 0.0),
                        ..default()
                    },
                    background_color: BackgroundColor::from(Color::BLACK.with_a(0.9)),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Hello Bevy!",
                            TextStyle {
                                font: asset_server.load("monogram.ttf"),
                                font_size: 40.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ),
                        DebugText,
                    ));
                });
        });
}

fn spawn_res_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    handle: Res<GlobalTextureAtlas>,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::End,
                    justify_content: JustifyContent::SpaceBetween,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            },
            GameEntity,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(1600.),
                        height: Val::Px(50.0),
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        padding: UiRect::all(Val::Px(8.0)),
                        margin: UiRect::px(10.0, 10.0, 10.0, 10.0),
                        ..default()
                    },
                    background_color: BackgroundColor::from(Color::BLACK.with_a(0.9)),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(AtlasImageBundle {
                        style: Style {
                            width: Val::Px(40.),
                            height: Val::Px(40.),
                            ..default()
                        },
                        texture_atlas: TextureAtlas {
                            layout: handle.coin_layout.clone().unwrap(),
                            index: 0,
                        },
                        image: UiImage::new(handle.coin_image.clone().unwrap()),
                        ..default()
                    });
                    parent.spawn((
                        TextBundle::from_section(
                            ": 0",
                            TextStyle {
                                font: asset_server.load("monogram.ttf"),
                                font_size: 40.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ),
                        CoinText,
                    ));
                });
        });
}

fn update_debug_text(
    mut query: Query<&mut Text, With<DebugText>>,
    diagnostics: Res<DiagnosticsStore>,
    enemy_query: Query<(), With<Enemy>>,
    player_query: Query<&Health, With<Player>>,
    castle_query: Query<&Health, With<Castle>>,
) {
    if query.is_empty() || player_query.is_empty() || enemy_query.is_empty() || castle_query.is_empty() {
        return;
    }

    let num_enemies = enemy_query.iter().count();
    let player_health = player_query.single().0;
    let castle_health = castle_query.single().0;
    let mut text = query.single_mut();
    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.smoothed() {
            text.sections[0].value =
                format!("Fps: {value:.2}\nEnemies: {num_enemies}\nHealth: {player_health}\nCastle_Health: {castle_health}");
        }
    }
}

fn update_res_text(
    mut query: Query<&mut Text, With<CoinText>>,
    player_query: Query<&GoldCount, With<Player>>,
) {
    if query.is_empty() || player_query.is_empty() {
        return;
    }

    let player_gold = player_query.single().0;
    let mut text = query.single_mut();

    text.sections[0].value = format!(": {player_gold}");
}

fn setup_main_menu(
    mut commands: Commands,
    handle: Res<GlobalTextureAtlas>,
) {
    commands
        .spawn((SpriteSheetBundle {
            texture: handle.menu_image.clone().unwrap(),
            atlas: TextureAtlas {
                layout: handle.menu_layout.clone().unwrap(),
                index: 0,
            },
            transform: Transform::from_scale(Vec3::splat(MENU_SPRITE_SCALE_FACTOR)),
            ..default()
        },
        MenuBG,
        AnimationTimer(Timer::from_seconds(0.08, TimerMode::Repeating)),
        MainMenuItem,
        ));

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BackgroundColor::from(Color::BLACK.with_a(0.)),
                    ..default()
                }).with_children(|parent| {
                    parent.spawn((
                        AtlasImageBundle {
                            style: Style {
                                width: Val::Px(200.),
                                height: Val::Px(65.),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            texture_atlas: TextureAtlas {
                                layout: handle.button_layout.clone().unwrap(),
                                index: 0,
                            },
                            image: UiImage::new(handle.button_image.clone().unwrap()),
                            ..default()
                        },
                        Interaction::default(),
                        MenuImage,
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "Play",
                            TextStyle {
                                
                                font_size: 40.0,
                                color: Color::BLACK,
                                ..default()
                            },
                        ));
                    });
                });
        })
        .insert(MainMenuItem);
}

fn handle_main_menu_buttons(
    mut button_query: Query<(&mut TextureAtlas, &Interaction),  (Changed<Interaction>, With<MenuImage>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if button_query.is_empty() {
        return;
    }

    for (mut atlas, interaction) in button_query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                atlas.index = 2;
                next_state.set(GameState::GameInit);
            }
            Interaction::Hovered => {
                atlas.index = 1;
            }
            Interaction::None => {
                atlas.index = 0;
            }
        }
    }
}

fn despawn_main_menu(mut commands: Commands, menu_items_query: Query<Entity, With<MainMenuItem>>) {
    for e in menu_items_query.iter() {
        commands.entity(e).despawn_recursive();
    }
}
