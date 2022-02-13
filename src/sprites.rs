use bevy::{prelude::*, reflect::{ReflectRef, ReflectMut}, asset::LoadState};

pub struct SpriteGraphicsHandles {
    sprites: Vec<(String, Handle<Image>)>,
}
pub struct SpriteGraphicsStatus {
    done: bool,
}
impl SpriteGraphicsStatus {
    pub fn is_loading_finished(&self) -> bool {
        self.done
    }
}

pub struct TextureAtlasHolder {
    pub texture_atlas: Handle<TextureAtlas>,
}

pub fn loading_startup<T: Reflect>(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    handle_resource: Res<T>,
) {
    let handles = SpriteGraphicsHandles {
        sprites: if let ReflectRef::Struct(reflect_struct) = handle_resource.reflect_ref() {
            (0..).zip(reflect_struct.iter_fields())
                .filter(|(_, reflect)| reflect.type_name() == 0usize.type_name())
                .map(|(i, _)| reflect_struct.name_at(i).expect("name_at should not be out of range").to_string())
                .map(|field_name| (asset_server.load(&format!("sprites/{field_name}.png")), field_name))
                .map(|(handle, field_name)| (field_name, handle))
                .collect()
        } else {
            Vec::new()
        }
    };
    bevy::log::info!("Loading {} sprites", handles.sprites.len());
    commands.insert_resource(handles);
    commands.insert_resource(SpriteGraphicsStatus { done: false });
}

pub fn loading_update<T: Reflect>(
    mut commands: Commands,
    mut handles: ResMut<SpriteGraphicsHandles>,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Image>>,
    mut handle_resource: ResMut<T>,
    mut texture_atlasses: ResMut<Assets<TextureAtlas>>,
    mut status: ResMut<SpriteGraphicsStatus>,
) {
    if LoadState::Loaded == asset_server.get_group_load_state(handles.sprites.iter().map(|(_, handle)| handle.id))
            && !handles.sprites.is_empty() {
        bevy::log::info!("{} assets are loaded", handles.sprites.len());

        bevy::log::info!("Create texture atlas");
        let mut texture_atlas_builder = TextureAtlasBuilder::default();                
        for handle in handles.sprites.iter().map(|(_, handle)| handle) {
            let texture = textures.get(handle).expect("Texture asset not found");
            texture_atlas_builder.add_texture(handle.clone_weak(), texture);
        }
        let texture_atlas = texture_atlas_builder
            .finish(&mut textures)
            .expect("Could not create texture atlas");

        if let ReflectMut::Struct(reflect_struct) = handle_resource.reflect_mut() {
            for i in 0..reflect_struct.field_len() {
                let field_name = reflect_struct.name_at(i).expect("name_at should not be out of range").to_string();
                let field = reflect_struct.field_at_mut(i).expect("field_at_mut should not be out of range");
                let sprite_filename = format!("sprites/{field_name}.png");
                let texture_index = texture_atlas
                    .get_texture_index(&asset_server.get_handle(sprite_filename.clone())).expect(&format!("sprite not loaded: {sprite_filename}"));
                field.apply(&texture_index);
            }
        }
        let texture_atlas = texture_atlasses.add(texture_atlas);
        commands.insert_resource(TextureAtlasHolder { texture_atlas });
        handles.sprites.clear();
        status.done = true;
    }
}
