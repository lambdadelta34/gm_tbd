use crate::GameState;
use bevy::{app::AppExit, prelude::*};

pub struct BevyPlugin;

impl Plugin for BevyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ButtonMaterials>()
            .add_system_set(
                SystemSet::on_enter(GameState::MainMenu).with_system(build_main_menu.system()),
            )
            .add_system_set(
                SystemSet::on_resume(GameState::MainMenu).with_system(build_main_menu.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameState::MainMenu).with_system(click_menu_item.system()),
            )
            .add_system_set(
                SystemSet::on_pause(GameState::MainMenu).with_system(despawn_menu_items.system()),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::MainMenu).with_system(despawn_menu_items.system()),
            )
            .add_system_set(
                SystemSet::on_enter(GameState::OptionsMenu)
                    .with_system(build_options_menu.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameState::OptionsMenu).with_system(return_button.system()),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::OptionsMenu)
                    .with_system(despawn_options_menu.system()),
            )
            .add_startup_system(create_camera.system());
    }
}

fn create_camera(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}

struct ButtonMaterials {
    none: Handle<ColorMaterial>,
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
    font: Handle<Font>,
}

impl FromWorld for ButtonMaterials {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let font = asset_server.load("fonts/AeroviasBrasil.ttf");
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();

        ButtonMaterials {
            none: materials.add(Color::NONE.into()),
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
            font,
        }
    }
}

struct MainMenu;

#[derive(Clone, Copy)]
enum MenuItem {
    Play,
    Options,
    Exit,
}

pub struct Background;

fn build_main_menu(
    mut commands: Commands,
    button_materials: Res<ButtonMaterials>,
    mut scene_spawner: ResMut<SceneSpawner>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let font = button_materials.font.clone();
    commands
        .spawn_bundle(ImageBundle {
            style: Style {
                size: Size {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                },
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..Default::default()
            },
            material: materials.add(asset_server.load("background.jpeg").into()),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..Default::default()
        })
        .insert(Background)
        .insert(MainMenu)
        .with_children(|mut parent| {
            parent.spawn_bundle(TextBundle {
                style: Style::default(),
                text: Text::with_section(
                    "TITLE TBD",
                    TextStyle {
                        font: font.clone(),
                        font_size: 36.0,
                        color: Color::ORANGE_RED,
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                ..TextBundle::default()
            });

            spawn_button(&mut parent, font.clone(), MenuItem::Play, &button_materials);
            spawn_button(
                &mut parent,
                font.clone(),
                MenuItem::Options,
                &button_materials,
            );
            spawn_button(&mut parent, font.clone(), MenuItem::Exit, &button_materials);
        });
}

fn spawn_button(
    parent: &mut ChildBuilder,
    font: Handle<Font>,
    item: MenuItem,
    button_materials: &Res<ButtonMaterials>,
) {
    parent
        .spawn_bundle(ButtonBundle {
            material: button_materials.none.clone(),
            style: Style::default(),
            ..ButtonBundle::default()
        })
        .insert(item)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style::default(),
                text: Text::with_section(
                    match item {
                        MenuItem::Play => "PLAY",
                        MenuItem::Options => "OPTIONS",
                        MenuItem::Exit => "EXIT",
                    },
                    TextStyle {
                        font,
                        font_size: 20.0,
                        color: Color::BLACK,
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                ..TextBundle::default()
            });
        });
}

fn click_menu_item(
    mut app_exit_events: EventWriter<AppExit>,
    mut app_state: ResMut<State<GameState>>,
    query: Query<(&Interaction, &MenuItem)>,
) {
    query.for_each(|(interaction, item)| match interaction {
        Interaction::Clicked => match item {
            MenuItem::Play => {
                app_state
                    .push(GameState::Game)
                    .map_err(|err| error!("Failed to start game: {}", err))
                    .unwrap();
            }
            MenuItem::Options => {
                app_state
                    .push(GameState::OptionsMenu)
                    .map_err(|err| error!("Failed to open options menu: {}", err))
                    .unwrap();
            }
            MenuItem::Exit => app_exit_events.send(AppExit),
        },
        _ => {}
    });
}

fn despawn_menu_items(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    query.for_each(|entity| commands.entity(entity).despawn_recursive());
}

struct OptionsMenu;
struct ReturnButton;

fn build_options_menu(mut commands: Commands, button_materials: Res<ButtonMaterials>) {
    let font = button_materials.font.clone();

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                // size: Size {
                //     width: Val::Percent(100.0),
                //     height: Val::Percent(100.0),
                // },
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..Style::default()
            },
            visible: Visible {
                is_visible: false,
                ..Visible::default()
            },
            ..NodeBundle::default()
        })
        .insert(OptionsMenu)
        .with_children(|parent| {
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        // size: Size {
                        //     width: Val::Percent(10.),
                        //     height: Val::Px(30.),
                        // },
                        flex_direction: FlexDirection::ColumnReverse,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceEvenly,
                        ..Style::default()
                    },
                    ..ButtonBundle::default()
                })
                .insert(ReturnButton)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        style: Style::default(),
                        text: Text::with_section(
                            "RETURN",
                            TextStyle {
                                font,
                                font_size: 20.0,
                                color: Color::DARK_GRAY,
                            },
                            TextAlignment {
                                vertical: VerticalAlign::Center,
                                horizontal: HorizontalAlign::Center,
                            },
                        ),
                        ..TextBundle::default()
                    });
                });
        });
}

fn despawn_options_menu(mut commands: Commands, query: Query<Entity, With<OptionsMenu>>) {
    query.for_each(|entity| commands.entity(entity).despawn_recursive());
}

fn return_button(
    mut app_state: ResMut<State<GameState>>,
    query: Query<&Interaction, With<ReturnButton>>,
) {
    query.for_each(|interaction| {
        info!("{:?}", interaction);
        match interaction {
            Interaction::Clicked => {
                app_state
                    .pop()
                    .map_err(|err| error!("Failed to return to main menu: {}", err))
                    .unwrap();
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    });
}
