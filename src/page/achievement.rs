use crate::{app, page::*};
use bevy_persistent::prelude::*;
use bevy_ui_navigation::{prelude::*, NavRequestSystem};

pub const PAGE_CODE: &str = "achievement";
pub const PAGE_NAME: &str = "Marks";
pub const PAGE_ICON: &str = "crosshair";

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
        PageState::Achievement
    }
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(self.state()), page_enter)
            .add_systems(
                Update,
                handle_ui_navigation
                    .after(NavRequestSystem)
                    .run_if(in_state(self.state())),
            )
            .add_systems(OnExit(self.state()), (app::ui::despawn_ui::<OnPage>,));
    }
}

#[derive(Component)]
struct OnPage;

#[derive(Component)]
enum ButtonAction {
    TogglePin(String),
    BackToMainMenu,
}

#[derive(Component)]
struct LeaderboardList(String);

#[derive(Component)]
struct ScreenshotPanel;

#[derive(Component)]
struct ScreenshotImage;

#[derive(Component)]
struct AchPanelIcon(String);

fn page_enter(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    achievement: Res<Persistent<app::achievement::AchievementStore>>,
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
                    build_page_title(parent, &asset_server, "Marks", "crosshair");
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
                                        display: Display::Grid,
                                        grid_template_columns: vec![
                                            GridTrack::fr(1.0),
                                            GridTrack::fr(1.0),
                                            GridTrack::fr(1.0),
                                        ],
                                        column_gap: app::ui::px_p(2.0),
                                        row_gap: app::ui::px_p(2.0),
                                        ..default()
                                    },
                                    background_color: app::ui::BG_COLOR.into(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    for ach_def in app::achievement::ACHIEVEMENTS {
                                        let record = achievement.fetch_record(ach_def.code());
                                        build_panel_ui(
                                            parent,
                                            &asset_server,
                                            ach_def,
                                            &record,
                                            &achievement,
                                        );
                                    }
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
                    bottom: app::ui::px_p(PAGE_PADDING),
                    left: app::ui::px_p(PAGE_PADDING),
                    ..default()
                },
                "arrow-left-light",
            );
        });
}

fn handle_ui_navigation(
    mut actions: Query<&mut ButtonAction>,
    mut events: EventReader<NavEvent>,
    mut page_state: ResMut<NextState<PageState>>,
    mut achievement: ResMut<Persistent<app::achievement::AchievementStore>>,
    mut ach_icon_query: Query<(&AchPanelIcon, &mut UiImage), With<AchPanelIcon>>,
    asset_server: Res<AssetServer>,
) {
    events.nav_iter().activated_in_query_foreach_mut(
        &mut actions,
        |mut action| match &mut *action {
            ButtonAction::BackToMainMenu => page_state.set(PageState::Menu),
            ButtonAction::TogglePin(code) => {
                achievement
                    .update(|achievement| {
                        achievement.toggle_pin(code);
                    })
                    .expect("failed to update achievement pin");
                for (ach_icon, mut image) in ach_icon_query.iter_mut() {
                    image.texture = if achievement.is_pinned(ach_icon.0.as_str()) {
                        asset_server.load("images/icons/ach-push-pin.png")
                    } else {
                        asset_server.load("images/icons/ach-crosshair.png")
                    }
                }
            }
        },
    );
}

const ACH_ICON_SIZE: f32 = app::ui::FONT_SIZE * 2.5;
const ACH_STATUS_SIZE: f32 = app::ui::FONT_SIZE * 1.0;
const ACH_PANEL_W: f32 = app::ui::FONT_SIZE * 12.0;
const ACH_DESC_FS: f32 = app::ui::FONT_SIZE * 0.8;
const ACH_NAME_FS: f32 = app::ui::FONT_SIZE * 1.2;

fn build_panel_ui(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    ach_def: &dyn app::achievement::AchievementDefBase,
    record: &app::achievement::AchievementRecord,
    store: &Res<Persistent<app::achievement::AchievementStore>>,
) {
    let color = if record.is_done {
        ach_def.color()
    } else {
        app::ui::MUTE_COLOR
    };
    let mut entity = parent.spawn((ButtonBundle {
        style: Style {
            height: Val::Auto,
            width: Val::Px(ACH_PANEL_W),
            align_items: AlignItems::Start,
            padding: UiRect::all(app::ui::px_p(1.0)),
            border: UiRect::all(app::ui::px_p(1.0)),
            ..default()
        },
        background_color: app::ui::BG_COLOR.into(),
        border_color: color.into(),
        ..default()
    },));
    entity.with_children(|parent| {
        let icon = if record.is_done {
            asset_server.load(ach_def.icon_path())
        } else {
            asset_server.load("images/achievement/locked.png")
        };
        parent.spawn(ImageBundle {
            style: Style {
                width: Val::Px(ACH_ICON_SIZE),
                height: Val::Px(ACH_ICON_SIZE),
                margin: UiRect::all(app::ui::px_p(3.0)),
                ..default()
            },
            image: UiImage::new(icon),
            ..default()
        });
        parent
            .spawn((ButtonBundle {
                style: Style {
                    flex_grow: 1.0,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::SpaceBetween,
                    margin: UiRect::all(app::ui::px_p(1.0)),
                    padding: UiRect::right(Val::Px(ACH_STATUS_SIZE)),
                    ..default()
                },
                background_color: app::ui::BG_COLOR.into(),
                ..default()
            },))
            .with_children(|parent| {
                parent.spawn(
                    TextBundle::from_section(
                        ach_def.description(),
                        TextStyle {
                            font: asset_server.load(app::ui::FONT),
                            font_size: ACH_DESC_FS,
                            color: app::ui::SECONDARY_COLOR,
                        },
                    )
                    .with_style(Style {
                        margin: UiRect::right(app::ui::px_p(0.0)),
                        ..default()
                    }),
                );
                let name = if record.is_done {
                    ach_def.name()
                } else {
                    "???"
                };
                parent.spawn(
                    TextBundle::from_section(
                        name,
                        TextStyle {
                            font: asset_server.load(app::ui::FONT),
                            font_size: ACH_NAME_FS,
                            color,
                        },
                    )
                    .with_style(Style {
                        margin: UiRect::right(app::ui::px_p(0.0)),
                        ..default()
                    }),
                );
            });
        if !record.is_done {
            let icon = if store.is_pinned(ach_def.code()) {
                asset_server.load("images/icons/ach-push-pin.png")
            } else {
                asset_server.load("images/icons/ach-crosshair.png")
            };
            parent.spawn((
                ImageBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        top: Val::Px(0.0),
                        right: Val::Px(0.0),
                        width: Val::Px(ACH_STATUS_SIZE),
                        height: Val::Px(ACH_STATUS_SIZE),
                        margin: UiRect::all(app::ui::px_p(2.0)),
                        ..default()
                    },
                    image: UiImage::new(icon),
                    ..default()
                },
                AchPanelIcon(String::from(ach_def.code())),
            ));
        }
    });
    if !record.is_done {
        entity.insert((
            ButtonAction::TogglePin(String::from(ach_def.code())),
            app::interaction::IaPanel,
            Focusable::default(),
        ));
    }
}
