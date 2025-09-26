//! [Bevy TrenchBroom](https://github.com/Noxmore/bevy_trenchbroom) is the integration layer between Bevy and [TrenchBroom](https://trenchbroom.github.io/).
//! We use TrenchBroom to edit our levels.

use bevy::{ecs::world::DeferredWorld, image::ImageSampler, prelude::*};
use bevy_trenchbroom::prelude::*;

use crate::asset_processing::default_image_sampler_descriptor;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(TrenchBroomPlugins(
        TrenchBroomConfig::new("foxtrot")
            .texture_extensions(to_string_vec(&["png", "jpg", "jpeg"]))
            .texture_exclusions(to_string_vec(&[
                "*_disp_*",
                "*_arm_*",
                "*_nor_*",
                "*_local",
                "*_normal",
                "*_roughness",
            ]))
            .texture_sampler(texture_sampler())
            .default_solid_spawn_hooks(|| {
                SpawnHooks::new()
                    .convex_collider()
                    .smooth_by_default_angle()
            }),
    ));
}

fn texture_sampler() -> ImageSampler {
    let mut sampler = ImageSampler::linear();
    *sampler.get_or_init_descriptor() = default_image_sampler_descriptor();
    sampler
}

fn to_string_vec(slice: &[&str]) -> Vec<String> {
    slice.iter().map(|s| s.to_string()).collect()
}

pub(crate) trait GetTrenchbroomModelPath: QuakeClass {
    fn model_path() -> String {
        Self::CLASS_INFO.model_path().unwrap().to_string()
    }
    fn scene_path() -> String {
        format!("{file_path}#Scene0", file_path = Self::model_path())
    }
    fn animation_path(index: u32) -> String {
        format!(
            "{file_path}#Animation{index}",
            file_path = Self::model_path()
        )
    }
}

impl<T: QuakeClass> GetTrenchbroomModelPath for T {}

pub(crate) trait LoadTrenchbroomModel {
    fn load_trenchbroom_model<T: QuakeClass>(&self) -> Handle<Scene>;
}

impl LoadTrenchbroomModel for DeferredWorld<'_> {
    fn load_trenchbroom_model<T: QuakeClass>(&self) -> Handle<Scene> {
        self.resource::<AssetServer>().load_trenchbroom_model::<T>()
    }
}

impl LoadTrenchbroomModel for AssetServer {
    fn load_trenchbroom_model<T: QuakeClass>(&self) -> Handle<Scene> {
        self.load(T::scene_path())
    }
}
