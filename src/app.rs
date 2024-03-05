pub mod achievement;
pub mod anime_effect;
pub mod audio;
pub mod cursor;
pub mod cursor_icon;
pub mod interaction;
pub mod key_binding;
pub mod layer;
pub mod leaderboard;
pub mod plugin;
pub mod screenshot;
pub mod settings;
pub mod startup;
pub mod status;
pub mod theme;
pub mod timer;
pub mod ui;

pub use plugin::AppPlugin;
pub use startup::startup;

pub const WINDOW_W: f32 = 1280.0;
pub const WINDOW_H: f32 = 720.0;

pub const APP_CODE: &str = "chain_reaction";
pub const APP_NAME: &str = "Chain Reaction";
pub const APP_SLOGAN: &str = "An action game to dodge chain reaction particles, at the same time you have to create your own chain.";
pub const APP_ITCH_URL: &str = "https://sibevin.itch.io/chain-reaction";
pub const APP_GITHUB_URL: &str = "https://github.com/sibevin/chain-reaction";
