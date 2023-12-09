use crate::reactor::particle::{ParticleType, *};
use bevy::prelude::*;

pub struct TMM {
    pub texture: Handle<Image>,
    pub mesh: Handle<Mesh>,
    pub material: Handle<ColorMaterial>,
}

#[derive(Resource, Default)]
pub struct ParticleTMM {
    alpha: Option<TMM>,
    control: Option<TMM>,
    hyper: Option<TMM>,
    trigger: Option<TMM>,
    uou: Option<TMM>,
}

impl ParticleTMM {
    pub fn get(&self, particle_type: ParticleType) -> &Option<TMM> {
        match particle_type {
            ParticleType::Alpha => &self.alpha,
            ParticleType::Control => &self.control,
            ParticleType::Hyper => &self.hyper,
            ParticleType::Trigger => &self.trigger,
            ParticleType::Uou => &self.uou,
        }
    }
    fn set(&mut self, particle_type: ParticleType, tmm: TMM) {
        match particle_type {
            ParticleType::Alpha => self.alpha = Some(tmm),
            ParticleType::Control => self.control = Some(tmm),
            ParticleType::Hyper => self.hyper = Some(tmm),
            ParticleType::Trigger => self.trigger = Some(tmm),
            ParticleType::Uou => self.uou = Some(tmm),
        }
    }
}

pub struct ParticleTmmPlugin;

impl Plugin for ParticleTmmPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ParticleTMM::default());
    }
}

pub fn init_particle_tmm(
    particle_tmm: &mut ResMut<ParticleTMM>,
    asset_server: &Res<AssetServer>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    particle_tmm.set(
        ParticleType::Alpha,
        alpha::build_particle_tmm(asset_server, meshes, materials),
    );
    particle_tmm.set(
        ParticleType::Control,
        control::build_particle_tmm(asset_server, meshes, materials),
    );
    particle_tmm.set(
        ParticleType::Hyper,
        hyper::build_particle_tmm(asset_server, meshes, materials),
    );
    particle_tmm.set(
        ParticleType::Trigger,
        trigger::build_particle_tmm(asset_server, meshes, materials),
    );
    particle_tmm.set(
        ParticleType::Uou,
        uou::build_particle_tmm(asset_server, meshes, materials),
    );
}
