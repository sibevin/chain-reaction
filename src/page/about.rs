use crate::{app, page::*};
use bevy_ui_navigation::{prelude::*, NavRequestSystem};
use webbrowser;

const PAGE_CODE: &str = "about";
const PAGE_NAME: &str = "References";
const PAGE_ICON: &str = "star-light";

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
        PageState::About
    }
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(self.state()), page_enter)
            .add_systems(
                Update,
                (handle_ui_navigation, handle_hidden_button_click)
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
    Link(String),
    MoveToPage(PageState),
}

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
                                width: Val::Percent(100.0),
                                flex_grow: 1.0,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                padding: UiRect::top(app::ui::px_p(10.0)),
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
                                    app::ui::build_link(
                                        parent,
                                        &asset_server,
                                        (
                                            Button,
                                        Interaction::default(),
                                        ButtonAction::MoveToPage(PageState::Dev)
                                        ),
                                        env!("CARGO_PKG_VERSION"),
                                        None,
                                        "default",
                                        false
                                    );
                                    build_sep_title(
                                        parent,
                                        &asset_server,
                                        "Link",
                                        "link-bold",
                                    );
                                    app::ui::build_link(
                                        parent,
                                        &asset_server,
                                        ButtonAction::Link(String::from("https://sibevin.itch.io/chain-reaction")),
                                        "itch.io",
                                        Some("house-line-light"),
                                        "default",
                                        true
                                    );
                                    app::ui::build_link(
                                        parent,
                                        &asset_server,
                                        ButtonAction::Link(String::from("https://github.com/sibevin/chain-reaction")),
                                        "github.com",
                                        Some("github-logo-light"),
                                        "default",
                                        true
                                    );
                                    build_sep_title(
                                        parent,
                                        &asset_server,
                                        "Design",
                                        "pencil-line-fill",
                                    );
                                    app::ui::build_link(
                                        parent,
                                        &asset_server,
                                        (),
                                        "Kait Wang",
                                        None,
                                        "default",
                                        false
                                    );
                                    build_sep_title(
                                        parent,
                                        &asset_server,
                                        "Programming",
                                        "code-bold",
                                    );
                                    app::ui::build_link(
                                        parent,
                                        &asset_server,
                                        (),
                                        "Kait Wang",
                                        None,
                                        "default",
                                        false
                                    );
                                    build_sep_title(
                                        parent,
                                        &asset_server,
                                        "Art",
                                        "palette-fill",
                                    );
                                    app::ui::build_link(
                                        parent,
                                        &asset_server,
                                        (),
                                        "Miya",
                                        None,
                                        "default",
                                        false
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
                                    build_sep_title(
                                        parent,
                                        &asset_server,
                                        "Icon",
                                        "shapes-fill",
                                    );
                                    app::ui::build_link(
                                        parent,
                                        &asset_server,
                                        ButtonAction::Link(String::from("https://phosphoricons.com/")),
                                        "Phosphor Icons",
                                        Some("globe-light"),
                                        "default",
                                        true
                                    );
                                    build_sep_title(
                                        parent,
                                        &asset_server,
                                        "Font",
                                        "text-aa-fill",
                                    );
                                    app::ui::build_link(
                                        parent,
                                        &asset_server,
                                        ButtonAction::Link(String::from("https://www.fontsquirrel.com/fonts/syn-nova")),
                                        "SYN NOVA",
                                        Some("globe-light"),
                                        "default",
                                        true
                                    );
                                    app::ui::build_link(
                                        parent,
                                        &asset_server,
                                        ButtonAction::Link(String::from("http://yoworks.com/telegrama/index.html")),
                                        "Telegrama",
                                        Some("globe-light"),
                                        "digit",
                                        true
                                    );
                                    app::ui::build_link(
                                        parent,
                                        &asset_server,
                                        ButtonAction::Link(String::from("https://www.fontsquirrel.com/fonts/VAG-HandWritten")),
                                        "VAG-HandWritten",
                                        Some("globe-light"),
                                        "hw",
                                        true
                                    );
                                    build_sep_title(
                                        parent,
                                        &asset_server,
                                        "Audio",
                                        "microphone-fill",
                                    );
                                    app::ui::build_link(
                                        parent,
                                        &asset_server,
                                        ButtonAction::Link(String::from(
                                            "https://pixabay.com/music/suspense-synthetic-deception-loopable-epic-cyberpunk-crime-music-157454/"
                                        )),
                                        "Synthetic Deception - By GloeleFazzeri",
                                        Some("globe-light"),
                                        "default",
                                        true
                                    );
                                    app::ui::build_link(
                                        parent,
                                        &asset_server,
                                        ButtonAction::Link(String::from(
                                            "https://pixabay.com/sound-effects/heavy-cineamtic-hit-166888/",
                                        )),
                                        "Heavy Cineamtic Hit - By LordSonny",
                                        Some("globe-light"),
                                        "default",
                                        true
                                    );
                                    app::ui::build_link(
                                        parent,
                                        &asset_server,
                                        ButtonAction::Link(String::from(
                                            "https://pixabay.com/sound-effects/pick-92276/",
                                        )),
                                        "Pick - From Pixabay",
                                        Some("globe-light"),
                                        "default",
                                        true,
                                    );
                                    app::ui::build_link(
                                        parent,
                                        &asset_server,
                                        ButtonAction::Link(String::from(
                                            "https://pixabay.com/sound-effects/item-pick-up-38258/"
                                        )),
                                        "Item Pick Up - From Pixabay",
                                        Some("globe-light"),
                                        "default",
                                        true,

                                    );
                                    app::ui::build_link(
                                        parent,
                                        &asset_server,
                                        ButtonAction::Link(String::from(
                                            "https://pixabay.com/sound-effects/glass-shatter-3-100155/"
                                        )),
                                        "Glass Shatter 3 - From Pixabay",
                                        Some("globe-light"),
                                        "default",
                                        true
                                    );
                                    app::ui::build_link(
                                        parent,
                                        &asset_server,
                                        ButtonAction::Link(String::from(
                                            "https://pixabay.com/sound-effects/tada-military-3-183975/"
                                        )),
                                        "Tada Military 3 - By floraphonic",
                                        Some("globe-light"),
                                        "default",
                                        true
                                    );
                                });
                        });
                });
                app::ui::build_icon_btn(
                    parent,
                    &asset_server,
                    (ButtonAction::MoveToPage(PageState::Menu), app::interaction::IaButton,Focusable::new().prioritized()),
                    Style {
                        position_type: PositionType::Absolute,
                        bottom: app::ui::px_p(app::ui::PAGE_PADDING),
                        left: app::ui::px_p(app::ui::PAGE_PADDING),
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

type InteractionButtonCondition = (Changed<Interaction>, With<Button>);

fn handle_hidden_button_click(
    mut interaction_query: Query<(&Interaction, &ButtonAction), InteractionButtonCondition>,
    mut page_state: ResMut<NextState<PageState>>,
) {
    for (interaction, action) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            if let ButtonAction::MoveToPage(state) = action {
                page_state.set(*state)
            };
        }
    }
}
