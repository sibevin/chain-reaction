use bevy::prelude::*;
use webbrowser;

use crate::{app, page};

pub struct PagePlugin;

impl Plugin for PagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(app::GameState::About), page_setup)
            .add_systems(Update, page_action.run_if(in_state(app::GameState::About)))
            .add_systems(OnExit(app::GameState::About), app::ui::despawn_ui::<OnPage>);
    }
}

#[derive(Component)]
struct OnPage;

#[derive(Component)]
enum ButtonAction {
    BackToMainMenu,
    Link(Option<String>),
}

fn page_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((page::build_page_layout(), OnPage))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                align_items: AlignItems::Start,
                                justify_content: JustifyContent::SpaceBetween,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            page::build_game_title(parent, &asset_server);
                            page::build_page_title(
                                parent,
                                &asset_server,
                                "References",
                                "star-light",
                            );
                        });
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                flex_grow: 1.0,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        flex_direction: FlexDirection::Column,
                                        align_items: AlignItems::Center,
                                        margin: UiRect::right(app::ui::px_p(10.0)),
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|parent| {
                                    page::build_sep_title(
                                        parent,
                                        &asset_server,
                                        "Link",
                                        "link-bold",
                                    );
                                    build_about_link(
                                        parent,
                                        &asset_server,
                                        "itch.io",
                                        Some("house-line-light"),
                                        Some("https://sibevin.itch.io/chain-reaction"),
                                        "default",
                                    );
                                    build_about_link(
                                        parent,
                                        &asset_server,
                                        "github.com",
                                        Some("github-logo-light"),
                                        Some("https://github.com/sibevin/chain-reaction"),
                                        "default",
                                    );
                                    page::build_sep_title(
                                        parent,
                                        &asset_server,
                                        "Design",
                                        "pencil-line-fill",
                                    );
                                    build_about_link(
                                        parent,
                                        &asset_server,
                                        "Kait Wang",
                                        None,
                                        None,
                                        "default",
                                    );
                                    page::build_sep_title(
                                        parent,
                                        &asset_server,
                                        "Programming",
                                        "code-bold",
                                    );
                                    build_about_link(
                                        parent,
                                        &asset_server,
                                        "Kait Wang",
                                        None,
                                        None,
                                        "default",
                                    );
                                    page::build_sep_title(
                                        parent,
                                        &asset_server,
                                        "Art",
                                        "palette-fill",
                                    );
                                    build_about_link(
                                        parent,
                                        &asset_server,
                                        "Miya",
                                        None,
                                        None,
                                        "default",
                                    );
                                });
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        flex_direction: FlexDirection::Column,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|parent| {
                                    page::build_sep_title(
                                        parent,
                                        &asset_server,
                                        "Icon",
                                        "shapes-fill",
                                    );
                                    build_about_link(
                                        parent,
                                        &asset_server,
                                        "Phosphor Icons",
                                        Some("globe-light"),
                                        Some("https://phosphoricons.com/"),
                                        "default",
                                    );
                                    page::build_sep_title(
                                        parent,
                                        &asset_server,
                                        "Font",
                                        "text-aa-fill",
                                    );
                                    build_about_link(
                                        parent,
                                        &asset_server,
                                        "SYN NOVA",
                                        Some("globe-light"),
                                        Some("https://www.fontsquirrel.com/fonts/syn-nova"),
                                        "default",
                                    );
                                    build_about_link(
                                        parent,
                                        &asset_server,
                                        "Silkscreen",
                                        Some("globe-light"),
                                        Some("https://kottke.org/plus/type/silkscreen/index.html"),
                                        "digit",
                                    );
                                    build_about_link(
                                        parent,
                                        &asset_server,
                                        "VAG-HandWritten",
                                        Some("globe-light"),
                                        Some("https://www.fontsquirrel.com/fonts/VAG-HandWritten"),
                                        "hw",
                                    );
                                    page::build_sep_title(
                                        parent,
                                        &asset_server,
                                        "Audio",
                                        "microphone-fill",
                                    );
                                    build_about_link(
                                        parent,
                                        &asset_server,
                                        "Synthetic Deception - By GloeleFazzeri",
                                        Some("globe-light"),
                                        Some(
                                            "https://pixabay.com/music/suspense-synthetic-deception-loopable-epic-cyberpunk-crime-music-157454/"
                                        ),
                                        "default",
                                    );
                                    build_about_link(
                                        parent,
                                        &asset_server,
                                        "Heavy Cineamtic Hit - By LordSonny",
                                        Some("globe-light"),
                                        Some(
                                            "https://pixabay.com/sound-effects/heavy-cineamtic-hit-166888/",
                                        ),
                                        "default",
                                    );
                                    build_about_link(
                                        parent,
                                        &asset_server,
                                        "Pick - From Pixabay",
                                        Some("globe-light"),
                                        Some(
                                            "https://pixabay.com/sound-effects/pick-92276/",
                                        ),
                                        "default",
                                    );
                                    build_about_link(
                                        parent,
                                        &asset_server,
                                        "Item Pick Up - From Pixabay",
                                        Some("globe-light"),
                                        Some(
                                            "https://pixabay.com/sound-effects/item-pick-up-38258/"
                                        ),
                                        "default",
                                    );
                                    build_about_link(
                                        parent,
                                        &asset_server,
                                        "Glass Shatter 3 - From Pixabay",
                                        Some("globe-light"),
                                        Some(
                                            "https://pixabay.com/sound-effects/glass-shatter-3-100155/"
                                        ),
                                        "default",
                                    );
                                });
                        });
                    app::ui::build_icon_btn(
                        parent,
                        &asset_server,
                        ButtonAction::BackToMainMenu,
                        Style {
                            align_self: AlignSelf::Start,
                            ..default()
                        },
                        "arrow-left-light",
                    );
                });
        });
}

