use crate::{
    app,
    page::*,
    reactor::{field, particle, status},
};
use bevy_persistent::prelude::*;
use bevy_ui_navigation::{prelude::*, NavRequestSystem};

const PAGE_CODE: &str = "leaderboard";
const PAGE_NAME: &str = "Report";
const PAGE_ICON: &str = "list-numbers";

pub struct PageDef;

impl PageDefBase for PageDef {
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
        PageState::Leaderboard
    }
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(self.state()), page_enter)
            .add_systems(
                Update,
                handle_ui_navigation
                    .after(NavRequestSystem)
                    .run_if(in_state(self.state())),
            )
            .add_systems(OnExit(self.state()), app::ui::despawn_ui::<OnPage>);
    }
}

#[derive(Component)]
struct OnPage;

#[derive(Component)]
enum ButtonAction {
    SwitchList(String),
    BackToMainMenu,
    #[cfg(not(target_arch = "wasm32"))]
    ShowScreenshot(String, String),
    CloseScreenshot,
}

#[derive(Component)]
struct LeaderboardList(String);

#[derive(Component)]
struct ScreenshotPanel;

#[derive(Component)]
struct ScreenshotImage;

const LB_FS: f32 = app::ui::FONT_SIZE;
const LB_ICON_SIZE: f32 = 12.0;

fn page_enter(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    leaderboard: Res<Persistent<app::leaderboard::Leaderboard>>,
    status: Res<status::ReactorStatus>,
) {
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
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            build_list(parent, &asset_server, &leaderboard, &status, "score");
                            build_list(parent, &asset_server, &leaderboard, &status, "time");
                            build_list(
                                parent,
                                &asset_server,
                                &leaderboard,
                                &status,
                                "max_alpha_count",
                            );
                            build_list(
                                parent,
                                &asset_server,
                                &leaderboard,
                                &status,
                                "max_control_chain",
                            );
                            build_list(
                                parent,
                                &asset_server,
                                &leaderboard,
                                &status,
                                "max_hyper_chain",
                            );
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        position_type: PositionType::Absolute,
                                        bottom: Val::Px(0.0),
                                        left: Val::Px(0.0),
                                        right: Val::Px(0.0),
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        column_gap: app::ui::px_p(4.0),
                                        ..default()
                                    },
                                    background_color: app::ui::BG_COLOR.into(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    app::ui::build_icon_btn(
                                        parent,
                                        &asset_server,
                                        (
                                            ButtonAction::SwitchList(String::from("score")),
                                            app::interaction::IaButton,
                                            Focusable::default(),
                                        ),
                                        Style::default(),
                                        "trophy-fill",
                                    );
                                    app::ui::build_icon_btn(
                                        parent,
                                        &asset_server,
                                        (
                                            ButtonAction::SwitchList(String::from("time")),
                                            app::interaction::IaButton,
                                            Focusable::default(),
                                        ),
                                        Style::default(),
                                        "timer-fill",
                                    );
                                    app::ui::build_icon_btn(
                                        parent,
                                        &asset_server,
                                        (
                                            ButtonAction::SwitchList(String::from(
                                                "max_alpha_count",
                                            )),
                                            app::interaction::IaButton,
                                            Focusable::default(),
                                        ),
                                        Style::default(),
                                        "circles-three-fill",
                                    );
                                    let icon = asset_server.load("images/icons/line-segments.png");
                                    parent.spawn(ImageBundle {
                                        style: Style {
                                            width: Val::Px(app::ui::ICON_SIZE),
                                            height: Val::Px(app::ui::ICON_SIZE),
                                            margin: UiRect::left(app::ui::px_p(8.0)),
                                            ..default()
                                        },
                                        image: UiImage::new(icon),
                                        ..default()
                                    });
                                    app::ui::build_icon_btn(
                                        parent,
                                        &asset_server,
                                        (
                                            ButtonAction::SwitchList(String::from(
                                                "max_control_chain",
                                            )),
                                            app::interaction::IaButton,
                                            Focusable::default(),
                                        ),
                                        Style::default(),
                                        "square",
                                    );
                                    app::ui::build_icon_btn(
                                        parent,
                                        &asset_server,
                                        (
                                            ButtonAction::SwitchList(String::from(
                                                "max_hyper_chain",
                                            )),
                                            app::interaction::IaButton,
                                            Focusable::default(),
                                        ),
                                        Style::default(),
                                        "hexagon",
                                    );
                                });
                        });
                });
            app::ui::build_icon_btn(
                parent,
                &asset_server,
                (
                    ButtonAction::BackToMainMenu,
                    app::interaction::IaButton,
                    Focusable::new().prioritized(),
                ),
                Style {
                    position_type: PositionType::Absolute,
                    bottom: app::ui::px_p(app::ui::PAGE_PADDING),
                    left: app::ui::px_p(app::ui::PAGE_PADDING),
                    ..default()
                },
                "arrow-left-light",
            );
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            top: Val::Px(0.0),
                            bottom: Val::Px(0.0),
                            left: Val::Px(0.0),
                            right: Val::Px(0.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            padding: UiRect::all(app::ui::px_p(3.0)),
                            ..default()
                        },
                        background_color: app::ui::BG_COLOR.into(),
                        visibility: Visibility::Hidden,
                        ..default()
                    },
                    ScreenshotPanel,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        ImageBundle {
                            style: Style {
                                max_width: Val::Percent(100.0),
                                max_height: Val::Percent(100.0),
                                ..default()
                            },
                            image: UiImage::default(),
                            ..default()
                        },
                        ScreenshotImage,
                    ));
                    app::ui::build_icon_btn(
                        parent,
                        &asset_server,
                        (
                            ButtonAction::CloseScreenshot,
                            app::interaction::IaButton,
                            Focusable::default(),
                        ),
                        Style {
                            position_type: PositionType::Absolute,
                            bottom: app::ui::px_p(app::ui::PAGE_PADDING),
                            left: app::ui::px_p(app::ui::PAGE_PADDING),
                            ..default()
                        },
                        "arrow-left-light",
                    );
                });
        });
}

