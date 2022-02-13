use std::marker::PhantomData;

use bevy::prelude::*;
pub mod sprites;
pub mod spritesheetbuilder;
pub mod state;

fn wait_loading(
    status: Res<sprites::SpriteGraphicsStatus>,
    mut state: ResMut<State<state::GameState>>,
) {
    if status.is_loading_finished() {
        state.replace(state::GameState::Ingame).unwrap();
    }
}

#[derive(Default)]
pub struct BevyIngameTools<SPRITES: Reflect> {
    sprites: PhantomData<SPRITES>,
}
impl<SPRITES: Reflect> Plugin for BevyIngameTools<SPRITES> {
    fn build(&self, app: &mut App) {
        app.add_state(state::GameState::Loading)
            .add_system_set(
                SystemSet::on_enter(state::GameState::Loading)
                    .with_system(sprites::loading_startup::<SPRITES>),
            )
            .add_system_set(
                SystemSet::on_update(state::GameState::Loading)
                    .with_system(sprites::loading_update::<SPRITES>)
                    .with_system(wait_loading),
            );
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
