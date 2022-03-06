//! This is an example which shows how to load sprites and
//! animations using the BevySpriteHelper plugin.
 
use bevy::prelude::*;
use bevy_sprite_helper::{sprites, spritesheetbuilder::SpriteSheetBundleBuilder, state::GameState, animation::SpriteAnimationStore};

/// Resource which contains the sprite indices for the TextureAtlas.
/// It will search under sprites/(attribute_name).png in the assets
/// directory.
#[derive(Reflect, Default)]
pub struct Sprites {
    tree: usize,
    player_south: usize,
    player_south_1: usize,
    player_south_2: usize,
}

pub fn setup(
    mut commands: Commands,
    // Get the generated TextureAtlas.
    texture_atlas_holder: Res<sprites::TextureAtlasHolder>,
    // Get the texture indices
    sprites: Res<Sprites>,
    // Get the automatically detected animations
    sprite_animation_store: Res<SpriteAnimationStore>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Use SpriteSheetBundleBuilder to generate a SpriteSheetBundle using the builder pattern.
    let builder =
        SpriteSheetBundleBuilder::new().texture_atlas(texture_atlas_holder.texture_atlas.clone());

    // Lets create a static tree
    commands.spawn_bundle(
        builder
            .clone()
            .index(sprites.tree)
            .transform(Transform::from_xyz(0.0, 0.0, 100.0))
            .build(),
    );

    // And now lets play a walk animation
    commands.spawn_bundle(
        builder
            .clone()
            .index(sprites.tree)
            .transform(Transform::from_xyz(40.0, 0.0, 100.0))
            .build(),
    ).insert(sprite_animation_store.get("player_south").unwrap());
}

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        
        // Register the BevyIngameTools plugin.  It will automatically take care loading the
        // sprites and detect animations.
        .add_plugin(bevy_sprite_helper::BevySpriteHelper::<Sprites>::default())

        // The sprites index resource must be registered.
        .insert_resource(Sprites::default())

        .add_system_set(SystemSet::on_enter(GameState::Ingame).with_system(setup))
        .run();
}