type LeaderboardListOnly = (With<LeaderboardList>, Without<ScreenshotPanel>);
type ScreenshotPanelOnly = (With<ScreenshotPanel>, Without<LeaderboardList>);

fn handle_ui_navigation(
    mut actions: Query<&mut ButtonAction>,
    mut events: EventReader<NavEvent>,
    mut page_state: ResMut<NextState<PageState>>,
    mut lb_lists: Query<(&LeaderboardList, &mut Visibility), LeaderboardListOnly>,
    mut ss_panel_query: Query<&mut Visibility, ScreenshotPanelOnly>,
    #[cfg(not(target_arch = "wasm32"))] mut ss_image_query: Query<
        &mut UiImage,
        With<ScreenshotImage>,
    >,
    #[cfg(not(target_arch = "wasm32"))] mut images: ResMut<Assets<Image>>,
) {
    events.nav_iter().activated_in_query_foreach_mut(
        &mut actions,
        |mut action| match &mut *action {
            ButtonAction::BackToMainMenu => page_state.set(PageState::Menu),
            ButtonAction::SwitchList(list) => {
                for (lb_list, mut visibility) in lb_lists.iter_mut() {
                    if lb_list.0 == list.as_str() {
                        *visibility = Visibility::Visible;
                    } else {
                        *visibility = Visibility::Hidden;
                    }
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            ButtonAction::ShowScreenshot(uid, ss_type) => {
                let mut ss_image = ss_image_query.single_mut();
                let image = app::screenshot::fetch_screenshot_image(uid, ss_type);
                let image_handle = images.add(image);
                ss_image.texture = image_handle;
                let mut visibility = ss_panel_query.single_mut();
                *visibility = Visibility::Visible;
            }
            ButtonAction::CloseScreenshot => {
                let mut visibility = ss_panel_query.single_mut();
                *visibility = Visibility::Hidden;
            }
        },
    );
}

fn build_list(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    leaderboard: &Res<Persistent<app::leaderboard::Leaderboard>>,
    status: &Res<status::ReactorStatus>,
    list: &str,
) -> Entity {
    let records = leaderboard.fetch_records(list);
    let visibility = if list == "score" {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };
    let mut row_gap = app::ui::px_p(4.0);
    if cfg!(not(target_arch = "wasm32")) && (list == "score" || list == "max_alpha_count") {
        row_gap = app::ui::px_p(2.4)
    }
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    row_gap,
                    ..default()
                },
                visibility,
                ..default()
            },
            LeaderboardList(String::from(list)),
        ))
        .with_children(|parent| {
            let mut rank = 1;
            let mut prev_number: u32 = 0;
            for i in 0..app::leaderboard::MAX_RECORDS_PER_LIST {
                if let Some(record) = records.get(i) {
                    let number = record.fetch(list);
                    let number_text = match list {
                        "score" => field::format_field_text("score", number),
                        "time" => field::format_field_text("time", number),
                        "max_alpha_count" => field::format_field_text("alpha_count", number),
                        "max_control_chain" => field::format_field_text("chain", number),
                        "max_hyper_chain" => field::format_field_text("chain", number),
                        _ => panic!("Invalid list"),
                    };
                    let text_color = if record.uid() == status.highlight_uid {
                        particle::uou::COLOR
                    } else {
                        app::ui::FG_COLOR
                    };
                    let number_color = if record.uid() == status.highlight_uid {
                        particle::uou::COLOR
                    } else {
                        match list {
                            "score" | "time" | "max_alpha_count" => app::ui::FG_COLOR,
                            "max_control_chain" => particle::control::COLOR,
                            "max_hyper_chain" => particle::hyper::COLOR,
                            _ => panic!("Invalid list"),
                        }
                    };
                    let icon_path = match list {
                        "score" => "images/icons/trophy-fill.png",
                        "time" => "images/icons/timer-fill.png",
                        "max_alpha_count" => "images/icons/circles-three-fill.png",
                        "max_control_chain" | "max_hyper_chain" => "images/icons/line-segments.png",
                        _ => panic!("Invalid list"),
                    };
                    if i == 0 {
                        rank = 1;
                        prev_number = number;
                    } else if number < prev_number {
                        rank = i + 1;
                        prev_number = number;
                    }
                    let rank_text = match rank {
                        1 => String::from("1st"),
                        2 => String::from("2nd"),
                        3 => String::from("3rd"),
                        _ => format!("{}th", rank),
                    };
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::SpaceBetween,
                                column_gap: app::ui::px_p(4.0),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            let icon = asset_server.load(icon_path);
                            parent.spawn(ImageBundle {
                                style: Style {
                                    width: app::ui::px_p(LB_ICON_SIZE),
                                    height: app::ui::px_p(LB_ICON_SIZE),
                                    ..default()
                                },
                                image: UiImage::new(icon),
                                ..default()
                            });
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::SpaceBetween,
                                        padding: UiRect::all(app::ui::px_p(3.0)),
                                        ..default()
                                    },
                                    background_color: text_color.into(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn((TextBundle::from_section(
                                        rank_text.to_string(),
                                        TextStyle {
                                            font: asset_server.load(app::ui::FONT_DIGIT),
                                            font_size: LB_FS * 0.6,
                                            color: app::ui::BG_COLOR,
                                        },
                                    ),));
                                });
                            parent.spawn((TextBundle::from_section(
                                format!("{:<12}", record.player_name),
                                TextStyle {
                                    font: asset_server.load(app::ui::FONT_DIGIT),
                                    font_size: LB_FS,
                                    color: text_color,
                                },
                            ),));
                            if list == "max_control_chain" {
                                let icon = asset_server.load("images/icons/square.png");
                                parent.spawn(ImageBundle {
                                    style: Style {
                                        width: app::ui::px_p(LB_ICON_SIZE),
                                        height: app::ui::px_p(LB_ICON_SIZE),
                                        ..default()
                                    },
                                    image: UiImage::new(icon),
                                    ..default()
                                });
                            }
                            if list == "max_hyper_chain" {
                                let icon = asset_server.load("images/icons/hexagon.png");
                                parent.spawn(ImageBundle {
                                    style: Style {
                                        width: app::ui::px_p(LB_ICON_SIZE),
                                        height: app::ui::px_p(LB_ICON_SIZE),
                                        ..default()
                                    },
                                    image: UiImage::new(icon),
                                    ..default()
                                });
                            }
                            parent.spawn((TextBundle::from_section(
                                number_text,
                                TextStyle {
                                    font: asset_server.load(app::ui::FONT_DIGIT),
                                    font_size: LB_FS,
                                    color: number_color,
                                },
                            ),));
                            if list == "score" || list == "max_alpha_count" {
                                #[cfg(not(target_arch = "wasm32"))]
                                app::ui::build_icon_btn(
                                    parent,
                                    asset_server,
                                    (
                                        ButtonAction::ShowScreenshot(
                                            String::from(record.uid()),
                                            String::from(list),
                                        ),
                                        app::interaction::IaButton,
                                        Focusable::default(),
                                    ),
                                    Style { ..default() },
                                    "frame-corners",
                                );
                            }
                        });
                }
            }
        })
        .id()
}
