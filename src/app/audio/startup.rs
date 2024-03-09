use super::*;

pub fn startup(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    settings: &Res<Persistent<settings::Settings>>,
) {
    load_audios(asset_server);
    bgm::build_bgm(commands, asset_server, settings);
}

fn load_audios(asset_server: &Res<AssetServer>) {
    let _ = asset_server.load_folder("audio/bgm");
    let _ = asset_server.load_folder("audio/se");
}
