use crate::{
    app::{anime_effect, theme::*, ui},
    book::page::*,
};
use bevy_ui_navigation::{prelude::*, NavRequestSystem};

const PAGE_CODE: &str = "dev";
const PAGE_NAME: &str = "Dev";
const PAGE_ICON: &str = "wrench";

pub struct Page;

impl PageBase for Page {
    fn code(&self) -> &str {
        PAGE_CODE
    }
    fn name(&self) -> &str {
        PAGE_NAME
    }
    fn icon(&self) -> &str {
        PAGE_ICON
    }
    fn state(&self) -> PageState {
        PageState::Dev
    }
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(self.state()), page_enter)
            .add_systems(
                Update,
                handle_ui_navigation
                    .after(NavRequestSystem)
                    .run_if(in_state(self.state())),
            )
            .add_systems(
                OnExit(self.state()),
                (
                    anime_effect::clear_anime_effect,
                    app::ui::despawn_ui::<OnPage>,
                ),
            );
    }
}

#[derive(Component)]
struct OnPage;

#[derive(Component)]
enum ButtonAction {
    MoveToPage(PageState),
}

#[derive(Component)]
struct LeaderboardList(String);

#[derive(Component)]
struct ScreenshotPanel;

#[derive(Component)]
struct ScreenshotImage;

const COLORS: [Color; 4] = [FG_COLOR, BG_COLOR, SECONDARY_COLOR, MUTE_COLOR];

fn page_enter(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((build_page_layout(), OnPage))
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
                    build_game_title(parent, &asset_server);
                    build_page_title(parent, &asset_server, PAGE_NAME, PAGE_ICON);
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            build_sep_title(parent, &asset_server, "Font", "text-aa-fill");
                            parent.spawn(
                                TextBundle::from_section(
                                    "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789,.-",
                                    TextStyle {
                                        font: asset_server.load(FONT),
                                        font_size: ui::FONT_SIZE,
                                        color: FG_COLOR,
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::vertical(app::ui::px_p(2.0)),
                                    ..default()
                                }),
                            );
                            parent.spawn(
                                TextBundle::from_section(
                                    "abcdefghijklmnopqrstuvwxyzα!@#$%^&*()+=",
                                    TextStyle {
                                        font: asset_server.load(FONT),
                                        font_size: ui::FONT_SIZE,
                                        color: FG_COLOR,
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::vertical(app::ui::px_p(2.0)),
                                    ..default()
                                }),
                            );
                            parent.spawn(
                                TextBundle::from_section(
                                    "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789,.-",
                                    TextStyle {
                                        font: asset_server.load(FONT),
                                        font_size: ui::FONT_SIZE,
                                        color: FG_COLOR,
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::vertical(app::ui::px_p(2.0)),
                                    ..default()
                                }),
                            );
                            parent.spawn(
                                TextBundle::from_section(
                                    "abcdefghijklmnopqrstuvwxyzα!@#$%^&*()+=",
                                    TextStyle {
                                        font: asset_server.load(FONT),
                                        font_size: ui::FONT_SIZE,
                                        color: FG_COLOR,
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::vertical(app::ui::px_p(2.0)),
                                    ..default()
                                }),
                            );
                            parent.spawn(
                                TextBundle::from_section(
                                    "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789,.-",
                                    TextStyle {
                                        font: asset_server.load(FONT_TITLE),
                                        font_size: ui::FONT_SIZE,
                                        color: FG_COLOR,
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::vertical(app::ui::px_p(2.0)),
                                    ..default()
                                }),
                            );
                            parent.spawn(
                                TextBundle::from_section(
                                    "abcdefghijklmnopqrstuvwxyzα!@#$%^&*()+=",
                                    TextStyle {
                                        font: asset_server.load(FONT_TITLE),
                                        font_size: ui::FONT_SIZE,
                                        color: FG_COLOR,
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::vertical(app::ui::px_p(2.0)),
                                    ..default()
                                }),
                            );
                            build_sep_title(parent, &asset_server, "Color", "palette-fill");
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        justify_content: JustifyContent::Center,
                                        ..default()
                                    },
                                    background_color: BG_COLOR.into(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    for color in COLORS {
                                        parent
                                            .spawn(NodeBundle {
                                                style: Style {
                                                    margin: UiRect::all(app::ui::px_p(2.0)),
                                                    padding: UiRect::all(app::ui::px_p(2.0)),
                                                    border: UiRect::all(app::ui::px_p(0.5)),
                                                    ..default()
                                                },
                                                background_color: BG_COLOR.into(),
                                                border_color: FG_COLOR.into(),
                                                ..default()
                                            })
                                            .with_children(|parent| {
                                                parent.spawn(NodeBundle {
                                                    style: Style {
                                                        width: Val::Px(app::ui::ICON_SIZE * 1.5),
                                                        height: Val::Px(app::ui::ICON_SIZE * 1.5),
                                                        ..default()
                                                    },
                                                    background_color: color.into(),
                                                    ..default()
                                                });
                                            });
                                    }
                                });
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        justify_content: JustifyContent::Center,
                                        ..default()
                                    },
                                    background_color: BG_COLOR.into(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    for ach_def in app::achievement::ACHIEVEMENTS {
                                        parent
                                            .spawn(NodeBundle {
                                                style: Style {
                                                    margin: UiRect::all(app::ui::px_p(2.0)),
                                                    padding: UiRect::all(app::ui::px_p(2.0)),
                                                    border: UiRect::all(app::ui::px_p(0.5)),
                                                    ..default()
                                                },
                                                background_color: BG_COLOR.into(),
                                                border_color: FG_COLOR.into(),
                                                ..default()
                                            })
                                            .with_children(|parent| {
                                                parent.spawn(NodeBundle {
                                                    style: Style {
                                                        width: Val::Px(app::ui::ICON_SIZE * 1.5),
                                                        height: Val::Px(app::ui::ICON_SIZE * 1.5),
                                                        ..default()
                                                    },
                                                    background_color: ach_def.color().into(),
                                                    ..default()
                                                });
                                            });
                                    }
                                });
                        });
                });
            app::ui::build_icon_btn(
                parent,
                &asset_server,
                (
                    ButtonAction::MoveToPage(PageState::Menu),
                    app::interaction::IaButton,
                    Focusable::new().prioritized(),
                ),
                Style {
                    position_type: PositionType::Absolute,
                    bottom: app::ui::px_p(app::ui::PAGE_PADDING),
                    left: app::ui::px_p(app::ui::PAGE_PADDING),
                    ..default()
                },
                "arrow-left-light_1.5x",
            );
        });
}

fn handle_ui_navigation(
    mut actions: Query<&mut ButtonAction>,
    mut events: EventReader<NavEvent>,
    mut page_state: ResMut<NextState<PageState>>,
) {
    events.nav_iter().activated_in_query_foreach_mut(
        &mut actions,
        |mut action| match &mut *action {
            ButtonAction::MoveToPage(state) => page_state.set(*state),
        },
    );
}
