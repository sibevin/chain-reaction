use crate::app::{anime_effect, element, ui};
use crate::book::page::*;
#[cfg(not(target_arch = "wasm32"))]
use bevy::app::AppExit;
use bevy_persistent::prelude::*;
use bevy_ui_navigation::{prelude::*, NavRequestSystem};

const PAGE_CODE: &str = "menu";
const PAGE_NAME: &str = "Menu";
const PAGE_ICON: &str = "arrow-left-light";

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
        PageState::Menu
    }
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(self.state()),
            (app::interaction::reset_default_focus, page_enter),
        )
        .add_systems(
            Update,
            (
                handle_window_resize,
                (
                    handle_menu_navigation,
                    app::interaction::handle_default_focus,
                    element::element_systems(),
                )
                    .after(NavRequestSystem),
            )
                .run_if(in_state(self.state())),
        )
        .add_systems(
            OnExit(self.state()),
            (
                anime_effect::clear_anime_effect,
                element::clear_elements,
                ui::despawn_ui::<OnPage>,
            ),
        );
    }
}

#[derive(Component)]
struct OnPage;

#[derive(Component)]
enum ButtonAction {
    MoveToPage(PageState),
    #[cfg(not(target_arch = "wasm32"))]
    Quit,
}

const MENU_PAGES: [&dyn PageBase; 5] = [
    &history::Page,
    &achievement::Page,
    &help::Page,
    &settings::audio::Page,
    &about::main::Page,
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
                        margin: UiRect::bottom(ui::px_p(30.0)),
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
                                margin: UiRect::bottom(ui::px_p(12.0)),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            let icon = asset_server.load("images/app/title.png");
                            parent.spawn(ImageBundle {
                                style: Style {
                                    // width: Val::Px(80.0),
                                    // height: Val::Px(80.0),
                                    margin: UiRect::right(ui::px_p(6.0)),
                                    ..default()
                                },
                                image: UiImage::new(icon),
                                ..default()
                            });
                        });
                    let game_page = &game::Page;
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    height: Val::Auto,
                                    justify_content: JustifyContent::SpaceBetween,
                                    align_items: AlignItems::Center,
                                    padding: UiRect::new(
                                        ui::px_p(8.0),
                                        ui::px_p(8.0),
                                        ui::px_p(3.0),
                                        ui::px_p(3.0),
                                    ),
                                    margin: UiRect::bottom(ui::px_p(12.0)),
                                    ..default()
                                },
                                background_color: BTN_BG.into(),
                                ..default()
                            },
                            ButtonAction::MoveToPage(game_page.state()),
                            app::interaction::IaButton,
                            Focusable::default(),
                            app::interaction::IaDefaultFocus,
                        ))
                        .with_children(|parent| {
                            let icon = asset_server.load("images/icons/play-light.png");
                            parent.spawn(ImageBundle {
                                style: Style {
                                    margin: UiRect::right(ui::px_p(6.0)),
                                    ..default()
                                },
                                image: UiImage::new(icon),
                                ..default()
                            });
                            parent.spawn(TextBundle::from_section(
                                "Start",
                                TextStyle {
                                    font: asset_server.load(FONT),
                                    font_size: ui::FONT_SIZE * 1.8,
                                    color: FG_COLOR,
                                },
                            ));
                        });
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                display: Display::Grid,
                                grid_template_columns: vec![GridTrack::fr(1.0), GridTrack::fr(1.0)],
                                column_gap: ui::px_p(ui::MENU_ENTRY_PADDING),
                                row_gap: ui::px_p(ui::MENU_ENTRY_PADDING),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            for page_def in MENU_PAGES {
                                element::build_element(
                                    parent,
                                    &asset_server,
                                    (
                                        ButtonAction::MoveToPage(page_def.state()),
                                        app::interaction::IaMenuEntry,
                                        Focusable::default(),
                                    ),
                                    element::ElementInitParams::MenuEntry {
                                        icon: String::from(page_def.icon()),
                                        text: String::from(page_def.name()),
                                    },
                                );
                            }
                            #[cfg(not(target_arch = "wasm32"))]
                            element::build_element(
                                parent,
                                &asset_server,
                                (
                                    ButtonAction::Quit,
                                    app::interaction::IaMenuEntry,
                                    Focusable::default(),
                                ),
                                element::ElementInitParams::MenuEntry {
                                    icon: String::from("sign-out-light"),
                                    text: String::from("Quit"),
                                },
                            );
                        });
                });
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
            ButtonAction::MoveToPage(state) => {
                match state {
                    PageState::Game => {
                        if settings.is_enabled("first") {
                            settings.toggle("first");
                            page_state.set(PageState::Help)
                        } else {
                            page_state.set(PageState::Game)
                        }
                    }
                    _ => page_state.set(*state),
                };
            }
            #[cfg(not(target_arch = "wasm32"))]
            ButtonAction::Quit => app_exit_events.send(AppExit),
        },
    );
}

fn handle_window_resize(window: Query<&Window>) {
    let window = window.single();
    let _width = window.resolution.width();
    let _height = window.resolution.height();
    // dbg!(width, height);
}
