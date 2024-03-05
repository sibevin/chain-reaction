use crate::{app, app::theme::*, app::ui, book::*};

pub mod about;
pub mod achievement;
pub mod dev;
pub mod game;
pub mod help;
pub mod history;
pub mod leaderboard;
pub mod menu;
pub mod settings;

pub trait PageBase {
    fn code(&self) -> &str;
    fn name(&self) -> &str;
    fn icon(&self) -> &str;
    fn state(&self) -> PageState;
    fn build(&self, app: &mut App);
}

const PAGE_TITLE_RATIO: f32 = 1.2;
const SEP_W: f32 = 120.0;

fn build_page_layout() -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            padding: UiRect::all(ui::px_p(ui::PAGE_PADDING)),
            ..default()
        },
        background_color: COVER_COLOR.into(),
        ..default()
    }
}

fn build_game_title(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: ui::px_p(ui::PAGE_PADDING),
                left: ui::px_p(ui::PAGE_PADDING),
                align_items: AlignItems::Center,
                column_gap: ui::px_p(2.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            let icon = asset_server.load("images/app/title_small.png");
            parent.spawn(ImageBundle {
                image: UiImage::new(icon),
                ..default()
            });
        });
}

fn build_page_title(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    text: &str,
    icon: &str,
) -> Entity {
    parent
        .spawn((NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: ui::px_p(ui::PAGE_PADDING),
                right: ui::px_p(ui::PAGE_PADDING),
                height: Val::Auto,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                column_gap: ui::px_p(4.0),
                padding: UiRect::all(ui::px_p(2.0)),
                ..default()
            },
            background_color: BG_COLOR.into(),
            ..default()
        },))
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    text,
                    TextStyle {
                        font: asset_server.load(FONT_TITLE),
                        font_size: ui::FONT_SIZE * PAGE_TITLE_RATIO,
                        color: FG_COLOR,
                    },
                )
                .with_style(Style {
                    margin: UiRect::right(ui::px_p(2.0)),
                    ..default()
                }),
            );
            let icon_path = format!("images/icons/{}.png", icon);
            let icon = asset_server.load(icon_path);
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Px(ui::ICON_SIZE * PAGE_TITLE_RATIO),
                    height: Val::Px(ui::ICON_SIZE * PAGE_TITLE_RATIO),
                    ..default()
                },
                image: UiImage::new(icon),
                ..default()
            });
        })
        .id()
}

fn build_sep_title(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    text: &str,
    icon: &str,
) -> Entity {
    parent
        .spawn((NodeBundle {
            style: Style {
                width: ui::px_p(SEP_W),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                margin: UiRect::vertical(ui::px_p(5.0)),
                ..default()
            },
            ..default()
        },))
        .with_children(|parent| {
            parent.spawn((NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: ui::px_p(SEP_W),
                    height: ui::px_p(1.2),
                    margin: UiRect::top(ui::px_p(3.0)),
                    ..default()
                },
                background_color: SECONDARY_COLOR.into(),
                ..default()
            },));
            parent
                .spawn((NodeBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        padding: UiRect::horizontal(ui::px_p(3.0)),
                        ..default()
                    },
                    background_color: BG_COLOR.into(),
                    ..default()
                },))
                .with_children(|parent| {
                    let icon_path = format!("images/icons/{}.png", icon);
                    let icon = asset_server.load(icon_path);
                    parent.spawn(ImageBundle {
                        style: Style {
                            width: Val::Px(ui::ICON_SIZE * PAGE_TITLE_RATIO),
                            height: Val::Px(ui::ICON_SIZE * PAGE_TITLE_RATIO),
                            margin: UiRect::right(ui::px_p(4.0)),
                            ..default()
                        },
                        image: UiImage::new(icon),
                        ..default()
                    });
                    parent.spawn(
                        TextBundle::from_section(
                            text,
                            TextStyle {
                                font: asset_server.load(FONT),
                                font_size: ui::FONT_SIZE * PAGE_TITLE_RATIO,
                                color: SECONDARY_COLOR,
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::right(ui::px_p(2.0)),
                            ..default()
                        }),
                    );
                });
        })
        .id()
}
