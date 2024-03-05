use bevy::prelude::*;
use rand::seq::SliceRandom;

pub const VASE_SHAPES: [&str; 7] = [
    "cylinder", "moon", "plane", "oval", "clay_pot", "beaker", "rect_pot",
];
pub const VASE_TEXTURES: [&str; 7] = ["sand", "ink", "paint", "paper", "rock", "wave", "wind"];
pub const VASE_COLORS: [Color; 13] = [
    Color::rgb(0.96, 0.96, 0.96),
    Color::rgb(0.529, 0.808, 0.98),
    Color::rgb(0.98, 0.922, 0.843),
    Color::rgb(0.941, 1.0, 0.941),
    Color::rgb(1.0, 0.894, 0.882),
    Color::rgb(1.0, 1.0, 0.941),
    Color::rgb(0.08, 0.08, 0.08),
    Color::rgb(0.275, 0.51, 0.706),
    Color::rgb(0.804, 0.522, 0.247),
    Color::rgb(0.502, 0.0, 0.0),
    Color::rgb(0.333, 0.42, 0.184),
    Color::rgb(1.0, 0.843, 0.0),
    Color::rgb(0.753, 0.753, 0.753),
];

pub const STAGE_BGS: [&str; 7] = [
    "moon",
    "dock",
    "bar",
    "bonfire",
    "fuji",
    "bucket",
    "blue_paint",
];

pub enum StudioPropertyKind {
    VaseShape(String),
    VaseTexture(String),
    StageBg(String),
    VaseColor(Color),
}

#[derive(Default, Debug)]
pub struct StudioProperty {
    vase_shape: String,
    vase_texture: String,
    vase_color: Color,
    stage_bg: String,
}

impl StudioProperty {
    pub fn generate() -> Self {
        StudioProperty {
            vase_shape: String::from(*VASE_SHAPES.choose(&mut rand::thread_rng()).unwrap()),
            vase_texture: String::from(*VASE_TEXTURES.choose(&mut rand::thread_rng()).unwrap()),
            vase_color: (*VASE_COLORS.choose(&mut rand::thread_rng()).unwrap()),
            stage_bg: String::from(*STAGE_BGS.choose(&mut rand::thread_rng()).unwrap()),
        }
    }

    pub fn set_property(&mut self, kind: StudioPropertyKind) {
        match kind {
            StudioPropertyKind::VaseShape(name) => {
                self.vase_shape = name;
            }
            StudioPropertyKind::VaseTexture(name) => {
                self.vase_texture = name;
            }
            StudioPropertyKind::StageBg(name) => {
                self.stage_bg = name;
            }
            StudioPropertyKind::VaseColor(color) => self.vase_color = color,
        }
    }

    pub fn asset_path(&self, kind: &str) -> String {
        match kind {
            "vase_shape" => {
                format!("images/studio/vase_shape/{}.png", self.vase_shape)
            }
            "vase_texture" => {
                format!("images/studio/vase_texture/{}.png", self.vase_texture)
            }
            "stage_bg" => {
                format!("images/studio/stage_bg/{}.jpg", self.stage_bg)
            }
            _ => {
                panic!("Invalid property kind.")
            }
        }
    }
}
