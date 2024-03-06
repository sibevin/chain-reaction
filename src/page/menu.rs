use crate::{app, page::*};
#[cfg(not(target_arch = "wasm32"))]
use bevy::app::AppExit;
use bevy_persistent::prelude::*;
use bevy_ui_navigation::{prelude::*, NavRequestSystem};

const PAGE_CODE: &str = "menu";
const PAGE_NAME: &str = "Menu";
const PAGE_ICON: &str = "arrow-left-light";

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
        PageState::Menu
    }
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(self.state()), page_enter)
            .add_systems(
                Update,
                handle_menu_navigation
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
    FirstRun,
    MoveToPage(PageState),
    #[cfg(not(target_arch = "wasm32"))]
    Quit,
}

const MENU_PAGES: [&dyn PageDefBase; 5] = [
    &leaderboard::PageDef,
    &achievement::PageDef,
    &help::PageDef,
    &settings::PageDef,
    &about::PageDef,
];

fn page_enter(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((build_page_layout(), OnPage))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::bottom(app::ui::px_p(30.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                margin: UiRect::bottom(app::ui::px_p(12.0)),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            let icon = asset_server.load("images/app/title.png");
                            parent.spawn(ImageBundle {
                                image: UiImage::new(icon),
                                ..default()
                            });
                        });
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(
                                        app::ui::MENU_ENTRY_W * 2.0 + app::ui::MENU_ENTRY_PADDING,
                                    ),
                                    justify_content: JustifyContent::SpaceBetween,
                                    align_items: AlignItems::Center,
                                    padding: UiRect::all(app::ui::px_p(4.0)),
                                    margin: UiRect::bottom(Val::Px(app::ui::MENU_ENTRY_PADDING)),
                                    ..default()
                                },
                                background_color: app::ui::BG_COLOR.into(),
                                ..default()
                            },
                            ButtonAction::FirstRun,
                            app::interaction::IaButton,
                            Focusable::new().prioritized(),
                        ))
                        .with_children(|parent| {
                            let icon_path = fetch_page_icon_path("game");
                            parent.spawn(ImageBundle {
                                style: Style {
                                    width: Val::Px(app::ui::ICON_SIZE * 1.6),
                                    height: Val::Px(app::ui::ICON_SIZE * 1.6),
                                    margin: UiRect::right(app::ui::px_p(3.0)),
                                    ..default()
                                },
                                image: UiImage::new(asset_server.load(icon_path)),
                                ..default()
                            });
                            parent.spawn(
                                TextBundle::from_section(
                                    game::PageDef.name(),
                                    TextStyle {
                                        font: asset_server.load(app::ui::FONT),
                                        font_size: app::ui::FONT_SIZE * 1.6,
                                        color: app::ui::FG_COLOR,
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::right(app::ui::px_p(2.0)),
                                    ..default()
                                }),
                            );
                        });
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                display: Display::Grid,
                                grid_template_columns: vec![GridTrack::fr(1.0), GridTrack::fr(1.0)],
                                column_gap: Val::Px(app::ui::MENU_ENTRY_PADDING),
                                row_gap: Val::Px(app::ui::MENU_ENTRY_PADDING),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            for page_def in MENU_PAGES {
                                app::ui::build_menu_entry(
                                    parent,
                                    &asset_server,
                                    (
                                        ButtonAction::MoveToPage(page_def.state()),
                                        app::interaction::IaButton,
                                        Focusable::default(),
                                    ),
                                    page_def.name(),
                                    page_def.icon(),
                                );
                            }
                            #[cfg(not(target_arch = "wasm32"))]
                            app::ui::build_menu_entry(
                                parent,
                                &asset_server,
                                (
                                    ButtonAction::Quit,
                                    app::interaction::IaButton,
                                    Focusable::default(),
                                ),
                                "Quit",
                                "sign-out-light",
                            );
                        });
                });
            app::ui::build_icon_btn(
                parent,
                &asset_server,
                (
                    ButtonAction::MoveToPage(PageState::Auto),
                    app::interaction::IaButton,
                    Focusable::default(),
                ),
                Style {
                    position_type: PositionType::Absolute,
                    bottom: app::ui::px_p(app::ui::PAGE_PADDING),
                    left: app::ui::px_p(app::ui::PAGE_PADDING),
                    ..default()
                },
                "monitor",
            );
        });
}

fn handle_menu_navigation(
    mut actions: Query<&mut ButtonAction>,
    mut events: EventReader<NavEvent>,
    mut page_state: ResMut<NextState<PageState>>,
    mut settings: ResMut<Persistent<app::settings::Settings>>,
    #[cfg(not(target_arch = "wasm32"))] mut app_exit_events: EventWriter<AppExit>,
) {
    events.nav_iter().activated_in_query_foreach_mut(
        &mut actions,
        |mut action| match &mut *action {
            ButtonAction::FirstRun => {
                if settings.is_enabled("first") {
                    settings.toggle("first");
                    page_state.set(PageState::Help)
                } else {
                    page_state.set(PageState::Game)
                }
            }
            ButtonAction::MoveToPage(state) => page_state.set(*state),
            #[cfg(not(target_arch = "wasm32"))]
            ButtonAction::Quit => app_exit_events.send(AppExit),
        },
    );
}
