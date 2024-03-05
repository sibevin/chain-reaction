use crate::{app::anime_effect, app::theme::*, app::ui, book::page::*};
use bevy_persistent::prelude::*;
use bevy_ui_navigation::{prelude::*, NavRequestSystem};

const PAGE_CODE: &str = "history";
const PAGE_NAME: &str = "Report";
const PAGE_ICON: &str = "list-numbers";

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
        PageState::History
    }
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(self.state()),
            (app::interaction::reset_default_focus, page_enter),
        )
        .add_systems(
            Update,
            (handle_ui_navigation, app::interaction::handle_default_focus)
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
pub struct HelpPanel(u8);

#[derive(Component)]
pub struct HelpDot(u8);

#[derive(Component)]
enum ButtonAction {
    BackToMainMenu,
    PrevHelp,
    NextHelp,
    Start,
}

fn page_enter(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut settings: ResMut<Persistent<app::settings::Settings>>,
) {
    if settings.is_enabled("first") {
        settings
            .update(|settings| {
                settings.toggle("first");
            })
            .expect("failed to update first run in help");
    }
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
                            ui::build_icon_btn(
                                parent,
                                &asset_server,
                                (
                                    ButtonAction::PrevHelp,
                                    app::interaction::IaButton,
                                    Focusable::default(),
                                ),
                                Style::default(),
                                "caret-double-left-light",
                            );
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        flex_direction: FlexDirection::Column,
                                        align_items: AlignItems::Start,
                                        justify_content: JustifyContent::Center,
                                        margin: UiRect::all(ui::px_p(4.0)),
                                        row_gap: ui::px_p(4.0),
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|parent| {
                                    build_help_panel(parent, &asset_server);
                                    build_help_dots(parent);
                                });
                            ui::build_icon_btn(
                                parent,
                                &asset_server,
                                (
                                    ButtonAction::NextHelp,
                                    app::interaction::IaButton,
                                    Focusable::default(),
                                    app::interaction::IaDefaultFocus,
                                ),
                                Style::default(),
                                "caret-double-right-light",
                            );
                        });
                });
            ui::build_icon_btn(
                parent,
                &asset_server,
                (
                    ButtonAction::BackToMainMenu,
                    app::interaction::IaButton,
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
            ui::build_btn(
                parent,
                &asset_server,
                (
                    ButtonAction::Start,
                    app::interaction::IaButton,
                    Focusable::default(),
                ),
                Style {
                    position_type: PositionType::Absolute,
                    bottom: ui::px_p(ui::PAGE_PADDING),
                    right: ui::px_p(ui::PAGE_PADDING),
                    padding: UiRect::all(ui::px_p(ui::BTN_PADDING)),
                    ..default()
                },
                Some("Start"),
                Some("play-light"),
            );
        });
}

fn handle_ui_navigation(
    mut actions: Query<&mut ButtonAction>,
    mut events: EventReader<NavEvent>,
    mut page_state: ResMut<NextState<PageState>>,
    mut help_panel_query: Query<(&mut HelpPanel, &mut UiImage), With<HelpPanel>>,
    mut help_dot_query: Query<(&HelpDot, &mut BackgroundColor), With<HelpDot>>,
    asset_server: Res<AssetServer>,
) {
    events.nav_iter().activated_in_query_foreach_mut(
        &mut actions,
        |mut action| match &mut *action {
            ButtonAction::BackToMainMenu => page_state.set(PageState::Menu),
            ButtonAction::PrevHelp => {
                let (mut help_panel, mut image) = help_panel_query.single_mut();
                let prev_help = (help_panel.0 + HELP_COUNT - 1) % HELP_COUNT;
                help_panel.0 = prev_help;
                let icon_path = format!("images/help/{:0>2}.png", prev_help);
                let icon = asset_server.load(icon_path);
                image.texture = icon;
                for (help_dot, mut bg_color) in help_dot_query.iter_mut() {
                    if help_dot.0 == prev_help {
                        *bg_color = MUTE_COLOR.into();
                    } else {
                        *bg_color = BG_COLOR.into();
                    }
                }
            }
            ButtonAction::NextHelp => {
                let (mut help_panel, mut image) = help_panel_query.single_mut();
                let next_help = (help_panel.0 + 1) % HELP_COUNT;
                help_panel.0 = next_help;
                let icon_path = format!("images/help/{:0>2}.png", next_help);
                let icon = asset_server.load(icon_path);
                image.texture = icon;
                for (help_dot, mut bg_color) in help_dot_query.iter_mut() {
                    if help_dot.0 == next_help {
                        *bg_color = MUTE_COLOR.into();
                    } else {
                        *bg_color = BG_COLOR.into();
                    }
                }
            }
            ButtonAction::Start => page_state.set(PageState::Game),
        },
    );
}

const HELP_PANEL_SIZE: f32 = 480.0;
const HELP_COUNT: u8 = 14;

fn build_help_panel(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent
        .spawn((NodeBundle {
            style: Style {
                width: Val::Px(HELP_PANEL_SIZE),
                height: Val::Px(HELP_PANEL_SIZE),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                border: UiRect::all(ui::px_p(0.5)),
                ..default()
            },
            background_color: BG_COLOR.into(),
            border_color: MUTE_COLOR.into(),
            ..default()
        },))
        .with_children(|parent| {
            let icon = asset_server.load("images/help/00.png");
            parent.spawn((
                ImageBundle {
                    image: UiImage::new(icon),
                    ..default()
                },
                HelpPanel(0),
            ));
        });
}

const HELP_DOT_SIZE: f32 = 5.0;

fn build_help_dots(parent: &mut ChildBuilder) {
    parent
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                column_gap: ui::px_p(3.0),
                ..default()
            },
            ..default()
        },))
        .with_children(|parent| {
            for i in 0..HELP_COUNT {
                let mut bg_color = BG_COLOR.into();
                if i == 0 {
                    bg_color = MUTE_COLOR.into();
                }
                parent.spawn((
                    NodeBundle {
                        style: Style {
                            width: ui::px_p(HELP_DOT_SIZE),
                            height: ui::px_p(HELP_DOT_SIZE),
                            border: UiRect::all(ui::px_p(0.5)),
                            ..default()
                        },
                        background_color: bg_color,
                        border_color: MUTE_COLOR.into(),
                        ..default()
                    },
                    HelpDot(i),
                ));
            }
        });
}
