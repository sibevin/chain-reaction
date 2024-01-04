use crate::app;
use bevy::prelude::*;

pub mod field;
pub mod hit;
pub mod particle;
pub mod state;
pub mod status;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum ReactorState {
    #[default]
    Demo,
    Ready,
    Running,
    Paused,
    Submit,
    Ended,
}

pub struct ReactorPlugin;

impl Plugin for ReactorPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<ReactorState>()
            .insert_resource(status::ReactorStatus::default())
            .add_plugins((
                state::demo::StatePlugin,
                state::ready::StatePlugin,
                state::running::StatePlugin,
                state::paused::StatePlugin,
                state::submit::StatePlugin,
                state::ended::StatePlugin,
            ));
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct ReactorTimer(pub Timer);

#[derive(Component, Deref, DerefMut)]
pub struct PainterTimer(pub Timer);

#[derive(Component, Deref, DerefMut)]
pub struct ScoreTimer(pub Timer);

#[derive(Component)]
pub struct ControlParticle;

#[derive(Component)]
pub struct RunningParticle;

pub const U_SIZE: f32 = app::ui::SPACE_SIZE * 3.0;
pub const U_COLOR: Color = Color::rgb(1.0, 0.84, 0.2);

pub const FIELD_NAV_H: f32 = 80.0;
pub const FIELD_W: f32 = app::WINDOW_W;
pub const FIELD_H: f32 = app::WINDOW_H - FIELD_NAV_H;

pub fn startup(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn(ReactorTimer(Timer::from_seconds(
        0.01,
        TimerMode::Repeating,
    )));
    commands.spawn(PainterTimer(Timer::from_seconds(
        0.05,
        TimerMode::Repeating,
    )));
    commands.spawn(ScoreTimer(Timer::from_seconds(1.0, TimerMode::Repeating)));
    field::build_reactor_field(commands, asset_server);
}