fn page_action(
    interaction_query: Query<(&Interaction, &ButtonAction), (Changed<Interaction>, With<Button>)>,
    mut game_state: ResMut<NextState<app::GameState>>,
) {
    for (interaction, action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match action {
                ButtonAction::BackToMainMenu => game_state.set(app::GameState::Menu),
                ButtonAction::Link(url) => {
                    if let Some(url) = url {
                        let _ = webbrowser::open(url);
                    }
                }
            }
        }
    }
}

fn build_about_link(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    text: &str,
    icon: Option<&str>,
    link: Option<&str>,
    font: &str,
) -> Entity {
    let url = if let Some(link) = link {
        Some(String::from(link))
    } else {
        None
    };
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: app::ui::BG_COLOR.into(),
                ..default()
            },
            ButtonAction::Link(url),
            app::ui::LinkButton,
        ))
        .with_children(|parent| {
            parent
                .spawn((NodeBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        padding: UiRect::all(app::ui::px_p(0.5)),
                        ..default()
                    },
                    background_color: app::ui::BG_COLOR.into(),
                    ..default()
                },))
                .with_children(|parent| {
                    if let Some(icon) = icon {
                        let icon_path = format!("images/icons/{}.png", icon);
                        let icon = asset_server.load(icon_path);
                        parent.spawn(ImageBundle {
                            style: Style {
                                width: Val::Px(app::ui::ICON_SIZE),
                                height: Val::Px(app::ui::ICON_SIZE),
                                margin: UiRect::right(app::ui::px_p(4.0)),
                                ..default()
                            },
                            image: UiImage::new(icon),
                            ..default()
                        });
                    }
                    let font = if font == "default" {
                        app::ui::FONT
                    } else if font == "digit" {
                        app::ui::FONT_DIGIT
                    } else {
                        app::ui::FONT_HW
                    };
                    parent.spawn(
                        TextBundle::from_section(
                            text,
                            TextStyle {
                                font: asset_server.load(font),
                                font_size: app::ui::FONT_SIZE,
                                color: app::ui::FG_COLOR,
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::right(app::ui::px_p(2.0)),
                            ..default()
                        }),
                    );
                });
        })
        .id()
}
