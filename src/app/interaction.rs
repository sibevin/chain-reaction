use crate::{app::anime_effect::*, app::theme::*, app::ui};
use bevy::prelude::*;
use bevy_ui_navigation::{
    prelude::{FocusState, Focusable},
    NavRequestSystem,
};

mod default_focus;
mod handle;
mod plugin;

pub use default_focus::handle_default_focus;
pub use default_focus::reset_default_focus;
pub use default_focus::IaDefaultFocus;
pub use handle::IaButton;
pub use handle::IaCrossPanel;
pub use handle::IaLink;
pub use handle::IaMenuEntry;
pub use handle::IaSlider;
pub use handle::IaSwitch;
pub use plugin::InteractionPlugin;
