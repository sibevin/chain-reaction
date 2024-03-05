use crate::{app::*, book::page::*};
use bevy_persistent::prelude::*;
use bevy_ui_navigation::{prelude::*, NavRequestSystem};

const PAGE_CODE: &str = "achievement";
const PAGE_NAME: &str = "Marks";
const PAGE_ICON: &str = "crosshair";

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
        PageState::Achievement
    }
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(self.state()),
            (interaction::reset_default_focus, page_enter),
        )
        .add_systems(
            Update,
            (handle_ui_navigation, interaction::handle_default_focus)
                .after(NavRequestSystem)
                .run_if(in_state(self.state())),
        )
        .add_systems(
            OnExit(self.state()),
            (anime_effect::clear_anime_effect, ui::despawn_ui::<OnPage>),
        );
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
                    build_page_title(parent, &asset_server, PAGE_NAME, PAGE_ICON);
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
                                        column_gap: ui::px_p(2.0),
                                        row_gap: ui::px_p(2.0),
                                        ..default()
                                    },
                                    background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
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
            ui::build_icon_btn(
                parent,
                &asset_server,
                (
                    ButtonAction::BackToMainMenu,
                    interaction::IaButton,
                    Focusable::default(),
                ),
                Style {
                    position_type: PositionType::Absolute,
                    bottom: ui::px_p(ui::PAGE_PADDING),
                    left: ui::px_p(ui::PAGE_PADDING),
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

const ACH_ICON_SIZE: f32 = 80.0;
const ACH_STATUS_SIZE: f32 = ui::FONT_SIZE * 1.0;
const ACH_PANEL_W: f32 = ui::FONT_SIZE * 11.0;
const ACH_DESC_FS: f32 = ui::FONT_SIZE * 0.7;
const ACH_NAME_FS: f32 = ui::FONT_SIZE * 1.2;

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
        theme::MUTE_COLOR
    };
    let border_w = if record.is_done {
        ui::px_p(1.0)
    } else {
        Val::Px(0.0)
    };
    let mut entity = parent.spawn((ButtonBundle {
        style: Style {
            height: Val::Auto,
            width: Val::Px(ACH_PANEL_W),
            align_items: AlignItems::Center,
            padding: UiRect::all(ui::px_p(1.0)),
            border: UiRect::all(border_w),

            ..default()
        },
        background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
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
                margin: UiRect::all(ui::px_p(1.0)),
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
                    align_self: AlignSelf::Start,
                    justify_content: JustifyContent::SpaceBetween,
                    margin: UiRect::vertical(ui::px_p(1.0)),
                    padding: UiRect::right(Val::Px(ACH_STATUS_SIZE)),
                    ..default()
                },
                background_color: theme::BG_COLOR.into(),
                ..default()
            },))
            .with_children(|parent| {
                parent.spawn(
                    TextBundle::from_section(
                        ach_def.description(),
                        TextStyle {
                            font: asset_server.load(theme::FONT),
                            font_size: ACH_DESC_FS,
                            color: theme::SECONDARY_COLOR,
                        },
                    )
                    .with_style(Style {
                        margin: UiRect::right(ui::px_p(0.0)),
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
                            font: asset_server.load(theme::FONT),
                            font_size: ACH_NAME_FS,
                            color,
                        },
                    )
                    .with_style(Style {
                        margin: UiRect::right(ui::px_p(0.0)),
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
                        margin: UiRect::all(ui::px_p(2.0)),
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
            interaction::IaMenuEntry,
            Focusable::default(),
        ));
    }
}
