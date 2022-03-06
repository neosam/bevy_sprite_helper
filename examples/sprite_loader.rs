use bevy::prelude::*;
use bevy_sprite_helper::sprites;

/// Resource which contains the sprite indices for the TextureAtlas.
/// It will search under sprites/(attribute_name).png in the assets
/// directory.
#[derive(Reflect, Default)]
pub struct Sprites {
    tree: usize,
    player_south: usize,
}

/// Custom game state.
/// Two game states are required to start the loading process and one
/// to wait for them to be loaded.
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum GameState {
    Loading,
    Ingame,
}

pub fn setup(
    mut commands: Commands,
    // Get the generated TextureAtlas.
    texture_atlas_holder: Res<sprites::TextureAtlasHolder>,
    // Get the texture indices
    sprites: Res<Sprites>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // A static tree
    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_holder.texture_atlas.clone(),
        sprite: TextureAtlasSprite {
            index: sprites.tree,
            ..Default::default()
        },
        ..Default::default()
    });

    // A static figure
    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_holder.texture_atlas.clone(),
        sprite: TextureAtlasSprite {
            index: sprites.player_south,
            ..Default::default()
        },
        transform: Transform::from_xyz(50.0, 0.0, 100.0),
        ..Default::default()
    });
}

/// System which waits until the sprites are loaded.
pub fn wait_loading(
    status: Res<sprites::SpriteGraphicsStatus>,
    mut state: ResMut<State<GameState>>,
) {
    if status.is_loading_finished() {
        state.replace(GameState::Ingame).unwrap();
    }
}

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state(GameState::Loading)
        .insert_resource(Sprites::default())
        // On loading statet, register the loading_startup system which
        // will start loading the sprites.
        .add_system_set(
            SystemSet::on_enter(GameState::Loading)
                .with_system(sprites::loading_startup::<Sprites>),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Loading)
                // Wait until the sprites are loaded.  As soon they are loaded,
                // it will create a TextureAtlas and the animations.
                .with_system(sprites::loading_update::<Sprites>)
                .with_system(wait_loading),
        )
        .add_system_set(SystemSet::on_enter(GameState::Ingame).with_system(setup))
        .run();
}
