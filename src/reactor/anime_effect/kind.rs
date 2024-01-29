use crate::reactor::anime_effect::*;

pub mod bullet;
pub mod explosion;

pub trait AnimeEffectKindBase {
    fn create(&self, commands: &mut Commands, param: AnimeEffectParam);
    fn draw(&self, commands: &mut Commands, ae: &AnimeEffect);
}

pub fn fetch_builder(kind: AnimeEffectKind) -> &'static dyn AnimeEffectKindBase {
    match kind {
        AnimeEffectKind::Explosion => &kind::explosion::AnimeEffectKindExplosion,
        AnimeEffectKind::Bullet => &kind::bullet::AnimeEffectKindBullet,
    }
}
