use bevy::prelude::*;
use bevy_ingame_tools::{sprites, spritesheetbuilder::SpriteSheetBundleBuilder, state::GameState, animation::SpriteAnimationStore};

#[derive(Reflect, Default)]
pub struct Sprites {
    tree: usize,
    player_south_base: usize,
    player_south_walk_1: usize,
    player_south_walk_2: usize,
}

pub fn setup(
    mut commands: Commands,
    texture_atlas_holder: Res<sprites::TextureAtlasHolder>,
    sprites: Res<Sprites>,
    sprite_animation_store: Res<SpriteAnimationStore>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let builder =
        SpriteSheetBundleBuilder::new().texture_atlas(texture_atlas_holder.texture_atlas.clone());
    commands.spawn_bundle(
        builder
            .clone()
            .index(sprites.tree)
            .transform(Transform::from_xyz(0.0, 0.0, 100.0))
            .build(),
    );
    commands.spawn_bundle(
        builder
            .clone()
            .index(sprites.tree)
            .transform(Transform::from_xyz(40.0, 0.0, 100.0))
            .build(),
    ).insert(sprite_animation_store.get("player_south_walk").unwrap());
}

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_ingame_tools::BevyIngameTools::<Sprites>::default())
        .insert_resource(Sprites::default())
        .add_system_set(SystemSet::on_enter(GameState::Ingame).with_system(setup))
        .run();
}
