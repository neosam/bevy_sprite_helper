use bevy::prelude::*;
use bevy_ingame_tools::sprites;

#[derive(Reflect, Default)]
pub struct Sprites {
    tree: usize,
    player_south_base: usize,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum GameState {
    Loading,
    Ingame,
}

pub fn setup(
    mut commands: Commands,
    texture_atlas_holder: Res<sprites::TextureAtlasHolder>,
    sprites: Res<Sprites>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_holder.texture_atlas.clone(),
        sprite: TextureAtlasSprite {
            index: sprites.tree,
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_holder.texture_atlas.clone(),
        sprite: TextureAtlasSprite {
            index: sprites.player_south_base,
            ..Default::default()
        },
        transform: Transform::from_xyz(50.0, 0.0, 100.0),
        ..Default::default()
    });
}

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
        .add_system_set(
            SystemSet::on_enter(GameState::Loading)
                .with_system(sprites::loading_startup::<Sprites>),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Loading)
                .with_system(sprites::loading_update::<Sprites>)
                .with_system(wait_loading),
        )
        .add_system_set(SystemSet::on_enter(GameState::Ingame).with_system(setup))
        .run();
}
