use super::*;
use crate::app;
use bevy_ui_navigation::NavRequestSystem;
use webbrowser;

const PAGE_CODE: &str = "about_visual";
const PAGE_NAME: &str = "References";
const PAGE_ICON: &str = "star-light";

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
        PageState::AboutVisual
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
            (
                app::anime_effect::clear_anime_effect,
                app::ui::despawn_ui::<OnPage>,
            ),
        );
    }
}

#[derive(Component)]
struct OnPage;

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
                                flex_grow: 1.0,
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                column_gap: app::ui::px_p(10.0),
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
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|parent| {
                                    build_sep_title(parent, &asset_server, "Icon", "shapes-fill");
                                    app::ui::build_link(
                                        parent,
                                        &asset_server,
                                        ButtonAction::Link(String::from(
                                            "https://phosphoricons.com/",
                                        )),
                                        "Phosphor Icons",
                                        Some("globe-light"),
                                        "main",
                                        true,
                                    );
                                    build_sep_title(parent, &asset_server, "Font", "text-aa-fill");
                                    app::ui::build_link(
                                        parent,
                                        &asset_server,
                                        ButtonAction::Link(String::from(
                                            "https://www.fontsquirrel.com/fonts/syn-nova",
                                        )),
                                        "SYN NOVA",
                                        Some("globe-light"),
                                        "main",
                                        true,
                                    );
                                    app::ui::build_link(
                                        parent,
                                        &asset_server,
                                        ButtonAction::Link(String::from(
                                            "http://yoworks.com/telegrama/index.html",
                                        )),
                                        "Telegrama",
                                        Some("globe-light"),
                                        "digit",
                                        true,
                                    );
                                    app::ui::build_link(
                                        parent,
                                        &asset_server,
                                        ButtonAction::Link(String::from(
                                            "https://www.fontsquirrel.com/fonts/VAG-HandWritten",
                                        )),
                                        "VAG-HandWritten",
                                        Some("globe-light"),
                                        "hw",
                                        true,
                                    );
                                });
                        });
                    build_about_nav_bar(parent, &asset_server, PageState::AboutVisual);
                });
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
            ButtonAction::Link(url) => {
                let _ = webbrowser::open(url);
            }
            ButtonAction::MoveToPage(state) => page_state.set(*state),
        },
    );
}
